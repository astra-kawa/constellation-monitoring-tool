<script>
  import BackendStatusIndicator from "./BackendStatusIndicator.svelte";
  import DoyTimeDisplay from "./DoyTimeDisplay.svelte";
</script>

<div class="titlebar">
  <span>Fleet Monitoring Tool</span>
  <DoyTimeDisplay />
  <BackendStatusIndicator />
</div>

<style>
  .titlebar {
    display: flex;
    flex-direction: row;
    justify-content: space-around;
    flex-basis: 0;
    align-items: center;
  }
</style>
