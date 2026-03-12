module ConstellationModel

using LinearAlgebra
using ModelingToolkit
using ModelingToolkit: t_nounits as t, D_nounits as D
using OrdinaryDiffEq

export ConstellationSpec
export build_constellation_system, build_constellation_problem
export relative_state_ric

"""
Configuration for an `N`-satellite two-body constellation model.
`mu` is Earth gravitational parameter in km^3/s^2 by default.
"""
struct ConstellationSpec
    nsat::Int
    mu::Float64
    function ConstellationSpec(nsat::Integer; mu::Real = 398600.0)
        nsat >= 1 || throw(ArgumentError("nsat must be >= 1"))
        new(Int(nsat), Float64(mu))
    end
end

"""
Builds and compiles a ModelingToolkit system for `spec.nsat` satellites in
independent two-body dynamics around a central body.
Returns `(model, symbols)` where `symbols` stores variable vectors in a NamedTuple.
"""
function build_constellation_system(spec::ConstellationSpec)
    @parameters μ = spec.mu

    r_name = Vector{Symbol}(undef, spec.nsat)
    pos_x_name = Vector{Symbol}(undef, spec.nsat)
    pos_y_name = Vector{Symbol}(undef, spec.nsat)
    pos_z_name = Vector{Symbol}(undef, spec.nsat)
    vel_x_name = Vector{Symbol}(undef, spec.nsat)
    vel_y_name = Vector{Symbol}(undef, spec.nsat)
    vel_z_name = Vector{Symbol}(undef, spec.nsat)

    r = Vector{Num}(undef, spec.nsat)
    pos_x = Vector{Num}(undef, spec.nsat)
    pos_y = Vector{Num}(undef, spec.nsat)
    pos_z = Vector{Num}(undef, spec.nsat)
    vel_x = Vector{Num}(undef, spec.nsat)
    vel_y = Vector{Num}(undef, spec.nsat)
    vel_z = Vector{Num}(undef, spec.nsat)

    eqs = Vector{Equation}(undef, 7 * spec.nsat)
    idx = 1

    for i in 1:spec.nsat
        r_name[i] = Symbol(:r, i)
        pos_x_name[i] = Symbol(:rx, i)
        pos_y_name[i] = Symbol(:ry, i)
        pos_z_name[i] = Symbol(:rz, i)
        vel_x_name[i] = Symbol(:vx, i)
        vel_y_name[i] = Symbol(:vy, i)
        vel_z_name[i] = Symbol(:vz, i)

        r[i] = only(@variables $(r_name[i])(t))
        pos_x[i] = only(@variables $(pos_x_name[i])(t))
        pos_y[i] = only(@variables $(pos_y_name[i])(t))
        pos_z[i] = only(@variables $(pos_z_name[i])(t))
        vel_x[i] = only(@variables $(vel_x_name[i])(t))
        vel_y[i] = only(@variables $(vel_y_name[i])(t))
        vel_z[i] = only(@variables $(vel_z_name[i])(t))

        eqs[idx] = r[i] ~ sqrt(pos_x[i]^2 + pos_y[i]^2 + pos_z[i]^2); idx += 1
        eqs[idx] = D(pos_x[i]) ~ vel_x[i]; idx += 1
        eqs[idx] = D(pos_y[i]) ~ vel_y[i]; idx += 1
        eqs[idx] = D(pos_z[i]) ~ vel_z[i]; idx += 1
        eqs[idx] = D(vel_x[i]) ~ -μ * pos_x[i] / r[i]^3; idx += 1
        eqs[idx] = D(vel_y[i]) ~ -μ * pos_y[i] / r[i]^3; idx += 1
        eqs[idx] = D(vel_z[i]) ~ -μ * pos_z[i] / r[i]^3; idx += 1
    end

    @named base_model = System(eqs, t)
    model = mtkcompile(base_model)

    symbols = (
        r = r,
        pos_x = pos_x,
        pos_y = pos_y,
        pos_z = pos_z,
        vel_x = vel_x,
        vel_y = vel_y,
        vel_z = vel_z,
        r_name = r_name,
        pos_x_name = pos_x_name,
        pos_y_name = pos_y_name,
        pos_z_name = pos_z_name,
        vel_x_name = vel_x_name,
        vel_y_name = vel_y_name,
        vel_z_name = vel_z_name,
        μ = μ
    )
    return model, symbols
end

