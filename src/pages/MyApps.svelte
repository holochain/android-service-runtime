<script lang="ts">
  import BaseInstalledAppCard from "../components/BaseInstalledAppCard.svelte";
  import { installedApps, loadingLoadInstalledApps, loadingToggleEnableApp, loadInstalledApps, toggleEnableApp } from "../stores/holochain";
</script>

<div class="flex justify-between items-center mx-4">
  <h4>Installed Apps</h4>
  <button on:click={loadInstalledApps} class="btn btn-sm btn-circle btn-outline">
    {#if $loadingLoadInstalledApps}
      <span class="loading loading-spinner loading-xs"></span>
    {:else}
      <svg xmlns="http://www.w3.org/2000/svg" width="1.5em" height="1.5em" viewBox="0 0 24 24"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.651 7.65a7.131 7.131 0 0 0-12.68 3.15M18.001 4v4h-4m-7.652 8.35a7.13 7.13 0 0 0 12.68-3.15M6 20v-4h4"/></svg>
    {/if}
  </button>
</div>

{#each $installedApps as appInfo}
  <BaseInstalledAppCard 
    {appInfo} 
    loadingToggleEnable={$loadingToggleEnableApp[appInfo.installedAppId]}
    on:toggleEnable={() => toggleEnableApp(appInfo)}
  />
{/each}

