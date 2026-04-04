<script lang="ts">
  import { onMount } from "svelte";

  let healthMessage = $state("Loading backend status...");
  let messageClass = $state("backend-idle");

  onMount(async () => {
    try {
      const healthResponse = await fetch("http://127.0.0.1:3000/api/health");
      if (!healthResponse.ok) {
        throw new Error("backend unavailable");
        messageClass = "backend-unhealthy";
      }

      // todo - display health message
      const health = await healthResponse.json();
      healthMessage = "BACKEND CONNECTED";
      messageClass = "backend-healthy";
    } catch (error) {
      healthMessage = "BACKEND DISCONNECTED";
      messageClass = "backend-unhealthy";
    }
  });
</script>

<span class="status-indicator {messageClass}">{healthMessage}</span>

<style>
  .status-indicator {
    padding: 5px;
  }

  .backend-idle {
    border: 1px dashed var(--idle-colour);
    color: var(--idle-colour);
  }

  .backend-healthy {
    border: 1px dashed var(--healthy-colour);
    color: var(--healthy-colour);
  }

  .backend-unhealthy {
    border: 1px dashed var(--unhealthy_colour);
    color: var(--unhealthy_colour);
  }
</style>
