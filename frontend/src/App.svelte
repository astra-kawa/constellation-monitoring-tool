<script lang="ts">
  import { onMount } from "svelte";
  import ConstellationDisplay from "./lib/ConstellationDisplay.svelte";

  let healthMessage = "Loading backend status...";
  let constellationData;

  onMount(async () => {
    try {
      const healthResponse = await fetch("http://127.0.0.1:3000/api/health");
      if (!healthResponse.ok) {
        throw new Error("backend unavailable");
      }

      const health = await healthResponse.json();
      healthMessage = health.message;
    } catch (error) {
      healthMessage = "Backend unavailable";
    }
  });
</script>

<main>
  <p>Fleet Monitoring Tool</p>
  <p>Backend health: {healthMessage}</p>
  <ConstellationDisplay />
</main>

<style>
</style>
