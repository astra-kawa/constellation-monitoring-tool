<script lang="ts">
  import type { Satellite } from "$lib/types";
  import { getFormStringValue, getFormNumberValue } from "$lib/utils/utils";

  let {
    satellite,
    refreshConstellationData,
  }: { satellite: Satellite; refreshConstellationData: Function } = $props();

  let isSaving = $state(false);
  let saveMessage = $state("");
  let saveError = $state("");

  let isDeleting = $state(false);

  function clearFeedback() {
    saveMessage = "";
    saveError = "";
  }

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    clearFeedback();

    const form = event.currentTarget as HTMLFormElement | null;

    if (!form) {
      return;
    }

    const formData = new FormData(form);
    const stateType = getFormStringValue(formData, "state_type");

    if (stateType !== "eci") {
      saveError = "Only Earth-Centered Inertial state updates are supported.";
      return;
    }

    const payload = {
      id: satellite.id,
      data: {
        initial_state: {
          epoch: getFormStringValue(formData, "epoch"),
          pos_x: getFormNumberValue(formData, "pos_x"),
          pos_y: getFormNumberValue(formData, "pos_y"),
          pos_z: getFormNumberValue(formData, "pos_z"),
          vel_x: getFormNumberValue(formData, "vel_x"),
          vel_y: getFormNumberValue(formData, "vel_y"),
          vel_z: getFormNumberValue(formData, "vel_z"),
        },
      },
    };

    isSaving = true;

    try {
      const response = await fetch(
        `http://127.0.0.1:3000/api/constellation/${encodeURIComponent(satellite.id)}`,
        {
          method: "PUT",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify(payload),
        },
      );

      if (!response.ok) {
        const errorResponse = await response.json().catch(() => ({
          error: `Failed to update satellite ${satellite.id}`,
        }));

        throw new Error(
          typeof errorResponse?.error === "string"
            ? errorResponse.error
            : `Failed to update satellite ${satellite.id}`,
        );
      }

      saveMessage = "Saved";
    } catch (error) {
      saveError =
        error instanceof Error
          ? error.message
          : `Failed to update satellite ${satellite.id}`;
    } finally {
      isSaving = false;
    }
  }
</script>

<form class="satellite-card" onsubmit={handleSubmit} oninput={clearFeedback}>
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
            <select name="state_type" id="state-{satellite.id}">
              <option value="eci" selected>Earth-Centered Inertial</option>
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
              name="epoch"
              id="epoch-{satellite.id}"
              value={satellite.data.initial_state.epoch}
              required
            /></td
          >
        </tr>
        <tr>
          <td>Position</td>
          <td>x</td>
          <td
            ><input
              type="number"
              name="pos_x"
              id="pos_x-{satellite.id}"
              value={satellite.data.initial_state.pos_x}
              step="any"
              required
            /></td
          >
        </tr>
        <tr>
          <td>[km]</td>
          <td>y</td>
          <td
            ><input
              type="number"
              name="pos_y"
              id="pos_y-{satellite.id}"
              value={satellite.data.initial_state.pos_y}
              step="any"
              required
            /></td
          >
        </tr>
        <tr>
          <td></td>
          <td>z</td>
          <td
            ><input
              type="number"
              name="pos_z"
              id="pos_z-{satellite.id}"
              value={satellite.data.initial_state.pos_z}
              step="any"
              required
            /></td
          >
        </tr>
        <tr>
          <td>Velocity</td>
          <td>x</td>
          <td
            ><input
              type="number"
              name="vel_x"
              id="vel_x-{satellite.id}"
              value={satellite.data.initial_state.vel_x}
              step="any"
              required
            /></td
          >
        </tr>
        <tr>
          <td>[km/s]</td>
          <td>y</td>
          <td
            ><input
              type="number"
              name="vel_y"
              id="vel_y-{satellite.id}"
              value={satellite.data.initial_state.vel_y}
              step="any"
              required
            /></td
          >
        </tr>
        <tr>
          <td></td>
          <td>z</td>
          <td
            ><input
              type="number"
              name="vel_z"
              id="vel_z-{satellite.id}"
              value={satellite.data.initial_state.vel_z}
              step="any"
              required
            /></td
          >
        </tr>
      </tbody>
    </table>
  </div>
  <div class="satellite-actions">
    <button class="save-button" type="submit" disabled={isSaving}>
      SAVE
    </button>
    <button
      class="delete-button"
      type="button"
      disabled={isDeleting}
      onclick={async () => {
        const constellationResponse = await fetch(
          `http://127.0.0.1:3000/api/constellation/${encodeURIComponent(satellite.id)}`,
          {
            method: "DELETE",
          },
        );

        refreshConstellationData();
      }}
    >
      DELETE
    </button>
    <span class="satellite-status" aria-live="polite">
      {#if saveError}
        {saveError}
      {:else if saveMessage}
        {saveMessage}
      {/if}
    </span>
  </div>
</form>

<style>
  .satellite-card {
    border: 1px solid var(--main-font-colour);
    font-size: var(--secondary-font-size);
    display: flex;
    flex-direction: column;
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

  .satellite-actions {
    display: flex;
    flex-direction: row;
    align-items: center;
    gap: 10px;
    margin-top: 0.5rem;
    padding: 0.5rem;
    border-top: 1px solid var(--main-font-colour);
  }

  table td:nth-child(3),
  select,
  input {
    text-align: right;
  }

  select,
  input,
  button {
    border: none;
    background-color: none;
    background: none;
    font-family: var(--main-font-family);
    font-size: var(--secondary-font-size);
  }

  input {
    border-bottom: 1px dashed var(--main-font-colour);
  }

  button {
    cursor: pointer;
  }

  button:disabled {
    cursor: wait;
    opacity: 0.7;
  }

  .save-button {
    background-color: var(--interactive-colour1);
  }

  .delete-button {
    background-color: var(--interactive-colour2);
  }

  .satellite-status {
    text-align: right;
  }
</style>
