<script>
  let { x = 0, y = 0, options = [], onclose = () => {} } = $props();

  let el = $state(null);
  let rx = $state(x);
  let ry = $state(y);

  $effect(() => {
    if (!el) return;
    const r = el.getBoundingClientRect();
    rx = x + r.width > window.innerWidth ? x - r.width : x;
    ry = y + r.height > window.innerHeight ? y - r.height : y;
  });
</script>

<svelte:window onclick={onclose} oncontextmenu={onclose} />

<div
  bind:this={el}
  class="overflow-hidden fixed rounded-md shadow-lg min-w-40 z-[9999]"
  style="left: {rx}px; top: {ry}px; background: var(--ctx-bg);"
  onclick={(e) => e.stopPropagation()}
>
  {#each options as opt}
    <button
      onclick={() => { if (!opt.disabled) { opt.action(); onclose(); } }}
      class="w-full px-4 py-2 text-left font-[565] flex items-center gap-2"
      style={opt.disabled
        ? 'color: var(--ctx-item-disabled); cursor: default;'
        : 'color: var(--ctx-item-text); cursor: pointer;'}
      onmouseenter={(e) => { if (!opt.disabled) e.currentTarget.style.background = 'var(--ctx-item-hover)'; }}
      onmouseleave={(e) => { e.currentTarget.style.background = ''; }}
    >{opt.label}</button>
  {/each}
</div>