<script>
let { title, desc, type = "checkbox", value = $bindable(false), options = [], onclick, onchange, onddopen, icon = "", iconVb = "0 0 24 24", min = 0, max = 100, step = 1, placeholder = "", scrollable = false } = $props();
  let ddOpen = $state(false);
  let pct = $derived(((value - min) / (max - min)) * 100);

  function openDD(e) {
    e.stopPropagation();
    const opening = !ddOpen;
    ddOpen = !ddOpen;
    if (opening && onddopen) onddopen();
  }
</script>

<svelte:window onclick={() => ddOpen = false} />

<div class="rounded-md flex flex-col" style="background: var(--item-bg); width: 210px; height: 170px;">

  <div class="flex-1 p-3 pb-2 min-h-0">
    <div class="flex items-center gap-2 mb-2">
      {#if icon}
        <svg class="size-5 flex-shrink-0" viewBox={iconVb} fill="var(--item-icon)">{@html icon}</svg>
      {/if}
      <span class="text-[15px] font-[565] truncate" style="color: var(--item-title);">{title}</span>
    </div>
    <p class="text-[15px] font-[565] line-clamp-3" style="color: var(--item-desc);">{desc}</p>
  </div>

  <div class="flex items-center justify-between p-3 pt-0">

    <svg xmlns="http://www.w3.org/2000/svg" class="h-[25px] w-[25px] flex-shrink-0" viewBox="0 0 24 24" fill="var(--item-desc-icon)">
      <path fill-rule="evenodd" clip-rule="evenodd" d="M21 2.25C21.4142 2.25 21.75 2.58579 21.75 3V21C21.75 21.4142 21.4142 21.75 21 21.75H3C2.58579 21.75 2.25 21.4142 2.25 21V3C2.25 2.58579 2.58579 2.25 3 2.25H21ZM10.75 9.5L17 9.5V7.5L10.75 7.5V6.25H6.25V10.75H10.75V9.5ZM17.75 13.25V17.75H13.25V16.5H7V14.5H13.25V13.25H17.75Z"></path>
    </svg>

    {#if type === "checkbox"}
      <button
        onclick={() => { value = !value; if (onchange) onchange(value); }}
        class="relative rounded-md flex items-center justify-center flex-shrink-0"
        style="width: 30px; height: 30px; background: {value ? 'var(--item-check-on)' : 'var(--item-check-off)'};"
        onmouseenter={(e) => { if (!value) e.currentTarget.style.background = 'var(--item-check-hover)'; }}
        onmouseleave={(e) => { e.currentTarget.style.background = value ? 'var(--item-check-on)' : 'var(--item-check-off)'; }}
      >
        {#if value}
          <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="size-5" fill="var(--item-check-icon)">
            <path d="M20.002 10L20.002 14L4.00003 14L4.00003 10L20.002 10Z"></path>
          </svg>
        {/if}
      </button>

    {:else if type === "button"}
      <button
        onclick={onclick}
        class="rounded-md text-[15px] font-[565] flex-shrink-0 px-4"
        style="height: 30px; background: var(--item-btn-bg); color: var(--item-btn-text);"
        onmouseenter={(e) => e.currentTarget.style.background = 'var(--item-btn-hover)'}
        onmouseleave={(e) => e.currentTarget.style.background = 'var(--item-btn-bg)'}
      >{value || "Run"}</button>

    {:else if type === "dropdown"}
      <div class="relative flex-shrink-0">
        <button
          onclick={openDD}
          class="rounded-md text-[15px] font-[565] px-4"
          style="height: 30px; background: var(--item-dropdown-btn-bg); color: var(--item-dropdown-btn-text);"
          onmouseenter={(e) => e.currentTarget.style.background = 'var(--item-dropdown-btn-hover)'}
          onmouseleave={(e) => e.currentTarget.style.background = 'var(--item-dropdown-btn-bg)'}
        >{value}</button>
        {#if ddOpen}
          <div
            class="absolute bottom-full right-0 overflow-hidden rounded-md shadow-lg min-w-28 z-[9999]"
            style="background: var(--menu-bg); {scrollable ? 'max-height: 200px; overflow-y: auto; scrollbar-width: thin; scrollbar-color: var(--scrollbar-thumb) transparent;' : ''}"
            onclick={(e) => e.stopPropagation()}
          >
            {#each options as opt}
              {@const sel = opt === value}
              <button
                onclick={() => { value = opt; ddOpen = false; if (onchange) onchange(opt); }}
                class="w-full px-4 py-2 text-left text-[15px] font-[565] flex items-center justify-between gap-3"
                style="background: {sel ? 'var(--menu-item-selected-bg)' : ''}; color: var(--menu-item-text);"
                onmouseenter={(e) => e.currentTarget.style.background = sel ? 'var(--menu-item-selected-bg)' : 'var(--menu-item-hover)'}
                onmouseleave={(e) => e.currentTarget.style.background = sel ? 'var(--menu-item-selected-bg)' : ''}
              >
                {opt}
                {#if sel}
                  <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="size-4 flex-shrink-0" fill="var(--menu-item-selected-icon)">
                    <path d="M20.002 10L20.002 14L4.00003 14L4.00003 10L20.002 10Z"></path>
                  </svg>
                {/if}
              </button>
            {/each}
          </div>
        {/if}
      </div>

    {:else if type === "textbox"}
      <input
        bind:value
        {placeholder}
        class="rounded-md text-[15px] font-[565] outline-none px-2 flex-shrink-0"
        style="width: 110px; height: 30px; background: var(--item-textbox-bg); color: var(--item-textbox-text);"
        onmouseenter={(e) => e.currentTarget.style.background = 'var(--item-textbox-bg-hover)'}
        onmouseleave={(e) => { if (document.activeElement !== e.currentTarget) e.currentTarget.style.background = 'var(--item-textbox-bg)'; }}
        onfocus={(e) => e.currentTarget.style.background = 'var(--item-textbox-bg-hover)'}
        onblur={(e) => e.currentTarget.style.background = 'var(--item-textbox-bg)'}
      />

    {:else if type === "slider"}
      <div class="flex items-center gap-2 flex-shrink-0">
        <span class="text-[13px] font-[565] w-6 text-right tabular-nums" style="color: var(--item-slider-value);">{value}</span>
        <div class="relative flex items-center flex-shrink-0" style="width: 90px; height: 30px;">
          <div class="absolute w-full" style="height: 6px; background: var(--item-slider-track); border-radius: 2px; overflow: hidden;">
            <div class="h-full" style="width: {pct}%; background: var(--item-slider-fill); border-radius: 2px;"></div>
          </div>
          <div
            class="absolute pointer-events-none"
            style="width: 12px; height: 12px; border-radius: 2px; background: var(--item-slider-thumb); left: calc({pct}% - {pct * 0.12}px);"
          ></div>
          <input
            type="range"
            bind:value
            {min} {max} {step}
            class="absolute w-full cursor-pointer opacity-0"
            style="height: 30px;"
          />
        </div>
      </div>
    {/if}

  </div>

</div>

<style>
  input:not([type="range"])::placeholder {
    color: var(--item-textbox-placeholder);
  }
</style>