<script lang="ts">
	import BaseLoadingButton from "../components/BaseLoadingButton.svelte";
	import { BUNDLED_APPS } from "../config";
	import { installBundledApp, loadingInstallBundledApp } from "../stores/holochain";

</script>

<div class="flex justify-between items-center mx-4">
  <h4>App Library</h4>
</div>
{#each Object.entries(BUNDLED_APPS) as [key, app]}
  <div class="bg-base-200 p-4">
    <h4 class="m-0">{app.name}</h4>
    <p>{app.description}</p>
    <div class="flex justify-end">
      <BaseLoadingButton 
        btnClass="btn-md btn-outline" 
        loading={$loadingInstallBundledApp[key]}
        on:click={() => installBundledApp(key)}
      >
        Install
      </BaseLoadingButton>
    </div>
  </div>
{/each}