<script>
  let { x = 0, y = 0, options = [], onclose = () => {} } = $props();
</script>

<svelte:window onclick={onclose} oncontextmenu={onclose} />

<div 
  class="fixed bg-[#242424] rounded-md shadow-lg min-w-40 z-50"
  style="left: {x}px; top: {y}px;"
  onclick={(e) => e.stopPropagation()}
>
  {#each options as opt}
    <button
      onclick={() => { if (!opt.disabled) { opt.action(); onclose(); } }}
      class={`w-full px-4 py-2 text-left text-sm flex items-center gap-2 ${opt.disabled ? 'text-white/25 cursor-default' : 'text-white/75 hover:bg-[#323232] cursor-pointer'}`}
    >
      {opt.label}
    </button>
  {/each}
</div>