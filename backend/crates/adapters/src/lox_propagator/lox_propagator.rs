use async_trait::async_trait;
use chrono::{DateTime, NaiveDateTime, TimeZone};
use domain::{
    errors::ComputeError,
    models::{
        CartesianState, Constellation, EphemerisSource, KeplerianState, ReferenceFrame,
        SatelliteEphemeris,
    },
};
use lox_space::{
    bodies::PointMass,
    orbits::propagators::j4::J4Propagator,
    prelude::*,
    time::{
        calendar_dates::CalendarDate,
        time_of_day::CivilTime,
        time_scales::DynTimeScale::Tdb,
        utc::{leap_seconds::DefaultLeapSecondsProvider, transformations::ToUtc},
    },
};
use ports::inbound::OrbitPropagator;

pub struct LoxOrbitPropagator {}

#[async_trait]
impl OrbitPropagator for LoxOrbitPropagator {
    async fn propagate(
        &self,
        constellation: Constellation,
        duration: std::time::Duration,
        step: std::time::Duration,
    ) -> Result<Vec<SatelliteEphemeris>, ComputeError> {
        let mut ephemerides: Vec<SatelliteEphemeris> = Vec::new();

        for satellite in constellation.satellites {
            let formatted_epoch = &satellite
                .data
                .initial_state
                .epoch
                .format("%Y-%m-%dT%H:%M:%S")
                .to_string();

            let epoch = Utc::from_iso(formatted_epoch)
                .map_err(|err| ComputeError::Other(err.to_string()))?
                .to_time();

            let cartesian = Cartesian::from_array([
                1000.0 * satellite.data.initial_state.pos_x,
                1000.0 * satellite.data.initial_state.pos_y,
                1000.0 * satellite.data.initial_state.pos_z,
                1000.0 * satellite.data.initial_state.vel_x,
                1000.0 * satellite.data.initial_state.vel_y,
                1000.0 * satellite.data.initial_state.vel_z,
            ]);

            let orbit = CartesianOrbit::from_state(cartesian, epoch, Earth, Icrf);
            let propagator =
                J4Propagator::try_new(orbit).map_err(|err| ComputeError::Other(err.to_string()))?;

            let end = epoch + TimeDelta::from_seconds(duration.as_secs() as i64);
            let trajectory = propagator
                .propagate(Interval::new(epoch, end))
                .map_err(|err| ComputeError::Other(err.to_string()))?;

            let mut cartesian_ephemeris: Vec<CartesianState> = Vec::new();
            let mut keplerian_ephemeris: Vec<KeplerianState> = Vec::new();

            for delta in 0..duration.as_secs() {
                let time_delta = TimeDelta::from_seconds(delta as i64);
                let interp = trajectory.interpolate(time_delta);

                let cartesian = interp.state();
                let keplerian = cartesian.to_keplerian(Earth.gravitational_parameter());

                let lox_utc = (epoch + time_delta).to_utc().to_string();
                let naive = NaiveDateTime::parse_from_str(&lox_utc, "%Y-%m-%dT%H:%M:%S%.f UTC")
                    .map_err(|err| ComputeError::Other(err.to_string()))?;
                let datetime_utc: DateTime<chrono::Utc> = chrono::Utc.from_utc_datetime(&naive);

                let keplerian_state = KeplerianState {
                    datetime: datetime_utc,
                    sma_km: keplerian.semi_major_axis().to_kilometers(),
                    eccentricity: keplerian.eccentricity().as_f64(),
                    inclination_deg: keplerian.inclination().as_f64().to_degrees(),
                    raan_deg: keplerian
                        .longitude_of_ascending_node()
                        .as_f64()
                        .to_degrees(),
                    arg_periapsis_deg: keplerian.argument_of_periapsis().as_f64().to_degrees(),
                    true_anomaly_deg: keplerian.true_anomaly().as_f64().to_degrees(),
                    reference_frame: ReferenceFrame::Itrf,
                    source: EphemerisSource::Predicted,
                };

                keplerian_ephemeris.push(keplerian_state);

                let cartesian_state = CartesianState {
                    datetime: datetime_utc,
                    pos_x: cartesian.x().to_kilometers(),
                    pos_y: cartesian.y().to_kilometers(),
                    pos_z: cartesian.z().to_kilometers(),
                    vel_x: cartesian.vx().to_kilometers_per_second(),
                    vel_y: cartesian.vy().to_kilometers_per_second(),
                    vel_z: cartesian.vz().to_kilometers_per_second(),
                    reference_frame: ReferenceFrame::Itrf,
                    source: EphemerisSource::Predicted,
                };

                cartesian_ephemeris.push(cartesian_state);
            }

            let satellite_ephemeris = SatelliteEphemeris {
                id: satellite.id.clone(),
                cartesian_ephemeris,
                keplerian_ephemeris,
            };

            ephemerides.push(satellite_ephemeris);
        }

        Ok(ephemerides)
    }
}
