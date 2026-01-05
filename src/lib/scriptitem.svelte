<script>
  let { icon, name, luafile = false, selected = false, expanded = false, renaming = false, onrename = () => {} } = $props();
  let input;
  let val = $state(name);

  $effect(() => {
    if (renaming) {
      val = name;
      input?.focus();
      input?.select();
    }
  });
</script>

<div class={`w-full p-2 rounded-md font-[565] flex gap-3 items-center cursor-default ${selected ? "bg-[#01B8BA]/6.25 text-[#01B8BA] hover:bg-[#2ce8ec]/16" : "text-white/50 hover:bg-white/12.5"}`}>
  {#if !luafile}
    <img class={`w-4.5 h-4.5 flex-shrink-0 transition-transform ${expanded ? "rotate-0" : "-rotate-90"}`} alt="arrow" src="/arrowdown.svg" />
  {/if}

  <img class="w-5 h-5 flex-shrink-0" alt="icon" src={icon} />
  
  {#if renaming}
    <input
      bind:this={input}
      bind:value={val}
      class="flex-1 bg-[#323232] px-2 py-0.5 rounded outline-none text-white"
      onkeydown={(e) => {
        if (e.key === 'Enter') onrename(val);
        if (e.key === 'Escape') {
          e.preventDefault();
          e.stopPropagation();
          onrename(name);
        }
      }}
      onblur={() => onrename(name)}
      onclick={(e) => e.stopPropagation()}
    />
  {:else}
    <span class="truncate">{name}</span>
  {/if}
</div>