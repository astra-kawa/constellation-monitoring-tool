<script lang="ts">
  interface EciState {
    epoch: Date;
    pos_x: number;
    pos_y: number;
    pos_z: number;
    vel_x: number;
    vel_y: number;
    vel_z: number;
  }

  interface Satellite {
    id: string;
    initial_state: EciState;
  }

  let { satelliteProp } = $props();
  let satellite: Satellite = satelliteProp;
</script>

<div class="satellite-card">
  <div class="satellite-title">{satellite.id}</div>
  <div class="satellite-details">
    <div class="satellite-card-fields">
      <div class="card-field">
        <div>Initial State</div>
      </div>
      <div class="card-field">
        <div>Epoch</div>
      </div>
      <div class="card-field">
        <div>Position</div>
        <div>X</div>
      </div>
      <div class="card-field">
        <div></div>
        <div>Y</div>
      </div>
      <div class="card-field">
        <div></div>
        <div>Z</div>
      </div>
      <div class="card-field">
        <div>Velocity</div>
        <div>X</div>
      </div>
      <div class="card-field">
        <div></div>
        <div>Y</div>
      </div>
      <div class="card-field">
        <div></div>
        <div>Z</div>
      </div>
    </div>
    <div class="satellite-card-values">
      <div class="card-value"><span>CartesianECI</span></div>
      <div class="card-value">
        <span>{satellite.initial_state.epoch}</span>
      </div>
      <div class="card-value">
        <span>{satellite.initial_state.pos_x}</span>
      </div>
      <div class="card-value">
        <span>{satellite.initial_state.pos_y}</span>
      </div>
      <div class="card-value">
        <span>{satellite.initial_state.pos_z}</span>
      </div>
      <div class="card-value">
        <span>{satellite.initial_state.vel_x}</span>
      </div>
      <div class="card-value">
        <span>{satellite.initial_state.vel_y}</span>
      </div>
      <div class="card-value">
        <span>{satellite.initial_state.vel_z}</span>
      </div>
    </div>
  </div>
</div>

<style>
  .satellite-card {
    border: 1px solid var(--main-font-colour);
    max-width: 400px;
  }

  .satellite-title {
    text-align: center;
    width: 100%;
    border-bottom: 1px solid var(--main-font-colour);
  }

  .satellite-details {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }

  .card-field {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
  }
</style>
