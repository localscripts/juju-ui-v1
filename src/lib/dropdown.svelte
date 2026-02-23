<script>
  let { x = 0, y = 0, options = [], onclose = () => {} } = $props();
</script>

<svelte:window onclick={onclose} oncontextmenu={onclose} />

<div 
  class="absolute overflow-hidden rounded-md shadow-lg min-w-40 z-[9999]"
  style="left: {x}px; top: {y}px; background: var(--menu-bg);"
  onclick={(e) => e.stopPropagation()}
>
  {#each options as opt}
    {#if opt.separator}
      <div class="h-px my-1" style="background: var(--menu-separator);"></div>
    {:else}
      <button
        onclick={() => { opt.action(); onclose(); }}
        class="w-full px-4 py-2 text-left text-[15px] font-[565]"
        style="color: var(--menu-item-text);"
        onmouseenter={(e) => e.currentTarget.style.background = 'var(--menu-item-hover)'}
        onmouseleave={(e) => e.currentTarget.style.background = ''}
      >{opt.label}</button>
    {/if}
  {/each}
</div>