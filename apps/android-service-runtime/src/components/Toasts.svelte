<script lang="ts">
  import { toastsList } from '../stores/toasts';
</script>

<div class="toast toast-center z-50 mb-16 w-full min-w-full break-normal whitespace-normal">
  {#each $toastsList as toast}
    <div class="alert alert-{toast.status} text-xs text-left">{toast.text}</div>
  {/each}
</div>

<!-- This ensures that the alert-* tailwind classes are included in the css bundle-->
<div class="alert-error alert-success alert-warning alert-info hidden"></div>
