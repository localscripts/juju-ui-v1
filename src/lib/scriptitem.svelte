<script>
  let { icon, name, luafile = false, selected = false, expanded = false, renaming = false, onrename = () => {}, ondoubleclickrename = null, isDropTarget = false } = $props();
  let input;
  let nameOnly = $state('');
  let ext = $state('.lua');
  let extOpen = $state(false);

  $effect(() => {
    if (renaming) {
      const parts = name.split('.');
      if (parts.length > 1) {
        ext = '.' + parts.pop();
        nameOnly = parts.join('.');
      } else {
        nameOnly = name;
        ext = '.lua';
      }
      input?.focus();
      input?.select();
    }
  });

  function submit() {
    onrename(luafile ? nameOnly + ext : nameOnly);
  }

  function bg() {
    if (isDropTarget) return 'background: var(--scriptitem-bg-droptarget); color: var(--scriptitem-text-droptarget);';
    return 'background: var(--scriptitem-bg); color: var(--scriptitem-text);';
  }

  function enter(e) {
    if (isDropTarget) return;
    e.currentTarget.style.background = 'var(--scriptitem-bg-hover)';
  }

  function leave(e) {
    if (isDropTarget) return;
    e.currentTarget.style.background = 'var(--scriptitem-bg)';
  }
</script>

<svelte:window
  onclick={() => extOpen = false}
  oncontextmenu={() => extOpen = false}
/>

<div
  ondblclick={ondoubleclickrename}
  class="w-full p-2 rounded-md font-[565] flex gap-3 items-center cursor-default"
  style={bg()}
  onmouseenter={enter}
  onmouseleave={leave}
