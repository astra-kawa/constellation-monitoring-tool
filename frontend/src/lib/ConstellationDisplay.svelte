<script lang="ts">
  import { onMount } from "svelte";
  import SatelliteCard from "./SatelliteCard.svelte";

  interface EciState {
    epoch: Date;
    pos_x: number;
    pos_y: number;
    pos_z: number;
    vel_x: number;
    vel_y: number;
    vel_z: number;
  }

  interface Satellite {
    id: string;
    initial_state: EciState;
  }

  let constellationData: Satellite[];

  onMount(async () => {
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
  });
</script>

<div class="constellation-display">
  <div class="satellite-data">
    {#each constellationData as satellite: Satellite}
      <SatelliteCard {satellite} />
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
    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  }
</style>
