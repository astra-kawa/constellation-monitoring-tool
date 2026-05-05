<script lang="ts">
  import { getConstellationData } from "$lib/util";

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    const form = event.currentTarget as HTMLFormElement | null;
    if (!form) {
      return;
    }

    const formData = new FormData(form);

    const durationValue = formData.get("duration_seconds");
    if (!durationValue) {
      return;
    }

    const stepValue = formData.get("step_seconds");
    if (!stepValue) {
      return;
    }

    // const constellationData = await getConstellationData();

    const payload = {
      duration: durationValue,
      step: stepValue,
      // constellation: constellationData,
    };

    console.log(payload);

    const response = await fetch("http://127.0.0.1:3000/api/propagate", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(payload),
    });

    if (!response.ok) {
      console.log("Failed to propagate constellation");
    }

    let target = event.target;
    if (!target) {
      return;
    }
    target.reset();
  }
</script>

<div>
  <p>Propagate Constellation</p>
  <form method="POST" onsubmit={handleSubmit}>
    <label for="duration_seconds">
      Duration (s)
      <input type="number" name="duration_seconds" />
    </label>
    <label for="step_seconds">
      Step (s)
      <input type="number" name="step_seconds" />
    </label>
    <button>SUBMIT</button>
  </form>
</div>

<style>
  input {
    border: none;
    background-color: none;
    background: none;
    font-family: var(--main-font-family);
    font-size: var(--secondary-font-size);
  }

  button {
    border: none;
    background-color: var(--interactive-colour1);
    cursor: pointer;
    font-family: var(--main-font-family);
    font-size: var(--secondary-font-size);
  }

  input {
    border-bottom: 1px dashed var(--main-font-colour);
  }
</style>