>
  {#if !luafile}
    <svg class={`w-4 h-4 flex-shrink-0 ${expanded ? "rotate-0" : "-rotate-90"}`} viewBox="0 0 10 6" fill="none">
      <path fill-rule="evenodd" clip-rule="evenodd" d="M8.92799 1.9133C9.02551 1.82705 9.1041 1.72351 9.15925 1.60858C9.21441 1.49365 9.24506 1.36958 9.24945 1.24346C9.25384 1.11734 9.23189 0.991638 9.18485 0.873531C9.13781 0.755424 9.06661 0.647225 8.9753 0.555111C8.88399 0.462997 8.77436 0.388773 8.65268 0.336676C8.531 0.284579 8.39965 0.25563 8.26613 0.251481C8.1326 0.247332 7.99952 0.268065 7.87448 0.312496C7.74944 0.356926 7.63488 0.424184 7.53736 0.51043L4.75 2.97218L1.96264 0.50947C1.86512 0.423224 1.75056 0.355965 1.62552 0.311534C1.50048 0.267104 1.3674 0.246371 1.23387 0.25052C1.10035 0.254669 0.968997 0.283618 0.847318 0.335715C0.725638 0.387812 0.616014 0.462036 0.524704 0.55415C0.433393 0.646264 0.362186 0.754463 0.315146 0.87257C0.268107 0.990677 0.246157 1.11638 0.250549 1.2425C0.254942 1.36862 0.285591 1.49269 0.340747 1.60762C0.395903 1.72255 0.474485 1.82609 0.572007 1.91234L4.05519 4.99097C4.24364 5.15741 4.492 5.25 4.75 5.25C5.008 5.25 5.25636 5.15741 5.44481 4.99097L8.92799 1.9133Z" fill="var(--scriptitem-icon)" stroke-width="0.5"/>
    </svg>
  {/if}

  {#if !luafile}
    <svg class="w-5.5 h-5.5 flex-shrink-0" viewBox="0 0 24 24" fill="none">
      <path d="M13.821 6.5h5.929a2.25 2.25 0 0 1 2.229 1.938l.016.158.005.154v9a2.25 2.25 0 0 1-2.096 2.245L19.75 20H4.25a2.25 2.25 0 0 1-2.245-2.096L2 17.75v-7.251l6.207.001.196-.009a2.25 2.25 0 0 0 1.088-.393l.156-.12L13.821 6.5ZM8.207 4c.46 0 .908.141 1.284.402l.156.12 2.103 1.751-3.063 2.553-.085.061a.75.75 0 0 1-.29.106L8.206 9 2 8.999V6.25a2.25 2.25 0 0 1 2.096-2.245L4.25 4h3.957Z" fill="var(--scriptitem-icon)"/>
    </svg>
  {:else if name.endsWith('.txt')}
    <svg class="w-4.5 h-4.5 flex-shrink-0" viewBox="0 0 14 14" fill="none">
      <path fill-rule="evenodd" clip-rule="evenodd" d="M0 0.75C0 0.551088 0.0790176 0.360322 0.21967 0.21967C0.360322 0.0790176 0.551088 0 0.75 0H13.25C13.4489 2.96403e-09 13.6397 0.0790176 13.7803 0.21967C13.921 0.360322 14 0.551088 14 0.75C14 0.948912 13.921 1.13968 13.7803 1.28033C13.6397 1.42098 13.4489 1.5 13.25 1.5H0.75C0.551088 1.5 0.360322 1.42098 0.21967 1.28033C0.0790176 1.13968 0 0.948912 0 0.75ZM0.75 3C0.551088 3 0.360322 3.07902 0.21967 3.21967C0.0790176 3.36032 0 3.55109 0 3.75C0 3.94891 0.0790176 4.13968 0.21967 4.28033C0.360322 4.42098 0.551088 4.5 0.75 4.5H10.75C10.9489 4.5 11.1397 4.42098 11.2803 4.28033C11.421 4.13968 11.5 3.94891 11.5 3.75C11.5 3.55109 11.421 3.36032 11.2803 3.21967C11.1397 3.07902 10.9489 3 10.75 3H0.75ZM0 6.75C0 6.55109 0.0790176 6.36032 0.21967 6.21967C0.360322 6.07902 0.551088 6 0.75 6H7.75C7.94891 6 8.13968 6.07902 8.28033 6.21967C8.42098 6.36032 8.5 6.55109 8.5 6.75C8.5 6.94891 8.42098 7.13968 8.28033 7.28033C8.13968 7.42098 7.94891 7.5 7.75 7.5H0.75C0.551088 7.5 0.360322 7.42098 0.21967 7.28033C0.0790176 7.13968 0 6.94891 0 6.75ZM0 9.75C0 9.55109 0.0790176 9.36032 0.21967 9.21967C0.360322 9.07902 0.551088 9 0.75 9H10.75C10.9489 9 11.1397 9.07902 11.2803 9.21967C11.421 9.36032 11.5 9.55109 11.5 9.75C11.5 9.94891 11.421 10.1397 11.2803 10.2803C11.1397 10.421 10.9489 10.5 10.75 10.5H0.75C0.551088 10.5 0.360322 10.421 0.21967 10.2803C0.0790176 10.1397 0 9.94891 0 9.75ZM0.75 12C0.551088 12 0.360322 12.079 0.21967 12.2197C0.0790176 12.3603 0 12.5511 0 12.75C0 12.9489 0.0790176 13.1397 0.21967 13.2803C0.360322 13.421 0.551088 13.5 0.75 13.5H13.25C13.4489 13.5 13.6397 13.421 13.7803 13.2803C13.921 13.1397 14 12.9489 14 12.75C14 12.5511 13.921 12.3603 13.7803 12.2197C13.6397 12.079 13.4489 12 13.25 12H0.75Z" fill="var(--scriptitem-icon)"/>
    </svg>
  {:else}
    <svg class="w-6 h-6 flex-shrink-0 -ml-0.75" viewBox="0 0 24 24" fill="var(--scriptitem-icon)">
      <path fill-rule="evenodd" clip-rule="evenodd" d="M10.8506 5.29993C15.1836 5.29993 18.7012 8.81752 18.7012 13.1505C18.7012 17.4835 15.1836 21.0011 10.8506 21.0011C6.51759 21.0011 3 17.4835 3 13.1505C3 8.81752 6.51759 5.29993 10.8506 5.29993ZM14.1036 7.59867C15.3723 7.59867 16.4024 8.62879 16.4024 9.89747C16.4024 11.1661 15.3723 12.1963 14.1036 12.1963C12.835 12.1963 11.8048 11.1661 11.8048 9.89747C11.8048 8.62879 12.835 7.59867 14.1036 7.59867Z"/>
      <path d="M21 5.2977C21 4.02902 19.9699 2.9989 18.7012 2.9989C17.4326 2.9989 16.4024 4.02902 16.4024 5.2977C16.4024 6.56637 17.4326 7.59649 18.7012 7.59649C19.9699 7.59649 21 6.56854 21 5.2977Z"/>
    </svg>
  {/if}

  {#if renaming}
    <div class="flex-1 flex items-center gap-1 min-w-0">
      <input
        bind:this={input}
        bind:value={nameOnly}
        class="flex-1 min-w-0 bg rounded outline-none px-1"
        style="color: var(--scriptitem-input-text); background: var(--scriptitem-input-bg);"
        onkeydown={(e) => {
          if (e.key === '.') e.preventDefault();
          if (e.key === 'Enter') submit();
          if (e.key === 'Escape') { e.preventDefault(); e.stopPropagation(); onrename(name); }
        }}
        onblur={(e) => { if (!e.relatedTarget?.closest('.ext-dropdown')) submit(); }}
        onclick={(e) => e.stopPropagation()}
      />
      {#if luafile}
        <div class="relative flex-shrink-0 ext-dropdown">
          <button
            onclick={(e) => { e.stopPropagation(); e.preventDefault(); extOpen = !extOpen; }}
            class="px-3 py-1 rounded text-sm"
            style="background: var(--scriptitem-ext-btn-bg); color: var(--scriptitem-ext-btn-text);"
            onmouseenter={(e) => e.currentTarget.style.background = 'var(--scriptitem-ext-btn-bg-hover)'}
            onmouseleave={(e) => e.currentTarget.style.background = 'var(--scriptitem-ext-btn-bg)'}
          >{ext}</button>
          {#if extOpen}
            <div class="absolute top-full right-0 mt-1 rounded shadow-lg z-50 overflow-hidden" style="background: var(--scriptitem-ext-dropdown-bg);" onclick={(e) => e.stopPropagation()}>
              <button onclick={(e) => { e.preventDefault(); ext = '.lua'; extOpen = false; input?.focus(); }} class="block w-full px-4 py-2 text-left text-sm" style="color: var(--scriptitem-ext-dropdown-text);" onmouseenter={(e) => e.currentTarget.style.background = 'var(--scriptitem-ext-dropdown-hover)'} onmouseleave={(e) => e.currentTarget.style.background = ''}>.lua</button>
              <button onclick={(e) => { e.preventDefault(); ext = '.txt'; extOpen = false; input?.focus(); }} class="block w-full px-4 py-2 text-left text-sm" style="color: var(--scriptitem-ext-dropdown-text);" onmouseenter={(e) => e.currentTarget.style.background = 'var(--scriptitem-ext-dropdown-hover)'} onmouseleave={(e) => e.currentTarget.style.background = ''}>.txt</button>
            </div>
          {/if}
        </div>
      {/if}
    </div>
  {:else}
    <span class={`truncate ${luafile && !name.endsWith('.txt') ? '-ml-1' : ''}`}>{name}</span>
  {/if}
</div>