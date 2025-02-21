<script lang="ts">
  export let loading = false;
  export let btnClass = '';
</script>

<button class={`btn ${btnClass}`} on:click>
  {#if loading}
  <span class="loading loading-spinner"></span>
  {/if}
  
  <slot></slot>
</button>