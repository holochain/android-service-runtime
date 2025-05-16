<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { type AppInfo } from "tauri-plugin-holochain-service-api";
  import BaseLabelled from "./BaseLabelled.svelte";
  import BaseInstalledAppCardAdvanced from "./BaseInstalledAppCardAdvanced.svelte";
  import BaseLoadingToggle from './BaseLoadingToggle.svelte';
  import { _ } from 'svelte-i18n';
  
  export let appInfo: AppInfo;
  export let loadingToggleEnable: boolean = false;
  let showAdvanced = false;

	const dispatch = createEventDispatcher<{
    toggleEnable: {}
	}>();
</script>

<div class="px-4 py-8 bg-base-200 border-b-2 border-solid border-gray-300">
  <div class="flex justify-between items-start">
    <h3 class="m-0">{appInfo.installedAppId.split('-')[0]}</h3>
    <BaseLoadingToggle loading={loadingToggleEnable} value={appInfo.status.type === "Running"} on:toggle={() => dispatch('toggleEnable')} />
  </div>
  <div class="mt-4">
    <BaseLabelled label={$_('app_id')}>{appInfo.installedAppId}</BaseLabelled>
  </div>

  <div class={`collapse collapse-arrow border-base-300 bg-base-200 border`}>
    <button class="collapse-title text-sm font-medium bg-base-300 shadow-none focus:border-none" on:click={() => (showAdvanced = !showAdvanced)}>
      {$_('advanced_settings')}
    </button>
    <div class={`p-4 text-sm font-medium bg-base-300  ${!showAdvanced ? 'hidden' : ''}`}>
      <BaseInstalledAppCardAdvanced {appInfo} />
    </div>
  </div>
</div>