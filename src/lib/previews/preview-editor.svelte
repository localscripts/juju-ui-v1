<script>
  let { colors = {} } = $props();
  function g(k) { return colors[k] ?? getComputedStyle(document.documentElement).getPropertyValue(k).trim(); }

  const chevronPath = `M8.92799 1.9133C9.02551 1.82705 9.1041 1.72351 9.15925 1.60858C9.21441 1.49365 9.24506 1.36958 9.24945 1.24346C9.25384 1.11734 9.23189 0.991638 9.18485 0.873531C9.13781 0.755424 9.06661 0.647225 8.9753 0.555111C8.88399 0.462997 8.77436 0.388773 8.65268 0.336676C8.531 0.284579 8.39965 0.25563 8.26613 0.251481C8.1326 0.247332 7.99952 0.268065 7.87448 0.312496C7.74944 0.356926 7.63488 0.424184 7.53736 0.51043L4.75 2.97218L1.96264 0.50947C1.86512 0.423224 1.75056 0.355965 1.62552 0.311534C1.50048 0.267104 1.3674 0.246371 1.23387 0.25052C1.10035 0.254669 0.968997 0.283618 0.847318 0.335715C0.725638 0.387812 0.616014 0.462036 0.524704 0.55415C0.433393 0.646264 0.362186 0.754463 0.315146 0.87257C0.268107 0.990677 0.246157 1.11638 0.250549 1.2425C0.254942 1.36862 0.285591 1.49269 0.340747 1.60762C0.395903 1.72255 0.474485 1.82609 0.572007 1.91234L4.05519 4.99097C4.24364 5.15741 4.492 5.25 4.75 5.25C5.008 5.25 5.25636 5.15741 5.44481 4.99097L8.92799 1.9133Z`;

  // hover state for explorer items
  let hoverFolder = $state(false);
  let hoverFile1 = $state(false);
  let hoverFile2 = $state(false);
</script>

