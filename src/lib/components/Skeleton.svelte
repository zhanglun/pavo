<style>
  .skeleton {
    width: 100%;
    aspect-ratio: 16 / 9;
    background-color: var(--placeholder);
    border-radius: 6px;
    animation: pulse 2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%,
    100% {
      opacity: 1;
    }
    50% {
      opacity: 0.5;
    }
  }
</style>

<div class="skeleton" />