<script lang="ts">
  import { toasts, toastsList } from '../stores/toasts';
</script>

<div class="toast toast-center z-50 mb-16">
  {#each $toastsList as toast}
    <div class={`alert alert-${toast.status} text-xs`}>
      <div class="break-words">{toast.text}</div>
    </div>
  {/each}
</div>
