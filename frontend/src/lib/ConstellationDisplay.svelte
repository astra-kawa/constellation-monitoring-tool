<script lang="ts">
  import { onMount } from "svelte";

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
    name: string;
    initial_state: EciState;
  }

  let constellationMessage = "Loading constellation data...";
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

      constellationMessage = `Retrieved constellation data: ${constellationData.length}`;
    } catch (error) {
      constellationMessage = "Backend unavailable";
    }
  });
</script>

<div>
  <p>{constellationMessage}</p>
</div>
