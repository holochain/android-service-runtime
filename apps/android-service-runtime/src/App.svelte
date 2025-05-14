<script lang="ts">
  import './app.css';
  import Status from './pages/Status.svelte';
  import MyApps from './pages/MyApps.svelte';
  import Toasts from './components/Toasts.svelte';
  import { _ } from 'svelte-i18n';

  export let activeTab = 0;

  $: tabs = [
    {text: $_('status'), component: Status},
    {text: $_('my_apps'), component: MyApps},
  ];
</script>

<nav class="navbar bg-base-100 border-b-2 border-gray-300 fixed top-0 z-50">
  <div class="flex-1">
    <span class="btn btn-ghost text-xl">{$_('app_title')}</span>
  </div>
</nav>

<main class="prose my-20">
  <svelte:component this={tabs[activeTab].component} />
</main>

<div role="tablist" class="tabs tabs-boxed tabs-lg fixed bottom-0 w-full bg-base-100 border-t-2 border-gray-300 z-50">
  {#each tabs as tab, i}
    <button role="tab" class={`tab ${activeTab === i ? 'tab-active' : ''}`} on:click={() => (activeTab = i)}>{tab.text}</button>
  {/each}
</div>

<Toasts />