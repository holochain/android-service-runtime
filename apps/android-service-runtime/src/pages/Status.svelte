<script lang="ts">
  import { loadingLaunch, toggleLaunch, isRunning, loadIsRunning } from "../stores/holochain";
  import BaseLoadingToggle from "../components/BaseLoadingToggle.svelte";
  import { _ } from 'svelte-i18n';

  loadIsRunning();
</script>

<h4 class="mx-4">{$_('service_status')}</h4>
<div class="p-4 bg-base-200 flex justify-between items-center">
  <h3 class="m-0">{$isRunning ? $_('running') : $_('stopped') }</h3>
  <BaseLoadingToggle loading={$loadingLaunch} value={$isRunning} on:toggle={() => toggleLaunch()} />
</div>


  