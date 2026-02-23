<script>
  let { colors = {} } = $props();
  function g(k) { return colors[k] ?? getComputedStyle(document.documentElement).getPropertyValue(k).trim(); }

  const chevronPath = `M8.92799 1.9133C9.02551 1.82705 9.1041 1.72351 9.15925 1.60858C9.21441 1.49365 9.24506 1.36958 9.24945 1.24346C9.25384 1.11734 9.23189 0.991638 9.18485 0.873531C9.13781 0.755424 9.06661 0.647225 8.9753 0.555111C8.88399 0.462997 8.77436 0.388773 8.65268 0.336676C8.531 0.284579 8.39965 0.25563 8.26613 0.251481C8.1326 0.247332 7.99952 0.268065 7.87448 0.312496C7.74944 0.356926 7.63488 0.424184 7.53736 0.51043L4.75 2.97218L1.96264 0.50947C1.86512 0.423224 1.75056 0.355965 1.62552 0.311534C1.50048 0.267104 1.3674 0.246371 1.23387 0.25052C1.10035 0.254669 0.968997 0.283618 0.847318 0.335715C0.725638 0.387812 0.616014 0.462036 0.524704 0.55415C0.433393 0.646264 0.362186 0.754463 0.315146 0.87257C0.268107 0.990677 0.246157 1.11638 0.250549 1.2425C0.254942 1.36862 0.285591 1.49269 0.340747 1.60762C0.395903 1.72255 0.474485 1.82609 0.572007 1.91234L4.05519 4.99097C4.24364 5.15741 4.492 5.25 4.75 5.25C5.008 5.25 5.25636 5.15741 5.44481 4.99097L8.92799 1.9133Z`;

  let hoverTab = $state(-1);
</script>

