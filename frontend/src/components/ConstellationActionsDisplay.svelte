<script lang="ts">
  let { refreshConstellationData }: { refreshConstellationData: Function } =
    $props();

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    const form = event.currentTarget as HTMLFormElement | null;
    if (!form) {
      return;
    }

    const formData = new FormData(form);
    const value = formData.get("satellite_id");
    if (!value) {
      return;
    }

    if (typeof value !== "string" || value.trim() === "") {
      console.log("Missing satellite_id");
    }

    const payload = {
      id: value.trim(),
      initial_state: {
        epoch: "2026-01-01T00:00:00Z",
        pos_x: 0.0,
        pos_y: 0.0,
        pos_z: 0.0,
        vel_x: 0.0,
        vel_y: 0.0,
        vel_z: 0.0,
      },
    };

    const response = await fetch(
      `http://127.0.0.1:3000/api/constellation/${encodeURIComponent(value)}`,
      {
        method: "PUT",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
      },
    );

    if (!response.ok) {
      console.log(`Failed to create satellite ${value}`);
    }

    let target = event.target;
    if (!target) {
      return;
    }
    target.reset();

    refreshConstellationData();
  }
</script>

<div class="constellation-actions">
  <form onsubmit={handleSubmit}>
    <input type="text" name="satellite_id" />
    <button class="new-button">NEW</button>
  </form>
</div>

<style>
  .new-button {
    border: none;
    background-color: var(--interactive-colour1);
    cursor: pointer;
    font-family: var(--main-font-family);
    font-size: var(--secondary-font-size);
  }

  input {
    border: none;
    border-bottom: 1px dashed var(--main-font-colour);
    background-color: none;
    background: none;
    font-family: var(--main-font-family);
    font-size: var(--secondary-font-size);
  }
</style>
