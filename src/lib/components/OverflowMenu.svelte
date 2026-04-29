<script lang="ts">
  interface MenuItem {
    label: string;
    icon?: string;
    action: () => void;
    danger?: boolean;
  }

  let { items }: { items: MenuItem[] } = $props();
  let open = $state(false);

  function toggle() {
    open = !open;
  }

  function handleItemClick(item: MenuItem) {
    open = false;
    item.action();
  }

  function handleBackdropClick() {
    open = false;
  }
</script>

{#if open}
  <div
    class="backdrop"
    role="presentation"
    onclick={handleBackdropClick}
  ></div>
{/if}

<div class="overflow-menu-wrapper">
  <button class="trigger" onclick={toggle} aria-label="更多操作">
    ⋮
  </button>

  {#if open}
    <div class="menu">
      {#each items as item}
        <button
          class="menu-item"
          class:menu-item-danger={item.danger}
          onclick={() => handleItemClick(item)}
        >
          {#if item.icon}
            <span class="menu-item-icon">{item.icon}</span>
          {/if}
          {item.label}
        </button>
      {/each}
    </div>
  {/if}
</div>

<style>
  .overflow-menu-wrapper {
    position: relative;
    display: inline-flex;
  }

  .backdrop {
    position: fixed;
    inset: 0;
    z-index: 19;
  }

  .trigger {
    width: 24px;
    height: 24px;
    border-radius: 50%;
    border: none;
    background: transparent;
    color: var(--text-secondary);
    font-size: 14px;
    line-height: 1;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: background-color 0.15s;
  }

  .trigger:hover {
    background-color: var(--menu-hover);
  }

  .menu {
    position: absolute;
    bottom: 100%;
    right: 0;
    z-index: 50;
    min-width: 100px;
    background-color: var(--menu-bg);
    border-radius: 8px;
    box-shadow: var(--menu-shadow);
    padding: 4px 0;
    margin-bottom: 4px;
    pointer-events: auto;
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 6px;
    width: 100%;
    padding: 5px 12px;
    border: none;
    background: transparent;
    color: var(--text-primary);
    font-size: 11px;
    line-height: 1.4;
    cursor: pointer;
    text-align: left;
    white-space: nowrap;
    transition: background-color 0.1s;
  }

  .menu-item:hover {
    background-color: var(--menu-hover);
  }

  .menu-item-danger {
    color: #e55;
  }

  .menu-item-icon {
    flex-shrink: 0;
    font-size: 11px;
  }
</style>
