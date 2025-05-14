<script lang="ts">
	import { onMount } from "svelte";
  import BaseInstalledAppCard from "../components/BaseInstalledAppCard.svelte";
  import { apps, loadingApps, loadingToggleEnableApp, loadApps, toggleEnableApp, isRunning } from "../stores/holochain";
  import { _ } from 'svelte-i18n';

  onMount(() => {
    if($isRunning) {
      loadApps();
    }
  });
</script>

<div class="flex justify-between items-center mx-4">
  <h4>{$_('installed_apps')}</h4>
  <button on:click={loadApps} disabled={!$isRunning} class="btn btn-sm btn-circle btn-outline">
    {#if $loadingApps}
      <span class="loading loading-spinner loading-xs"></span>
    {:else}
      <svg xmlns="http://www.w3.org/2000/svg" width="1.5em" height="1.5em" viewBox="0 0 24 24"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.651 7.65a7.131 7.131 0 0 0-12.68 3.15M18.001 4v4h-4m-7.652 8.35a7.13 7.13 0 0 0 12.68-3.15M6 20v-4h4"/></svg>
    {/if}
  </button>
</div>

{#if !$isRunning}
  <div class="alert my-4">{$_('start_service_to_view_apps')}</div>
{:else if $apps.length > 0}
  {#each $apps as appInfo}
    <BaseInstalledAppCard 
      {appInfo} 
      loadingToggleEnable={$loadingToggleEnableApp[appInfo.installedAppId]}
      on:toggleEnable={() => toggleEnableApp(appInfo)}
    />
  {/each} 
{:else}
  <div class="alert my-4">{$_('no_apps_installed')}</div>
{/if}
