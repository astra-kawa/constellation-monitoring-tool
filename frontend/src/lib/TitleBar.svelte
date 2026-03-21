<script>
  import BackendStatusIndicator from "./BackendStatusIndicator.svelte";
</script>

<div class="titlebar">
  <span>Fleet Monitor</span>
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
