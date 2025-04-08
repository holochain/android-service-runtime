<script lang="ts">
  import { encodeHashToBase64 } from "@holochain/client";
  import { type AppInfo } from "tauri-plugin-holochain-service-api";
  import BaseLabelled from "./BaseLabelled.svelte";
  import BaseLoadingButton from "./BaseLoadingButton.svelte";
  import { loadingUninstallApp, loadUninstallApp } from "../stores/holochain";
  import { decode } from "@msgpack/msgpack";
  export let appInfo: AppInfo;
</script>

<div >
  <BaseLabelled label="App Id">
    {appInfo.installedAppId}
  </BaseLabelled>
  <BaseLabelled label="Status">
    {appInfo.status.type}
  </BaseLabelled>
  <BaseLabelled label="Agent Pub Key">
    {encodeHashToBase64(Uint8Array.from(appInfo.agentPubKey))}
  </BaseLabelled>

  <BaseLabelled label="Cells">
    <ul>
      {#each Object.entries(appInfo.cellInfo) as [baseRoleName, allCellInfos]}
        <li>
          <BaseLabelled label="Role Name">
            {baseRoleName}
          </BaseLabelled>
          {#each allCellInfos as cellInfo}
            <BaseLabelled label="Cell Name">
              {cellInfo.v1.name}
            </BaseLabelled>
            <BaseLabelled label="Agent Pub Key">
              {encodeHashToBase64(Uint8Array.from(cellInfo.v1.cellId.agentPubKey))}
            </BaseLabelled>
            <BaseLabelled label="DNA Hash">
              {encodeHashToBase64(Uint8Array.from(cellInfo.v1.cellId.dnaHash))}
            </BaseLabelled>
            <BaseLabelled label="DNA Modifiers">
              <div style="margin-left: 20px">
                <BaseLabelled label="Network Seed">
                  {cellInfo.v1.dnaModifiers.networkSeed}
                </BaseLabelled>
                <BaseLabelled label="Origin Time">
                  {cellInfo.v1.dnaModifiers.originTime}
                </BaseLabelled>
                <BaseLabelled label="Properties">
                  {decode(cellInfo.v1.dnaModifiers.properties)}
                </BaseLabelled>
                <BaseLabelled label="Quantum Time">
                  {cellInfo.v1.dnaModifiers.quantumTime.secs} secs, {cellInfo.v1.dnaModifiers.quantumTime.secs} nanos
                </BaseLabelled>
              </div>
            </BaseLabelled>
          {/each}
        </li>
      {/each}
    </ul>
  </BaseLabelled>
  
  <div class="flex justify-end">
    <BaseLoadingButton btnClass="btn-sm btn-error" loading={$loadingUninstallApp[appInfo.installedAppId]} on:click={() => loadUninstallApp(appInfo.installedAppId)}>Uninstall</BaseLoadingButton>
  </div>
</div>