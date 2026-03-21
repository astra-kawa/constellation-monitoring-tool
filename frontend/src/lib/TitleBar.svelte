<script>
  import BackendStatusIndicator from "./BackendStatusIndicator.svelte";
</script>

<div class="titlebar">
  <span>Fleet Monitor</span>
  <span>2026-001 12:00:00</span>
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
