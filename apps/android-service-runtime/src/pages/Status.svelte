<script lang="ts">
  import { loadingLaunch, toggleLaunch, isRunning, loadIsRunning } from "../stores/holochain";
  import BaseLoadingToggle from "../components/BaseLoadingToggle.svelte";
  
  loadIsRunning();
</script>

<h4 class="mx-4">Service Status</h4>
<div class="p-4 bg-base-200 flex justify-between items-center">
  <h3 class="m-0">{$isRunning ? 'Running' : 'Stopped' }</h3>
  <BaseLoadingToggle loading={$loadingLaunch} value={$isRunning} on:toggle={() => toggleLaunch()} />
</div>


  