<div class="w-full h-full flex flex-col overflow-hidden" style="background:{g('--bg-app')}">

  <!-- TOPBAR: circle icon | File View Help (lines) | window circles -->
  <div class="flex items-center gap-2 px-2 flex-shrink-0" style="height:32px;background:{g('--topbar-bg')}">
    <!-- logo: single filled circle -->
    <div class="rounded-full flex-shrink-0" style="width:12px;height:12px;background:#fff"></div>
    <!-- File View Help: 3 lines (no bg pill, just lines) -->
    {#each [22,16,18] as w}
      <div class="rounded-sm flex-shrink-0" style="width:{w}px;height:3px;background:{g('--topbar-menu-text')}90"></div>
    {/each}
    <!-- window buttons: 3 circles far right -->
    <div class="ml-auto flex gap-1.5 flex-shrink-0">
      {#each [0,1,2] as _}
        <div class="rounded-full" style="width:10px;height:10px;background:{g('--topbar-icon')}60"></div>
      {/each}
    </div>
  </div>

  <div class="flex flex-1 overflow-hidden">

    <!-- LEFTBAR -->
    <div class="flex flex-col items-center gap-1.5 py-2 flex-shrink-0" style="width:32px;background:{g('--leftbar-bg')}">
      <!-- active: gradient bg + gradient circle -->
      <div class="rounded-md flex items-center justify-center" style="width:24px;height:24px;background:linear-gradient(to bottom,{g('--leftbar-btn-active-from')},{g('--leftbar-btn-active-to')})">
        <div class="rounded-full" style="width:10px;height:10px;background:linear-gradient(to bottom,{g('--icon-leftbar-selected-top')},{g('--icon-leftbar-selected-bottom')})"></div>
      </div>
      <!-- 2 inactive -->
      {#each [0,1] as _}
        <div class="rounded-md flex items-center justify-center" style="width:24px;height:24px">
          <div class="rounded-full" style="width:10px;height:10px;background:{g('--icon-leftbar-unselected')}"></div>
        </div>
      {/each}
      <!-- bottom: inactive + indicator -->
      <div class="mt-auto flex flex-col items-center gap-1.5">
        <div class="rounded-md flex items-center justify-center" style="width:24px;height:24px">
          <div class="rounded-full" style="width:10px;height:10px;background:{g('--icon-leftbar-unselected')}"></div>
        </div>
        <div class="rounded-full" style="width:7px;height:7px;background:{g('--uninjected')}"></div>
      </div>
    </div>

    <!-- EXPLORER -->
    <div class="flex flex-col flex-shrink-0" style="width:145px;background:{g('--explorer-bg')}">

      <!-- search + opendir -->
      <div class="flex gap-1 px-2 pt-1.5 pb-1 flex-shrink-0">
        <div class="flex-1 flex items-center gap-1 rounded-md px-1.5" style="height:20px;background:{g('--search-input-bg')}">
          <div class="rounded-full flex-shrink-0" style="width:7px;height:7px;background:{g('--search-input-icon')}"></div>
          <div class="rounded-sm flex-1" style="height:2.5px;background:{g('--search-input-placeholder')}"></div>
        </div>
        <div class="rounded-md flex items-center justify-center flex-shrink-0" style="width:20px;height:20px;background:{g('--search-opendir-bg')}">
          <div class="rounded-full" style="width:7px;height:7px;background:{g('--search-opendir-icon')}"></div>
        </div>
      </div>

      <!-- EXPLORER label -->
      <div class="px-2 pb-1 flex-shrink-0">
        <div class="rounded-sm" style="width:40px;height:2.5px;background:{g('--explorer-label')}"></div>
      </div>

      <!-- Folder row (like scriptitem: chevron + folder-circle + text line) -->
      <!-- hover shows scriptitem-bg-hover -->
      <div class="mx-2 mb-0.5 rounded-md px-1.5 flex items-center gap-1.5 flex-shrink-0 cursor-pointer"
        style="height:22px;background:{hoverFolder?g('--scriptitem-bg-hover'):g('--folder-expanded-bg')}"
        onmouseenter={() => hoverFolder = true}
        onmouseleave={() => hoverFolder = false}>
        <!-- chevron (expanded = pointing down) -->
        <svg style="width:8px;height:6px;flex-shrink:0" viewBox="0 0 10 6" fill="none">
          <path fill-rule="evenodd" clip-rule="evenodd" d={chevronPath} fill={g('--scriptitem-icon')}/>
        </svg>
        <!-- folder icon: filled circle (like the folder SVG in scriptitem, simplified) -->
        <div class="rounded-full flex-shrink-0" style="width:9px;height:9px;background:{g('--scriptitem-icon')}"></div>
        <!-- name line -->
        <div class="rounded-sm flex-1" style="height:2.5px;background:{g('--scriptitem-text')}"></div>
      </div>

      <!-- Files (indented, with tree line) -->
      <!-- files use lua icon (circle) from scriptitem, no chevron -->
      <div class="relative flex flex-col gap-0.5 flex-shrink-0" style="margin-left:16px;margin-right:8px">
        <div class="absolute rounded-sm" style="left:-2px;top:0;bottom:0;width:2px;background:{g('--explorer-tree-line')}"></div>
        <div class="rounded-md px-1.5 flex items-center gap-1.5 cursor-pointer"
          style="height:20px;background:{hoverFile1?g('--scriptitem-bg-hover'):g('--scriptitem-bg')}"
          onmouseenter={() => hoverFile1 = true}
          onmouseleave={() => hoverFile1 = false}>
          <!-- lua file: filled circle (no chevron) -->
          <div class="rounded-full flex-shrink-0" style="width:8px;height:8px;background:{g('--scriptitem-icon')}"></div>
          <div class="rounded-sm flex-1" style="height:2.5px;background:{g('--scriptitem-text')}80"></div>
        </div>
        <div class="rounded-md px-1.5 flex items-center gap-1.5 cursor-pointer"
          style="height:20px;background:{hoverFile2?g('--scriptitem-bg-hover'):g('--scriptitem-bg')}"
          onmouseenter={() => hoverFile2 = true}
          onmouseleave={() => hoverFile2 = false}>
          <div class="rounded-full flex-shrink-0" style="width:8px;height:8px;background:{g('--scriptitem-icon')}"></div>
          <div class="rounded-sm flex-1" style="height:2.5px;background:{g('--scriptitem-text')}80"></div>
        </div>
      </div>

      <!-- Bottom: Open + Save — same style as Execute/Inject (circle + line, same height) -->
      <div class="mt-auto flex gap-1.5 px-2 py-1.5 flex-shrink-0">
        {#each [[g('--open-bg'),g('--open-hover'),g('--open-icon'),g('--open-text')],[g('--save-bg'),g('--save-hover'),g('--save-icon'),g('--save-text')]] as [bg,hov,ic,tx]}
          <div class="flex-1 rounded-md px-1.5 flex items-center gap-1.5 cursor-pointer"
            style="height:22px;background:{bg}"
            onmouseenter={(e)=>e.currentTarget.style.background=hov}
            onmouseleave={(e)=>e.currentTarget.style.background=bg}>
            <div class="rounded-full flex-shrink-0" style="width:7px;height:7px;background:{ic}"></div>
            <div class="rounded-sm flex-1" style="height:2.5px;background:{tx}80"></div>
          </div>
        {/each}
      </div>
    </div>

    <!-- EDITOR -->
    <div class="flex-1 flex flex-col overflow-hidden">

      <!-- TABBAR -->
      <div class="flex items-center px-2 gap-1 flex-shrink-0" style="height:26px;background:{g('--editor-tabbar-bg')}">
        <div class="rounded-md px-1.5 flex items-center gap-1 flex-shrink-0" style="height:18px;background:{g('--tab-bg-selected')}">
          <div class="rounded-full flex-shrink-0" style="width:7px;height:7px;background:{g('--tab-icon-selected')}"></div>
          <div class="rounded-sm" style="width:32px;height:2.5px;background:{g('--tab-text-selected')}"></div>
          <div class="rounded-full flex-shrink-0" style="width:6px;height:6px;background:{g('--tab-remove-icon')}80"></div>
        </div>
        <div class="rounded-md px-1.5 flex items-center gap-1 flex-shrink-0" style="height:18px;background:{g('--tab-bg-unselected')}">
          <div class="rounded-full flex-shrink-0" style="width:7px;height:7px;background:{g('--tab-icon')}"></div>
          <div class="rounded-sm" style="width:26px;height:2.5px;background:{g('--tab-text-unselected')}"></div>
          <div class="rounded-full flex-shrink-0" style="width:6px;height:6px;background:{g('--tab-remove-icon')}40"></div>
        </div>
        <div class="ml-auto rounded-md flex items-center justify-center flex-shrink-0" style="width:18px;height:18px;background:{g('--editor-newtab-bg')}">
          <div class="rounded-sm" style="width:8px;height:2px;background:{g('--editor-newtab-icon')}"></div>
        </div>
      </div>

      <!-- MONACO -->
      <div class="flex flex-1 overflow-hidden" style="background:{g('--monaco-bg')}">
        <div class="flex flex-col gap-2 pt-2 px-1.5 flex-shrink-0 items-end" style="border-right:1px solid {g('--monaco-scrollbar')}60">
          {#each [0,1,2,3,4,5] as i}
            <div class="rounded-sm" style="width:{i===0?9:6}px;height:2px;background:{i===0?g('--monaco-line-number-active'):g('--monaco-line-number')}"></div>
          {/each}
        </div>
        <div class="flex flex-col gap-2 pt-2 pl-2 flex-1 overflow-hidden">
          <div class="flex gap-1 items-center">
            <div class="rounded-sm" style="height:2px;width:28px;background:{g('--monaco-keyword')}"></div>
            <div class="rounded-sm" style="height:2px;width:5px;background:{g('--monaco-delimiter')}"></div>
            <div class="rounded-sm" style="height:2px;width:48px;background:{g('--monaco-string')}"></div>
          </div>
          <div class="flex gap-1 items-center">
            <div class="rounded-sm" style="height:2px;width:20px;background:{g('--monaco-identifier')}"></div>
            <div class="rounded-sm" style="height:2px;width:4px;background:{g('--monaco-delimiter')}"></div>
            <div class="rounded-sm" style="height:2px;width:16px;background:{g('--monaco-number')}"></div>
          </div>
          <div class="rounded-sm" style="height:2px;width:65px;background:{g('--monaco-comment')}"></div>
          <div class="flex gap-1 items-center">
            <div class="rounded-sm" style="height:2px;width:26px;background:{g('--monaco-keyword')}"></div>
            <div class="rounded-sm" style="height:2px;width:40px;background:{g('--monaco-identifier')}"></div>
          </div>
          <div class="rounded-sm" style="height:2px;width:55px;background:{g('--monaco-selection')}"></div>
          <div class="flex gap-1 items-center">
            <div class="rounded-sm" style="height:2px;width:12px;background:{g('--monaco-identifier')}"></div>
            <div class="rounded-sm" style="height:2px;width:35px;background:{g('--monaco-string')}"></div>
          </div>
        </div>
      </div>

      <!-- TOOLBAR: Execute | Inject | [ml-auto] Clear -->
      <div class="flex items-center gap-1.5 px-2 py-1.5 flex-shrink-0" style="background:{g('--editor-toolbar-bg')}">
        <div class="rounded-md px-1.5 flex items-center gap-1 flex-shrink-0" style="height:22px;background:{g('--editor-execute-bg')}">
          <div class="rounded-full flex-shrink-0" style="width:7px;height:7px;background:{g('--editor-execute-icon')}"></div>
          <div class="rounded-sm" style="width:26px;height:2px;background:{g('--editor-execute-text')}"></div>
        </div>
        <div class="rounded-md px-1.5 flex items-center gap-1 flex-shrink-0" style="height:22px;background:{g('--editor-inject-bg')}">
          <div class="rounded-full flex-shrink-0" style="width:7px;height:7px;background:{g('--editor-inject-icon')}"></div>
          <div class="rounded-sm" style="width:22px;height:2px;background:{g('--editor-inject-text')}"></div>
        </div>
        <div class="ml-auto rounded-md flex items-center justify-center flex-shrink-0" style="width:22px;height:22px;background:{g('--editor-clear-bg')}">
          <div class="rounded-full" style="width:7px;height:7px;background:{g('--editor-clear-icon')}"></div>
        </div>
      </div>

    </div>
  </div>
</div>