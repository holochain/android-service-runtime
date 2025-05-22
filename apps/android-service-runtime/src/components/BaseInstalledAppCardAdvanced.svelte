<script lang="ts">
  import { encodeHashToBase64 } from "@holochain/client";
  import { type AppInfo } from "tauri-plugin-holochain-service-api";
  import BaseLabelled from "./BaseLabelled.svelte";
  import BaseLoadingButton from "./BaseLoadingButton.svelte";
  import { loadingUninstallApp, loadUninstallApp } from "../stores/holochain";
  import { decode } from "@msgpack/msgpack";
  import { _ } from 'svelte-i18n';

  export let appInfo: AppInfo;
</script>

<div >
  <BaseLabelled label={$_('app_id')}>
    {appInfo.installedAppId}
  </BaseLabelled>
  <BaseLabelled label={$_('status')}>
    {appInfo.status.type}
  </BaseLabelled>
  <BaseLabelled label={$_('agent_public_key')}>
    {encodeHashToBase64(Uint8Array.from(appInfo.agentPubKey))}
  </BaseLabelled>

  <BaseLabelled label={$_('cells')}>
    <ul>
      {#each Object.entries(appInfo.cellInfo) as [baseRoleName, allCellInfos]}
        <li>
          <BaseLabelled label={$_('role_name')}>
            {baseRoleName}
          </BaseLabelled>
          {#each allCellInfos as cellInfo}
            <BaseLabelled label={$_('cell_name')}>
              {cellInfo.v1.name}
            </BaseLabelled>
            <BaseLabelled label={$_('agent_public_key')}>
              {encodeHashToBase64(Uint8Array.from(cellInfo.v1.cellId.agentPubKey))}
            </BaseLabelled>
            <BaseLabelled label={$_('dna_hash')}>
              {encodeHashToBase64(Uint8Array.from(cellInfo.v1.cellId.dnaHash))}
            </BaseLabelled>
            <BaseLabelled label={$_('dna_modifiers')}>
              <div style="margin-left: 20px">
                <BaseLabelled label={$_('network_seed')}>
                  {cellInfo.v1.dnaModifiers.networkSeed}
                </BaseLabelled>
                <BaseLabelled label={$_('origin_time')}>
                  {cellInfo.v1.dnaModifiers.originTime}
                </BaseLabelled>
                <BaseLabelled label={$_('properties')}>
                  {decode(cellInfo.v1.dnaModifiers.properties)}
                </BaseLabelled>
                <BaseLabelled label={$_('quantum_time')}>
                  {cellInfo.v1.dnaModifiers.quantumTime.secs} {$_('seconds_short')}, {cellInfo.v1.dnaModifiers.quantumTime.secs} {$_('nanoseconds_short')}
                </BaseLabelled>
              </div>
            </BaseLabelled>
          {/each}
        </li>
      {/each}
    </ul>
  </BaseLabelled>
  
  <div class="flex justify-end">
    <BaseLoadingButton btnClass="btn-sm btn-error" loading={$loadingUninstallApp[appInfo.installedAppId]} on:click={() => loadUninstallApp(appInfo.installedAppId)}>
      {$_('uninstall')}
    </BaseLoadingButton>
  </div>
</div>