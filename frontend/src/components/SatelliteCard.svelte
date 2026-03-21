<script lang="ts">
  import type { Satellite } from "../lib/types";

  let { satellite }: { satellite: Satellite } = $props();
</script>

<div class="satellite-card">
  <div class="satellite-title">{satellite.id}</div>
  <div class="satellite-details">
    <table>
      <tbody>
        <tr>
          <td>Status</td>
          <td></td>
          <td>NOMINAL</td>
        </tr>
        <tr>
          <td>Initial State</td>
          <td></td>
          <td>
            <select name="state-types" id="state-{satellite.id}">
              <option value="eci">Earth-Centered Inertial</option>
              <option value="kepler">Keplerian Orbital Elements</option>
            </select>
          </td>
        </tr>
        <tr>
          <td>Epoch</td>
          <td>UTC</td>
          <td
            ><input
              type="text"
              id="epoch-{satellite.id}"
              value={satellite.initial_state.epoch}
            /></td
          >
        </tr>
        <tr>
          <td>Position</td>
          <td>X</td>
          <td
            ><input
              type="number"
              id="pos_x-{satellite.id}"
              value={satellite.initial_state.pos_x}
            /></td
          >
        </tr>
        <tr>
          <td>[km]</td>
          <td>Y</td>
          <td
            ><input
              type="number"
              id="pos_y-{satellite.id}"
              value={satellite.initial_state.pos_y}
            /></td
          >
        </tr>
        <tr>
          <td></td>
          <td>Y</td>
          <td
            ><input
              type="number"
              id="pos_z-{satellite.id}"
              value={satellite.initial_state.pos_z}
            /></td
          >
        </tr>
        <tr>
          <td>Velocity</td>
          <td>X</td>
          <td
            ><input
              type="number"
              id="vel_x-{satellite.id}"
              value={satellite.initial_state.vel_x}
            /></td
          >
        </tr>
        <tr>
          <td>[km/s]</td>
          <td>Y</td>
          <td
            ><input
              type="number"
              id="vel_y-{satellite.id}"
              value={satellite.initial_state.vel_y}
            /></td
          >
        </tr>
        <tr>
          <td></td>
          <td>Y</td>
          <td
            ><input
              type="number"
              id="vel_z-{satellite.id}"
              value={satellite.initial_state.vel_z}
            /></td
          >
        </tr>
      </tbody>
    </table>
  </div>
</div>

<style>
  .satellite-card {
    border: 1px solid var(--main-font-colour);
    font-size: var(--secondary-font-size);
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
    gap: 5px;
  }

  table td:nth-child(3),
  select,
  input {
    text-align: right;
  }

  select,
  input {
    border: none;
    background-color: none;
    background: none;
    font-family: var(--main-font-family);
    font-size: var(--secondary-font-size);
  }

  input {
    border-bottom: 1px dashed var(--main-font-colour);
  }
</style>