<div class="w-full h-full flex flex-col overflow-hidden" style="background:{g('--bg-app')}">

  <div class="flex items-center gap-2 px-2 flex-shrink-0" style="height:32px;background:{g('--topbar-bg')}">
    <div class="rounded-full flex-shrink-0" style="width:12px;height:12px;background:#fff"></div>
    {#each [22,16,18] as w}
      <div class="rounded-sm flex-shrink-0" style="width:{w}px;height:3px;background:{g('--topbar-menu-text')}90"></div>
    {/each}
    <div class="ml-auto flex gap-1.5 flex-shrink-0">
      {#each [0,1,2] as _}
        <div class="rounded-full" style="width:10px;height:10px;background:{g('--topbar-icon')}60"></div>
      {/each}
    </div>
  </div>

  <div class="flex flex-1 overflow-hidden">

    <!-- LEFTBAR: 3rd item active (settings tab) -->
    <div class="flex flex-col items-center gap-1.5 py-2 flex-shrink-0" style="width:32px;background:{g('--leftbar-bg')}">
      {#each [0,1] as _}
        <div class="rounded-md flex items-center justify-center" style="width:24px;height:24px">
          <div class="rounded-full" style="width:10px;height:10px;background:{g('--icon-leftbar-unselected')}"></div>
        </div>
      {/each}
      <!-- active -->
      <div class="rounded-md flex items-center justify-center" style="width:24px;height:24px;background:linear-gradient(to bottom,{g('--leftbar-btn-active-from')},{g('--leftbar-btn-active-to')})">
        <div class="rounded-full" style="width:10px;height:10px;background:linear-gradient(to bottom,{g('--icon-leftbar-selected-top')},{g('--icon-leftbar-selected-bottom')})"></div>
      </div>
      <div class="mt-auto flex flex-col items-center gap-1.5">
        <div class="rounded-md flex items-center justify-center" style="width:24px;height:24px">
          <div class="rounded-full" style="width:10px;height:10px;background:{g('--icon-leftbar-unselected')}"></div>
        </div>
        <div class="rounded-full" style="width:7px;height:7px;background:{g('--uninjected')}"></div>
      </div>
    </div>

    <!-- SETTINGS SIDEBAR -->
    <div class="flex flex-col flex-shrink-0 pt-1.5" style="width:125px;background:{g('--settings-sidebar-bg')}">
      <!-- search + filter -->
      <div class="flex gap-1 px-2 pb-1 flex-shrink-0">
        <div class="flex-1 flex items-center gap-1 rounded-md px-1.5" style="height:20px;background:{g('--search-bg')}">
          <div class="rounded-full flex-shrink-0" style="width:7px;height:7px;background:{g('--search-icon')}"></div>
          <div class="rounded-sm flex-1" style="height:2.5px;background:{g('--search-text')}30"></div>
        </div>
        <div class="rounded-md flex items-center justify-center flex-shrink-0" style="width:20px;height:20px;background:{g('--search-filter-btn-bg')}">
          <div class="rounded-full" style="width:7px;height:7px;background:{g('--search-filter-btn-icon')}"></div>
        </div>
      </div>
      <!-- SETTINGS label -->
      <div class="px-2 pb-1 flex-shrink-0">
        <div class="rounded-sm" style="width:38px;height:2.5px;background:{g('--settings-sidebar-label')}"></div>
      </div>
      <!-- 4 tabs: INTERFACE (index 0) active, rest hover-able -->
      <!-- Active icon/text use gradient via SVG linearGradient trick on a rect -->
      {#each [true,false,false,false] as active, i}
        <div class="mx-2 mb-0.5 rounded-md px-1.5 flex items-center gap-1.5 flex-shrink-0 cursor-pointer"
          style="height:22px;background:{active?`linear-gradient(to bottom,${g('--settings-btn-selected-from')},${g('--settings-btn-selected-to')})`:(hoverTab===i?g('--settings-sidebar-item-hover'):'transparent')}"
          onmouseenter={() => { if (!active) hoverTab = i; }}
          onmouseleave={() => { if (!active) hoverTab = -1; }}>
          {#if active}
            <!-- active icon: gradient circle via SVG -->
            <svg style="width:10px;height:10px;flex-shrink:0" viewBox="0 0 10 10">
              <defs>
                <linearGradient id="ps_icon_g" x1="0" y1="0" x2="0" y2="10" gradientUnits="userSpaceOnUse">
                  <stop offset="0%" stop-color={g('--settings-icon-selected-top')}/>
                  <stop offset="100%" stop-color={g('--settings-icon-selected-bottom')}/>
                </linearGradient>
              </defs>
              <circle cx="5" cy="5" r="5" fill="url(#ps_icon_g)"/>
            </svg>
            <!-- active text: gradient line via SVG -->
            <svg style="flex:1;height:3px" viewBox="0 0 60 3" preserveAspectRatio="none">
              <defs>
                <linearGradient id="ps_text_g" x1="0" y1="0" x2="0" y2="3" gradientUnits="userSpaceOnUse">
                  <stop offset="0%" stop-color={g('--settings-text-selected-top')}/>
                  <stop offset="100%" stop-color={g('--settings-text-selected-bottom')}/>
                </linearGradient>
              </defs>
              <rect width="60" height="3" rx="1.5" fill="url(#ps_text_g)"/>
            </svg>
          {:else}
            <div class="rounded-full flex-shrink-0" style="width:10px;height:10px;background:{g('--settings-icon-unselected')}"></div>
            <div class="rounded-sm flex-1" style="height:2.5px;background:{g('--settings-text-unselected')}"></div>
          {/if}
        </div>
      {/each}
    </div>

    <!-- SETTINGS CONTENT -->
    <div class="flex-1 flex flex-col overflow-hidden p-2" style="background:{g('--settings-content-bg')}">
      <!-- title row -->
      <div class="flex items-center gap-2 mb-2 flex-shrink-0">
        <div class="rounded-sm" style="width:55px;height:2.5px;background:{g('--interface-title')}"></div>
        <div class="rounded-sm" style="width:75px;height:2.5px;background:{g('--interface-desc')}"></div>
      </div>
      <!-- item cards: 3 col -->
      <div class="grid grid-cols-3 gap-1.5 flex-shrink-0">
        {#each ['checkbox','checkbox','dropdown','dropdown'] as type}
          <div class="rounded-md p-1.5 flex flex-col gap-1.5" style="background:{g('--item-bg')}">
            <div class="flex items-center gap-1">
              <div class="rounded-full flex-shrink-0" style="width:7px;height:7px;background:{g('--item-icon')}"></div>
              <div class="rounded-sm flex-1" style="height:2px;background:{g('--item-title')}"></div>
            </div>
            <div class="rounded-sm" style="height:2px;background:{g('--item-desc')}70"></div>
            <div class="rounded-sm" style="width:65%;height:2px;background:{g('--item-desc')}50"></div>
            <div class="flex items-center justify-between mt-0.5">
              <div class="rounded-full" style="width:7px;height:7px;background:{g('--item-desc-icon')}40"></div>
              {#if type === 'checkbox'}
                <div class="rounded-full flex items-center" style="width:20px;height:9px;background:{g('--item-check-off')};padding:1px">
                  <div class="rounded-full ml-auto" style="width:6px;height:6px;background:{g('--item-check-mark')}60"></div>
                </div>
              {:else}
                <div class="rounded-md flex items-center px-1" style="height:9px;min-width:24px;background:{g('--item-dropdown-btn-bg')}">
                  <div class="rounded-sm flex-1" style="height:2px;background:{g('--item-dropdown-btn-text')}70"></div>
                </div>
              {/if}
            </div>
          </div>
        {/each}
      </div>
    </div>

  </div>
</div>