<script>
  let { value = $bindable(""), filterOpen = $bindable(false), filters = [], toggleFilter = () => {}, filterOpts = [] } = $props();
</script>

<div class="w-full flex items-center gap-2">
  <div class="flex-1 flex items-center gap-3 text-[15px] font-[565] p-2.5 px-3 rounded-md"
    style="color: var(--search-text); background: var(--search-bg);"
    onmouseenter={(e) => e.currentTarget.style.background = 'var(--search-hover)'}
    onmouseleave={(e) => e.currentTarget.style.background = 'var(--search-bg)'}
  >
    <svg xmlns="http://www.w3.org/2000/svg" class="size-5 flex-shrink-0" viewBox="0 0 24 24" fill="var(--search-icon)">
      <path fill-rule="evenodd" clip-rule="evenodd" d="M10.7498 1.75C5.77919 1.75 1.74976 5.77944 1.74976 10.75C1.74976 15.7206 5.77919 19.75 10.7498 19.75C12.8747 19.75 14.8277 19.0135 16.3674 17.7819L20.8355 22.25L22.2498 20.8358L17.7816 16.3677C19.0133 14.8281 19.7498 12.875 19.7498 10.75C19.7498 5.77944 15.7203 1.75 10.7498 1.75ZM3.74976 10.75C3.74976 6.88401 6.88376 3.75 10.7498 3.75C14.6157 3.75 17.7498 6.88401 17.7498 10.75C17.7498 14.616 14.6157 17.75 10.7498 17.75C6.88376 17.75 3.74976 14.616 3.74976 10.75Z"></path>
    </svg>
    <input bind:value class="w-full bg-transparent outline-none" style="color: var(--search-text);" placeholder="Search..." />
  </div>

  <div class="relative">
    <button
      class="flex items-center justify-center rounded-md aspect-square h-[42px] relative"
      style="background: var(--search-filter-btn-bg);"
      onmouseenter={(e) => e.currentTarget.style.background = 'var(--search-filter-btn-hover)'}
      onmouseleave={(e) => e.currentTarget.style.background = 'var(--search-filter-btn-bg)'}
      onclick={(e) => { e.stopPropagation(); filterOpen = !filterOpen; }}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="var(--search-filter-btn-icon)" class="size-6">
        <path d="M22 5V7H2V5H22Z"></path>
        <path d="M19 11V13H5V11H19Z"></path>
        <path d="M16 17V19H8V17H16Z"></path>
      </svg>
      {#if filters.length > 0}
        <div class="absolute -top-1 -right-1 size-4 rounded-full text-[10px] font-bold flex items-center justify-center" style="background: var(--badge-bg); color: var(--badge-text);">{filters.length}</div>
      {/if}
    </button>

    {#if filterOpen}
      <div
        class="absolute right-0 top-full overflow-hidden rounded-md shadow-lg min-w-36 z-[9999]"
        style="background: var(--menu-bg);"
        onclick={(e) => e.stopPropagation()}
      >
        {#each filterOpts as opt}
          {@const active = filters.includes(opt)}
          <button
            onclick={() => toggleFilter(opt)}
            class="w-full px-4 py-2 text-left text-[15px] font-[565] flex items-center justify-between gap-3"
            style="background: {active ? 'var(--menu-item-selected-bg)' : ''}; color: var(--menu-item-text);"
            onmouseenter={(e) => e.currentTarget.style.background = active ? 'var(--menu-item-selected-bg)' : 'var(--menu-item-hover)'}
            onmouseleave={(e) => e.currentTarget.style.background = active ? 'var(--menu-item-selected-bg)' : ''}
          >
          {opt.charAt(0).toUpperCase() + opt.slice(1)}
            {#if active}
              <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="size-4 flex-shrink-0" fill="var(--menu-item-selected-icon)">
                <path d="M20.002 10L20.002 14L4.00003 14L4.00003 10L20.002 10Z"></path>
              </svg>
            {/if}
          </button>
        {/each}
      </div>
    {/if}
  </div>
</div>