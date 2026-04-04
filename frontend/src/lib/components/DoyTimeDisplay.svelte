<script lang="ts">
  import { onMount } from "svelte";

  const getDayOfYear = (date: Date) => {
    const startOfYear = new Date(Date.UTC(date.getUTCFullYear(), 0, 1));
    const differenceInMilliseconds = date.getTime() - startOfYear.getTime();

    const millisecondsPerDay = 1000 * 60 * 60 * 24;

    const day = Math.floor(differenceInMilliseconds / millisecondsPerDay) + 1;

    return day;
  };

  function zeroPad(num: number, size: number) {
    return num.toString().padStart(size, "0");
  }

  let time = $state(new Date());

  onMount(() => {
    const interval = setInterval(() => {
      time = new Date();
    }, 1000);

    return () => {
      clearInterval(interval);
    };
  });

  let hours = $derived(time.getUTCHours());
  let minutes = $derived(time.getUTCMinutes());
  let seconds = $derived(time.getUTCSeconds());
  let dayOfYear = $derived(getDayOfYear(time));
  let year = $derived(time.getUTCFullYear());
</script>

<div class="clock-display">
  {zeroPad(year, 4)}-{zeroPad(dayOfYear, 3)}
  {zeroPad(hours, 2)}:{zeroPad(minutes, 2)}:{zeroPad(seconds, 2)}
</div>

<style>
  .clock-display {
    font-variant-numeric: tabular-nums;
  }
</style>
