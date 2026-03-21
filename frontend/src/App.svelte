<script lang="ts">
  import { onMount } from "svelte";

  let healthMessage = "Loading backend status...";
  let constellationMessage = "Loading constellation data...";
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

    try {
      const constellationResponse = await fetch(
        "http://127.0.0.1:3000/api/constellation",
      );
      if (!constellationResponse.ok) {
        throw new Error("backend unavailable");
      }

      constellationData = await constellationResponse.json();

      console.log(constellationData);
      constellationMessage = "Retrieved constellation data:";
    } catch (error) {
      constellationMessage = "Backend unavailable";
    }
  });
</script>

<main>
  <p>Fleet Monitoring Tool</p>
  <p>Backend health: {healthMessage}</p>
  <p>Constellation data: {constellationMessage}</p>
</main>

<style>
</style>