"""
Construct an `ODEProblem` for the compiled constellation model.

`initial_states` must contain one 6-tuple per satellite:
`(rx0, ry0, rz0, vx0, vy0, vz0)`.
"""
function build_constellation_problem(
    model,
    symbols::NamedTuple,
    initial_states::AbstractVector{<:NTuple{6,<:Real}},
    tspan::Tuple{<:Real,<:Real}
)
    nsat = length(symbols.pos_x)
    length(initial_states) == nsat ||
        throw(ArgumentError("initial_states length must match number of satellites ($nsat)"))

    u0 = Pair[]
    for i in 1:nsat
        s = initial_states[i]
        push!(u0, getproperty(model, symbols.pos_x_name[i]) => Float64(s[1]))
        push!(u0, getproperty(model, symbols.pos_y_name[i]) => Float64(s[2]))
        push!(u0, getproperty(model, symbols.pos_z_name[i]) => Float64(s[3]))
        push!(u0, getproperty(model, symbols.vel_x_name[i]) => Float64(s[4]))
        push!(u0, getproperty(model, symbols.vel_y_name[i]) => Float64(s[5]))
        push!(u0, getproperty(model, symbols.vel_z_name[i]) => Float64(s[6]))
    end

    return ODEProblem(model, u0, (Float64(tspan[1]), Float64(tspan[2])))
end

"""
Compute relative target state with respect to a reference satellite in the
radial/in-track/cross-track (RIC) frame of the reference.

Returns a NamedTuple:
- `t`: sample times
- `position_ric`: `N x 3` matrix with columns `[R, I, C]` in km
- `velocity_ric`: `N x 3` matrix with columns `[R, I, C]` in km/s
- `velocity_projected_ric`: inertial relative velocity projected on RIC axes
"""
function relative_state_ric(
    spec::ConstellationSpec,
    model,
    symbols::NamedTuple,
    sol;
    reference_satellite::Integer,
    target_satellite::Integer,
    sample_times = nothing
)
    1 <= reference_satellite <= spec.nsat ||
        throw(ArgumentError("reference_satellite must be in 1:$(spec.nsat)"))
    1 <= target_satellite <= spec.nsat ||
        throw(ArgumentError("target_satellite must be in 1:$(spec.nsat)"))
    length(symbols.rx_name) == spec.nsat ||
        throw(ArgumentError("symbols do not match spec.nsat"))

    times = sample_times === nothing ? collect(sol.t) : Float64.(collect(sample_times))
    sample = sol(times)

    ref_pos_x = sample[getproperty(model, symbols.pos_x_name[reference_satellite])]
    ref_pos_y = sample[getproperty(model, symbols.pos_y_name[reference_satellite])]
    ref_pos_z = sample[getproperty(model, symbols.pos_z_name[reference_satellite])]
    ref_vel_x = sample[getproperty(model, symbols.vel_x_name[reference_satellite])]
    ref_vel_y = sample[getproperty(model, symbols.vel_y_name[reference_satellite])]
    ref_vel_z = sample[getproperty(model, symbols.vel_z_name[reference_satellite])]

    tgt_pos_x = sample[getproperty(model, symbols.pos_x_name[target_satellite])]
    tgt_pos_y = sample[getproperty(model, symbols.pos_y_name[target_satellite])]
    tgt_pos_z = sample[getproperty(model, symbols.pos_z_name[target_satellite])]
    tgt_vel_x = sample[getproperty(model, symbols.vel_x_name[target_satellite])]
    tgt_vel_y = sample[getproperty(model, symbols.vel_y_name[target_satellite])]
    tgt_vel_z = sample[getproperty(model, symbols.vel_z_name[target_satellite])]

    ns = length(times)
    position_ric = Matrix{Float64}(undef, ns, 3)
    velocity_ric = Matrix{Float64}(undef, ns, 3)
    velocity_projected_ric = Matrix{Float64}(undef, ns, 3)

    for k in 1:ns
        r_ref = [ref_pos_x[k], ref_pos_y[k], ref_pos_z[k]]
        v_ref = [ref_vel_x[k], ref_vel_y[k], ref_vel_z[k]]
        r_tgt = [tgt_pos_x[k], tgt_pos_y[k], tgt_pos_z[k]]
        v_tgt = [tgt_vel_x[k], tgt_vel_y[k], tgt_vel_z[k]]

        r_norm = norm(r_ref)
        r_norm > 0.0 || throw(ArgumentError("Reference radius is zero at sample index $k"))

        h_ref = cross(r_ref, v_ref)
        h_norm = norm(h_ref)
        h_norm > 0.0 ||
            throw(ArgumentError("Reference angular momentum is zero at sample index $k"))

        e_r = r_ref / r_norm
        e_c = h_ref / h_norm
        e_i = cross(e_c, e_r)

        c_ric_from_eci = vcat(e_r', e_i', e_c')
        dr_eci = r_tgt - r_ref
        dv_eci = v_tgt - v_ref

        dr_ric = c_ric_from_eci * dr_eci
        dv_proj_ric = c_ric_from_eci * dv_eci

        # LVLH/RIC frame spin rate, expressed in RIC coordinates.
        omega_eci = h_ref / (r_norm^2)
        omega_ric = c_ric_from_eci * omega_eci
        dv_ric = dv_proj_ric - cross(omega_ric, dr_ric)

        position_ric[k, :] .= dr_ric
        velocity_ric[k, :] .= dv_ric
        velocity_projected_ric[k, :] .= dv_proj_ric
    end

    return (
        t = times,
        position_ric = position_ric,
        velocity_ric = velocity_ric,
        velocity_projected_ric = velocity_projected_ric
    )
end

end # module
