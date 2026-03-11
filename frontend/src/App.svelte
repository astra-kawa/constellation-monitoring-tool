<script lang="ts">
  import { onMount } from "svelte";

  let message = "Loading backend status...";

  onMount(async () => {
    try {
      const healthResponse = await fetch("http://127.0.0.1:3000/api/health");
      if (!healthResponse.ok) {
        throw new Error("backend unavailable");
      }

      const health = await healthResponse.json();
      message = health.message;
    } catch (error) {
      message = "Backend unavailable";
    }
  });
</script>

<main>
  <p>Fleet Monitoring Tool</p>
  <p>Backend health: {message}</p>
</main>

<style>

</style>
