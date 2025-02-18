<script lang="ts">
	import { createEventDispatcher } from "svelte";

  export let loading = false;
  export let value = true;

  const dispatch = createEventDispatcher<{ toggle: boolean }>();
</script>
<div class="flex justify-between items-center">
  {#if loading}
    <span class="loading loading-spinner loading-xs mr-2"></span>
  {/if}
  <input
    type="checkbox" 
    class={`toggle toggle-lg ${value ? '!text-success !bg-success' : '!text-error !bg-error'}`}
    checked={value} 
    on:click={() => dispatch('toggle', !value)}
    disabled={loading}
  />
</div>