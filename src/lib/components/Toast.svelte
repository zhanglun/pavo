<script lang="ts">
  import { toastMessage, toastVisible } from "../utils/wallpaper";
</script>

{#if $toastVisible}
  <div class="toast-container">
    <div class="toast">
      {$toastMessage}
    </div>
  </div>
{/if}

<style>
  .toast-container {
    position: fixed;
    bottom: 24px;
    left: 50%;
    transform: translateX(-50%);
    z-index: 9999;
    animation: toastIn 0.2s ease;
    transition: opacity 0.3s ease;
  }

  .toast {
    padding: 8px 20px;
    border-radius: 99px;
    background: rgba(0, 0, 0, 0.75);
    color: rgba(255, 255, 255, 0.95);
    font-size: 13px;
    line-height: 1.4;
    white-space: nowrap;
    backdrop-filter: blur(8px);
  }

  @keyframes toastIn {
    from { opacity: 0; transform: translateY(8px); }
    to { opacity: 1; transform: translateY(0); }
  }
</style>
