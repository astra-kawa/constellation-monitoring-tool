<script lang="ts">
  import type { Satellite } from "../lib/types";

  let { satellite }: { satellite: Satellite } = $props();

  let isSaving = $state(false);
  let saveMessage = $state("");
  let saveError = $state("");

  function clearFeedback() {
    saveMessage = "";
    saveError = "";
  }

  function getStringValue(formData: FormData, fieldName: string): string {
    const value = formData.get(fieldName);

    if (typeof value !== "string" || value.trim() === "") {
      throw new Error(`Missing ${fieldName}`);
    }

    return value.trim();
  }

  function getNumberValue(formData: FormData, fieldName: string): number {
    const value = getStringValue(formData, fieldName);
    const parsedValue = Number(value);

    if (Number.isNaN(parsedValue)) {
      throw new Error(`Invalid ${fieldName}`);
    }

    return parsedValue;
  }

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();
    clearFeedback();

    const form = event.currentTarget as HTMLFormElement | null;

    if (!form) {
      return;
    }

    const formData = new FormData(form);
    const stateType = getStringValue(formData, "state_type");

    if (stateType !== "eci") {
      saveError = "Only Earth-Centered Inertial state updates are supported.";
      return;
    }

    const payload = {
      id: satellite.id,
      initial_state: {
        epoch: getStringValue(formData, "epoch"),
        pos_x: getNumberValue(formData, "pos_x"),
        pos_y: getNumberValue(formData, "pos_y"),
        pos_z: getNumberValue(formData, "pos_z"),
        vel_x: getNumberValue(formData, "vel_x"),
        vel_y: getNumberValue(formData, "vel_y"),
        vel_z: getNumberValue(formData, "vel_z"),
      },
    };

    console.log(payload);

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
        const errorResponse = await response
          .json()
          .catch(() => ({
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
              value={satellite.initial_state.epoch}
              required
            /></td
          >
        </tr>
        <tr>
          <td>Position</td>
          <td>X</td>
          <td
            ><input
              type="number"
              name="pos_x"
              id="pos_x-{satellite.id}"
              value={satellite.initial_state.pos_x}
              step="any"
              required
            /></td
          >
        </tr>
        <tr>
          <td>[km]</td>
          <td>Y</td>
          <td
            ><input
              type="number"
              name="pos_y"
              id="pos_y-{satellite.id}"
              value={satellite.initial_state.pos_y}
              step="any"
              required
            /></td
          >
        </tr>
        <tr>
          <td></td>
          <td>Y</td>
          <td
            ><input
              type="number"
              name="pos_z"
              id="pos_z-{satellite.id}"
              value={satellite.initial_state.pos_z}
              step="any"
              required
            /></td
          >
        </tr>
        <tr>
          <td>Velocity</td>
          <td>X</td>
          <td
            ><input
              type="number"
              name="vel_x"
              id="vel_x-{satellite.id}"
              value={satellite.initial_state.vel_x}
              step="any"
              required
            /></td
          >
        </tr>
        <tr>
          <td>[km/s]</td>
          <td>Y</td>
          <td
            ><input
              type="number"
              name="vel_y"
              id="vel_y-{satellite.id}"
              value={satellite.initial_state.vel_y}
              step="any"
              required
            /></td
          >
        </tr>
        <tr>
          <td></td>
          <td>Y</td>
          <td
            ><input
              type="number"
              name="vel_z"
              id="vel_z-{satellite.id}"
              value={satellite.initial_state.vel_z}
              step="any"
              required
            /></td
          >
        </tr>
      </tbody>
    </table>
  </div>
  <div class="satellite-actions">
    <button type="submit" disabled={isSaving}>
      {isSaving ? "Saving..." : "Save"}
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
    align-items: center;
    justify-content: space-between;
    gap: 10px;
    padding: 0.75rem;
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
    border: 1px solid var(--main-font-colour);
    cursor: pointer;
    padding: 0.25rem 0.75rem;
  }

  button:disabled {
    cursor: wait;
    opacity: 0.7;
  }

  .satellite-status {
    text-align: right;
  }
</style>
