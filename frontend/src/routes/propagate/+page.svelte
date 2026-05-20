<script lang="ts">
  import { getConstellationData } from "$lib/util";
  import { getFormNumberValue } from "$lib/utils/utils";

  async function handleSubmit(event: SubmitEvent) {
    event.preventDefault();

    const form = event.currentTarget as HTMLFormElement | null;
    if (!form) {
      return;
    }

    const formData = new FormData(form);

    const durationValue = getFormNumberValue(formData, "duration_seconds");
    const stepValue = getFormNumberValue(formData, "step_seconds");

    const payload = {
      duration_seconds: durationValue,
      step_seconds: stepValue,
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

    console.log(response);

    // let target = event.target;
    // if (!target) {
    //   return;
    // }
    // target.reset();
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
