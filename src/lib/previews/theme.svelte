<script>
  let { colors = {} } = $props();
  function g(k) { return colors[k] ?? getComputedStyle(document.documentElement).getPropertyValue(k).trim(); }
</script>

<div class="w-full h-full overflow-y-auto flex flex-col gap-4 p-2" style="scrollbar-width:thin;scrollbar-color:{g('--scrollbar-thumb')} transparent;background:{g('--prompt-bg')}">

  <!-- label -->
  <div class="text-[10px] font-bold px-1" style="color:{g('--themes-desc')}">EDITOR VIEW</div>

  <!-- SCREEN 1: EDITOR -->
  <div class="flex flex-col rounded-md overflow-hidden flex-shrink-0" style="height:280px;background:{g('--bg-app')}">
    <!-- topbar -->
    <div class="flex items-center gap-1.5 px-2 flex-shrink-0" style="height:24px;background:{g('--topbar-bg')}">
      <div class="w-3.5 h-3.5 rounded-sm grid grid-cols-2 gap-[1.5px] p-[2.5px] flex-shrink-0" style="background:{g('--topbar-icon')}20">
        {#each [0,1,2,3] as _}<div style="background:{g('--topbar-icon')}"></div>{/each}
      </div>
      <div class="flex flex-col gap-[2px] mr-1.5 flex-shrink-0">
        <div class="h-[3px] w-8 rounded-sm" style="background:{g('--topbar-menu-text')}70"></div>
        <div class="h-[2px] w-5 rounded-sm" style="background:{g('--topbar-menu-text')}35"></div>
      </div>
      {#each [28,24,24] as w}
        <div class="rounded-sm flex items-center justify-center" style="height:12px;width:{w}px;background:{g('--topbar-menu-bg')}">
          <div class="h-[2px] rounded-sm" style="width:{w-8}px;background:{g('--topbar-menu-text')}60"></div>
        </div>
      {/each}
      <div class="ml-auto flex gap-1">
        {#each [0,1,2] as _}<div class="w-3 h-3 rounded-sm" style="background:{g('--topbar-btn-bg')};border:1px solid {g('--topbar-icon')}15"></div>{/each}
      </div>
    </div>

    <div class="flex flex-1 overflow-hidden">
      <!-- leftbar -->
      <div class="flex flex-col items-center gap-1.5 py-2 flex-shrink-0" style="width:24px;background:{g('--leftbar-bg')}">
        <div class="w-4 h-4 rounded-sm grid grid-cols-2 gap-[2px] p-[3px]" style="background:linear-gradient(to bottom,{g('--leftbar-btn-active-from')},{g('--leftbar-btn-active-to')})">
          {#each [0,1,2,3] as _}<div class="rounded-[1px]" style="background:linear-gradient(to bottom,{g('--icon-leftbar-selected-top')},{g('--icon-leftbar-selected-bottom')})"></div>{/each}
        </div>
        {#each [0,1] as _}
          <div class="w-4 h-4 rounded-sm flex items-center justify-center" style="background:{g('--leftbar-btn-hover')}20">
            <div class="w-2.5 h-2.5 rounded-full" style="border:1.5px solid {g('--icon-leftbar-unselected')}"></div>
          </div>
        {/each}
        <div class="mt-auto flex flex-col items-center gap-1.5">
          <div class="w-4 h-4 rounded-sm flex items-center justify-center" style="background:{g('--leftbar-btn-hover')}20">
            <div class="w-2.5 h-2.5 rounded-sm" style="border:1.5px solid {g('--icon-leftbar-unselected')}"></div>
          </div>
          <div class="w-2 h-2 rounded-full" style="background:{g('--uninjected')}"></div>
        </div>
      </div>

      <!-- explorer -->
      <div class="flex flex-col flex-shrink-0" style="width:95px;background:{g('--explorer-bg')}">
        <div class="flex gap-1 px-1.5 pt-1.5 pb-1">
          <div class="flex-1 flex items-center gap-1 rounded-sm px-1" style="height:16px;background:{g('--search-input-bg')}">
            <div class="w-2 h-2 rounded-full flex-shrink-0" style="border:1.5px solid {g('--search-input-icon')}"></div>
            <div class="h-[2.5px] rounded-sm flex-1" style="background:{g('--search-input-placeholder')}"></div>
          </div>
          <div class="w-4 h-4 rounded-sm flex-shrink-0" style="background:{g('--search-opendir-bg')}"></div>
        </div>
        <div class="px-1.5 pb-1"><div class="h-[2.5px] w-10 rounded-sm" style="background:{g('--explorer-label')}"></div></div>
        <!-- expanded folder -->
        <div class="mx-1.5 rounded-sm px-1.5 flex items-center gap-1" style="height:18px;background:{g('--folder-expanded-bg')}">
          <div class="w-0 h-0 flex-shrink-0" style="border-left:3px solid transparent;border-right:3px solid transparent;border-top:4px solid {g('--scriptitem-icon')}"></div>
          <div class="w-3 h-2 rounded-sm flex-shrink-0" style="background:{g('--scriptitem-icon')}50"></div>
          <div class="h-[2.5px] rounded-sm flex-1" style="background:{g('--scriptitem-text')}"></div>
        </div>
        <!-- tree + files -->
        <div class="relative flex flex-col gap-0.5 mt-0.5" style="margin-left:16px;margin-right:6px">
          <div class="absolute left-[-5px] top-0 bottom-0 w-[2px] rounded-sm" style="background:{g('--explorer-tree-line')}"></div>
          {#each [0,1] as _}
            <div class="rounded-sm px-1.5 flex items-center gap-1" style="height:16px;background:{g('--scriptitem-bg')}">
              <div class="w-2 h-2 rounded-full flex-shrink-0" style="background:{g('--scriptitem-icon')}70"></div>
              <div class="h-[2.5px] rounded-sm flex-1" style="background:{g('--scriptitem-text')}60"></div>
            </div>
          {/each}
        </div>
        <!-- open / save -->
        <div class="mt-auto flex gap-1 px-1.5 py-1.5">
          {#each [[g('--open-bg'),g('--open-icon')],[g('--save-bg'),g('--save-icon')]] as [bg,ic]}
            <div class="flex-1 rounded-sm flex items-center justify-center gap-1" style="height:14px;background:{bg}">
              <div class="w-2 h-2 rounded-full flex-shrink-0" style="background:{ic}"></div>
              <div class="h-[2.5px] w-4 rounded-sm" style="background:{g('--open-text')}60"></div>
            </div>
          {/each}
        </div>
      </div>

      <!-- editor area -->
      <div class="flex-1 flex flex-col overflow-hidden">
        <!-- tabbar -->
        <div class="flex items-center px-1 gap-1 flex-shrink-0" style="height:20px;background:{g('--editor-tabbar-bg')}">
          <div class="rounded-sm px-1.5 flex items-center gap-1 flex-shrink-0" style="height:14px;background:{g('--tab-bg-selected')}">
            <div class="w-2 h-2 rounded-sm flex-shrink-0" style="background:{g('--tab-icon-selected')}"></div>
            <div class="h-[2.5px] w-10 rounded-sm" style="background:{g('--tab-text-selected')}"></div>
            <div class="w-1.5 h-1.5 rounded-full flex-shrink-0" style="border:1px solid {g('--tab-remove-icon')}70"></div>
          </div>
          <div class="ml-auto w-4 h-4 rounded-sm flex items-center justify-center" style="background:{g('--editor-newtab-bg')}">
            <div class="w-2 h-[1.5px]" style="background:{g('--editor-newtab-icon')}"></div>
          </div>
        </div>
        <!-- monaco -->
        <div class="flex flex-1 overflow-hidden" style="background:{g('--monaco-bg')}">
          <div class="flex flex-col gap-2 pt-2 px-1.5 flex-shrink-0 items-end" style="border-right:1px solid {g('--monaco-scrollbar')}30">
            {#each [0,1,2,3,4] as i}
              <div class="h-[2.5px] rounded-sm" style="width:{i===0?8:6}px;background:{i===0?g('--monaco-line-number-active'):g('--monaco-line-number')}"></div>
            {/each}
          </div>
          <div class="flex flex-col gap-2 pt-2 pl-2 flex-1">
            <div class="flex gap-1 items-center">
              <div class="h-[2.5px] w-8 rounded-sm" style="background:{g('--monaco-keyword')}"></div>
              <div class="h-[2.5px] w-3 rounded-sm" style="background:{g('--monaco-delimiter')}"></div>
              <div class="h-[2.5px] w-12 rounded-sm" style="background:{g('--monaco-string')}"></div>
            </div>
            <div class="flex gap-1 items-center">
              <div class="h-[2.5px] w-6 rounded-sm" style="background:{g('--monaco-identifier')}"></div>
              <div class="h-[2.5px] w-2 rounded-sm" style="background:{g('--monaco-delimiter')}"></div>
              <div class="h-[2.5px] w-8 rounded-sm" style="background:{g('--monaco-number')}"></div>
            </div>
            <div class="h-[2.5px] w-16 rounded-sm" style="background:{g('--monaco-comment')}"></div>
            <div class="flex gap-1 items-center">
              <div class="h-[2.5px] w-7 rounded-sm" style="background:{g('--monaco-keyword')}"></div>
              <div class="h-[2.5px] w-12 rounded-sm" style="background:{g('--monaco-identifier')}"></div>
            </div>
          </div>
        </div>
        <!-- toolbar: Execute + Inject only -->
        <div class="flex items-center gap-1 px-1.5 py-1.5 flex-shrink-0" style="background:{g('--editor-toolbar-bg')}">
          {#each [[g('--editor-execute-bg'),g('--editor-execute-icon')],[g('--editor-inject-bg'),g('--editor-inject-icon')]] as [bg,ic]}
            <div class="rounded-sm px-1.5 flex items-center gap-1 flex-shrink-0" style="height:15px;background:{bg}">
              <div class="w-2 h-2 rounded-full flex-shrink-0" style="background:{ic}"></div>
              <div class="h-[2.5px] w-6 rounded-sm" style="background:{g('--editor-execute-text')}"></div>
            </div>
          {/each}
          <div class="ml-auto w-3.5 h-3.5 rounded-sm flex items-center justify-center" style="background:{g('--editor-clear-bg')}">
            <div class="w-2 h-2 rounded-full" style="border:1px solid {g('--editor-clear-icon')}70"></div>
          </div>
        </div>
      </div>
    </div>
  </div>

  <!-- label -->
  <div class="text-[10px] font-bold px-1" style="color:{g('--themes-desc')}">SETTINGS VIEW</div>

  <!-- SCREEN 2: SETTINGS -->
  <div class="flex flex-col rounded-md overflow-hidden flex-shrink-0" style="height:280px;background:{g('--bg-app')}">
    <!-- topbar -->
    <div class="flex items-center gap-1.5 px-2 flex-shrink-0" style="height:24px;background:{g('--topbar-bg')}">
      <div class="w-3.5 h-3.5 rounded-sm grid grid-cols-2 gap-[1.5px] p-[2.5px] flex-shrink-0" style="background:{g('--topbar-icon')}20">
        {#each [0,1,2,3] as _}<div style="background:{g('--topbar-icon')}"></div>{/each}
      </div>
      <div class="flex flex-col gap-[2px] mr-1.5 flex-shrink-0">
        <div class="h-[3px] w-8 rounded-sm" style="background:{g('--topbar-menu-text')}70"></div>
        <div class="h-[2px] w-5 rounded-sm" style="background:{g('--topbar-menu-text')}35"></div>
      </div>
      {#each [28,24,24] as w}
        <div class="rounded-sm flex items-center justify-center" style="height:12px;width:{w}px;background:{g('--topbar-menu-bg')}">
          <div class="h-[2px] rounded-sm" style="width:{w-8}px;background:{g('--topbar-menu-text')}60"></div>
        </div>
      {/each}
      <div class="ml-auto flex gap-1">
        {#each [0,1,2] as _}<div class="w-3 h-3 rounded-sm" style="background:{g('--topbar-btn-bg')};border:1px solid {g('--topbar-icon')}15"></div>{/each}
      </div>
    </div>

    <div class="flex flex-1 overflow-hidden">
      <!-- leftbar: settings/gear active -->
      <div class="flex flex-col items-center gap-1.5 py-2 flex-shrink-0" style="width:24px;background:{g('--leftbar-bg')}">
        <div class="w-4 h-4 rounded-sm flex items-center justify-center" style="background:{g('--leftbar-btn-hover')}20">
          <div class="grid grid-cols-2 gap-[2px]" style="width:9px;height:9px">
            {#each [0,1,2,3] as _}<div class="rounded-[1px]" style="background:{g('--icon-leftbar-unselected')}"></div>{/each}
          </div>
        </div>
        <div class="w-4 h-4 rounded-sm flex items-center justify-center" style="background:{g('--leftbar-btn-hover')}20">
          <div class="w-2.5 h-2.5 rounded-full" style="border:1.5px solid {g('--icon-leftbar-unselected')}"></div>
        </div>
        <!-- ACTIVE gear -->
        <div class="w-4 h-4 rounded-sm grid grid-cols-2 gap-[2px] p-[3px]" style="background:linear-gradient(to bottom,{g('--leftbar-btn-active-from')},{g('--leftbar-btn-active-to')})">
          {#each [0,1,2,3] as _}<div class="rounded-[1px]" style="background:linear-gradient(to bottom,{g('--icon-leftbar-selected-top')},{g('--icon-leftbar-selected-bottom')})"></div>{/each}
        </div>
        <div class="mt-auto flex flex-col items-center gap-1.5">
          <div class="w-4 h-4 rounded-sm flex items-center justify-center" style="background:{g('--leftbar-btn-hover')}20">
            <div class="flex flex-col gap-[2px]">
              {#each [0,1,2] as _}<div class="h-[1.5px] w-2.5 rounded-sm" style="background:{g('--icon-leftbar-unselected')}"></div>{/each}
            </div>
          </div>
          <div class="w-2 h-2 rounded-full" style="background:{g('--uninjected')}"></div>
        </div>
      </div>

      <!-- settings sidebar -->
      <div class="flex flex-col flex-shrink-0 pt-2" style="width:90px;background:{g('--settings-sidebar-bg')}">
        <div class="flex gap-1 px-1.5 pb-1.5">
          <div class="flex-1 flex items-center gap-1 rounded-sm px-1" style="height:16px;background:{g('--search-bg')}">
            <div class="w-2 h-2 rounded-full flex-shrink-0" style="border:1.5px solid {g('--search-icon')}"></div>
            <div class="h-[2.5px] rounded-sm flex-1" style="background:{g('--search-text')}20"></div>
          </div>
          <div class="w-4 h-4 rounded-sm flex items-center justify-center flex-shrink-0" style="background:{g('--search-filter-btn-bg')}">
            <div class="flex flex-col gap-[2px] items-center">
              {#each [8,5,3] as w}
                <div class="h-[1.5px] rounded-sm" style="width:{w}px;background:{g('--search-filter-btn-icon')}"></div>
              {/each}
            </div>
          </div>
        </div>
        <div class="px-1.5 pb-1"><div class="h-[2.5px] w-10 rounded-sm" style="background:{g('--settings-sidebar-label')}"></div></div>
        <!-- INTERFACE active, others inactive -->
        {#each [true,false,false,false] as active}
          <div class="mx-1.5 mb-0.5 rounded-sm px-1.5 flex items-center gap-1" style="height:20px;background:{active?`linear-gradient(to bottom,${g('--settings-btn-selected-from')},${g('--settings-btn-selected-to')})`:'transparent'}">
            <div class="w-2.5 h-2.5 rounded-sm flex-shrink-0" style="background:{active?g('--settings-icon-selected-top'):g('--settings-icon-unselected')}35;border:1px solid {active?g('--settings-icon-selected-top'):g('--settings-icon-unselected')}50"></div>
            <div class="h-[2.5px] rounded-sm flex-1" style="background:{active?g('--settings-text-selected-top'):g('--settings-text-unselected')}"></div>
          </div>
        {/each}
      </div>

      <!-- settings content -->
      <div class="flex-1 flex flex-col overflow-hidden p-2" style="background:{g('--settings-content-bg')}">
        <div class="flex items-center gap-2 mb-2 flex-shrink-0">
          <div class="h-[3px] w-16 rounded-sm" style="background:{g('--interface-title')}"></div>
          <div class="h-[2.5px] w-20 rounded-sm" style="background:{g('--interface-desc')}"></div>
        </div>
        <!-- 3-col card grid -->
        <div class="grid grid-cols-3 gap-1.5">
          {#each ['checkbox','checkbox','dropdown','dropdown'] as type}
            <div class="rounded-md p-1.5 flex flex-col gap-1.5" style="background:{g('--item-bg')}">
              <div class="flex items-center gap-1">
                <div class="w-2.5 h-2.5 rounded-full flex-shrink-0" style="background:{g('--item-icon')}40"></div>
                <div class="h-[2.5px] rounded-sm flex-1" style="background:{g('--item-title')}"></div>
              </div>
              <div class="h-[2px] rounded-sm" style="background:{g('--item-desc')}50"></div>
              <div class="h-[2px] rounded-sm w-2/3" style="background:{g('--item-desc')}35"></div>
              <div class="flex items-center justify-between mt-0.5">
                <div class="w-2.5 h-2.5 rounded-sm" style="background:{g('--item-desc-icon')}25"></div>
                {#if type === 'checkbox'}
                  <div class="rounded-full" style="width:18px;height:9px;background:{g('--item-check-off')}"></div>
                {:else}
                  <div class="rounded-sm" style="width:24px;height:9px;background:{g('--item-dropdown-btn-bg')}"></div>
                {/if}
              </div>
            </div>
          {/each}
        </div>
      </div>
    </div>
  </div>

</div>