<script>
  let { name, cssKey, cssKey2 = null, first = false, getPending, setPending, onchange = () => {} } = $props();

  const icon = `<path fill-rule="evenodd" clip-rule="evenodd" d="M10.19 1.74902H1.74884V10.1906L10.19 1.74902ZM1.74884 12.3119V15.6917L15.6901 1.74902H12.3113L1.74884 12.3119ZM17.8113 1.74902L1.74884 17.8131V21.1917L21.1901 1.74902H17.8113ZM22.2488 2.81159L2.81285 22.249H6.1916L22.2488 6.19021V2.81159ZM22.2488 8.31164L8.31282 22.249H11.6916L22.2488 11.6902V8.31164ZM22.2488 13.8117L13.8127 22.249H22.2488V13.8117Z"/>`;

  function get(k) {
    return getPending()[k] ?? getComputedStyle(document.documentElement).getPropertyValue(k).trim();
  }

  function hex6(val) {
    if (!val) return '#000000';
    const v = val.trim();
    if (/^#[0-9a-fA-F]{6}$/.test(v)) return v;
    if (/^#[0-9a-fA-F]{8}$/.test(v)) return v.slice(0, 7);
    return '#000000';
  }

  function clear(val) {
    if (!val) return true;
    const v = val.trim().toLowerCase();
    return v === 'transparent' || v === '#00000000' || v === '';
  }

  let v1 = $state(get(cssKey));
  let v2 = $state(cssKey2 ? get(cssKey2) : '');

  function ch1(e) { v1 = e.currentTarget.value; setPending(cssKey, v1); onchange(); }
  function ch2(e) { v2 = e.currentTarget.value; setPending(cssKey2, v2); onchange(); }
</script>

<div class="flex items-center justify-between px-4 gap-4" style="height:46px;{first?'':'border-top:1px solid #ffffff06'}">
  <div class="flex items-center gap-3 min-w-0">
    <svg class="flex-shrink-0" style="width:18px;height:18px" viewBox="0 0 24 24" fill="var(--theme-item-icon)">{@html icon}</svg>
    <span class="font-[565] text-[15px] truncate" style="color:var(--theme-item-title)">{name}</span>
  </div>
  <div class="flex items-center gap-3 flex-shrink-0">
    {#if cssKey2}
      {#each [[v1,ch1,'Top'],[v2,ch2,'Bottom']] as [val,fn,lbl]}
        <div class="flex items-center gap-2">
          <span class="text-[15px] font-[565]" style="color:var(--theme-item-label)">{lbl}</span>
          <label class="cursor-pointer relative flex-shrink-0" style="width:28px;height:28px">
            {#if clear(val)}
              <div class="w-full h-full rounded-sm" style="background:repeating-conic-gradient(#ffffff10 0% 25%,transparent 0% 50%) 0 0/8px 8px;outline:1px solid var(--theme-item-swatch-outline)"></div>
            {:else}
              <div class="w-full h-full rounded-sm" style="background:{val};outline:1px solid var(--theme-item-swatch-outline)"></div>
            {/if}
            <input type="color" value={hex6(val)} onchange={fn} class="absolute inset-0 opacity-0 w-full h-full cursor-pointer" />
          </label>
        </div>
      {/each}
    {:else}
      <span class="text-[15px] font-[565] font-mono tabular-nums" style="color:var(--theme-item-hex)">{clear(v1)?'transparent':v1}</span>
      <label class="cursor-pointer relative flex-shrink-0" style="width:28px;height:28px">
        {#if clear(v1)}
          <div class="w-full h-full rounded-sm" style="background:repeating-conic-gradient(#ffffff10 0% 25%,transparent 0% 50%) 0 0/8px 8px;outline:1px solid var(--theme-item-swatch-outline)"></div>
        {:else}
          <div class="w-full h-full rounded-sm" style="background:{v1};outline:1px solid var(--theme-item-swatch-outline)"></div>
        {/if}
        <input type="color" value={hex6(v1)} onchange={ch1} class="absolute inset-0 opacity-0 w-full h-full cursor-pointer" />
      </label>
    {/if}
  </div>
</div>