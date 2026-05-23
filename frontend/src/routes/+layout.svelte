<script lang="ts">
  import "$lib/styles.css";
  import favicon from "$lib/assets/favicon.svg";
  import TitleBar from "$lib/components/TitleBar.svelte";

  let { children } = $props();
</script>

<main>
  <TitleBar />
  {@render children()}
</main>
