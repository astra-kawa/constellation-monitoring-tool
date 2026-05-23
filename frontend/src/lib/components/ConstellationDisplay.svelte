<script lang="ts">
  import { onMount } from "svelte";
  import SatelliteCard from "./SatelliteCard.svelte";
  import type { Satellite } from "$lib/types";
  import ConstellationActionsDisplay from "./ConstellationActionsDisplay.svelte";
  import { getConstellationData } from "$lib/util";

  async function refreshConstellationData() {
    constellationData = await getConstellationData();
  }

  let constellationData: Satellite[] = $state([]);

  onMount(async () => {
    refreshConstellationData();
  });
</script>

<div class="constellation-display">
  <ConstellationActionsDisplay {refreshConstellationData} />
  <div class="satellite-data">
    {#each constellationData as satellite: Satellite (satellite.id)}
      <SatelliteCard {satellite} {refreshConstellationData} />
    {/each}
  </div>
</div>

<style>
  .constellation-display {
    margin-top: 2rem;
  }

  .satellite-data {
    margin-top: 1rem;
    display: grid;
    gap: 15px;
    grid-template-columns: repeat(auto-fit, minmax(325px, 1fr));
  }
</style>
