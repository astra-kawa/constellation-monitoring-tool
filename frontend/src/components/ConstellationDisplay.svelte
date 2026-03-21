<script lang="ts">
  import { onMount } from "svelte";
  import SatelliteCard from "./SatelliteCard.svelte";
  import type { Satellite } from "../lib/types";

  async function refreshConstellationData() {
    try {
      const constellationResponse = await fetch(
        "http://127.0.0.1:3000/api/constellation",
      );
      if (!constellationResponse.ok) {
        throw new Error("backend unavailable");
      }

      const response = await constellationResponse.json();
      constellationData = response.satellites;
    } catch (error) {
      console.log(`Error: ${error}`);
    }
  }

  let constellationData: Satellite[] = $state([]);

  onMount(async () => {
    refreshConstellationData();
  });
</script>

<div class="constellation-display">
  <div class="satellite-data">
    {#each constellationData as satellite: Satellite}
      <SatelliteCard {satellite} {refreshConstellationData} />
    {/each}
  </div>
</div>

<style>
  .constellation-display {
    margin-top: 2rem;
  }

  .satellite-data {
    display: grid;
    gap: 15px;
    grid-template-columns: repeat(auto-fit, minmax(325px, 1fr));
  }
</style>
