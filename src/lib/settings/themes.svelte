<script>
  import ThemeItem from './theme.svelte';
  import { invoke } from '@tauri-apps/api/core';

  let {
    query = "", filters = [],
    onpreview = () => {}, onimport = () => {}, onexport = () => {}, onreset = () => {},
    bindreset = () => {}, bindimport = () => {}
  } = $props();

  let changes = $state(0);
  let pending = $state({});
  let resetKey = $state(0);
  let scrollEl = $state(null);
  let sticky = $state('');
  let sectionEls = {};

  function getPending() { return pending; }
  function setPending(k, v) { pending = { ...pending, [k]: v }; }
  function dom(k) { return getComputedStyle(document.documentElement).getPropertyValue(k).trim(); }

  // register callbacks so +page.svelte can trigger these after confirm
  $effect(() => {
    bindreset(() => { pending = {}; resetKey = Date.now(); changes = 0; });
    bindimport((colors) => {
      let p = { ...pending };
      for (const [k, v] of Object.entries(colors)) p[k] = v;
      pending = p;
      resetKey = Date.now();
      changes = Object.keys(colors).length;
    });
  });

  const groups = [
    { label: "APP", vars: [
      { name: "Background", key: "--bg-app" },
      { name: "Loader Screen", key: "--loader" },
      { name: "Scrollbar", key: "--scrollbar-thumb" },
      { name: "Badge Background", key: "--badge-bg" },
      { name: "Badge Text", key: "--badge-text" },
    ]},
    { label: "TITLEBAR", vars: [
      { name: "Background", key: "--topbar-bg" },
      { name: "Icons", key: "--topbar-icon" },
      { name: "Menu Button", key: "--topbar-menu-bg" },
      { name: "Menu Text", key: "--topbar-menu-text" },
      { name: "Menu Hover", key: "--topbar-menu-hover-bg" },
      { name: "Window Button", key: "--topbar-btn-bg" },
      { name: "Window Button Hover", key: "--topbar-btn-hover" },
    ]},
    { label: "SIDEBAR", vars: [
      { name: "Background", key: "--leftbar-bg" },
      { name: "Button Hover", key: "--leftbar-btn-hover" },
      { name: "Active Button", key: "--leftbar-btn-active-from", key2: "--leftbar-btn-active-to" },
      { name: "Active Icon", key: "--icon-leftbar-selected-top", key2: "--icon-leftbar-selected-bottom" },
      { name: "Inactive Icon", key: "--icon-leftbar-unselected" },
    ]},
    { label: "SIDEBAR — INDICATOR", vars: [
      { name: "Uninjected Color", key: "--uninjected" },
    ]},
    { label: "EXECUTION — TABS", vars: [
      { name: "Tab Bar Background", key: "--tab-bg" },
      { name: "Active Tab", key: "--tab-bg-selected" },
      { name: "Inactive Tab", key: "--tab-bg-unselected" },
      { name: "Tab Hover", key: "--tab-hover" },
      { name: "Active Tab Text", key: "--tab-text-selected" },
      { name: "Inactive Tab Text", key: "--tab-text-unselected" },
      { name: "Tab Icon", key: "--tab-icon" },
      { name: "Active Tab Icon", key: "--tab-icon-selected" },
      { name: "Close Button", key: "--tab-remove-icon" },
    ]},
    { label: "EXECUTION — TOOLBAR", vars: [
      { name: "Background", key: "--editor-toolbar-bg" },
      { name: "Execute Button", key: "--editor-execute-bg" },
      { name: "Execute Hover", key: "--editor-execute-bg-hover" },
      { name: "Execute Text", key: "--editor-execute-text" },
      { name: "Execute Icon", key: "--editor-execute-icon" },
      { name: "Inject Button", key: "--editor-inject-bg" },
      { name: "Inject Hover", key: "--editor-inject-bg-hover" },
      { name: "Inject Text", key: "--editor-inject-text" },
      { name: "Inject Icon", key: "--editor-inject-icon" },
      { name: "Clear Button", key: "--editor-clear-bg" },
      { name: "Clear Hover", key: "--editor-clear-bg-hover" },
      { name: "Clear Icon", key: "--editor-clear-icon" },
    ]},
    { label: "EXECUTION — CANVAS & TAB BAR", vars: [
      { name: "Canvas Background", key: "--editor-canvas-bg" },
      { name: "Tab Bar Background", key: "--editor-tabbar-bg" },
      { name: "New Tab Button", key: "--editor-newtab-bg" },
      { name: "New Tab Hover", key: "--editor-newtab-bg-hover" },
      { name: "New Tab Icon", key: "--editor-newtab-icon" },
    ]},
    { label: "EXECUTION — MONACO CODE EDITOR", vars: [
      { name: "Background", key: "--monaco-bg" },
      { name: "Keywords", key: "--monaco-keyword" },
      { name: "Strings", key: "--monaco-string" },
      { name: "Numbers", key: "--monaco-number" },
      { name: "Comments", key: "--monaco-comment" },
      { name: "Identifiers", key: "--monaco-identifier" },
      { name: "Delimiters", key: "--monaco-delimiter" },
      { name: "Line Numbers", key: "--monaco-line-number" },
      { name: "Active Line Number", key: "--monaco-line-number-active" },
      { name: "Selection", key: "--monaco-selection" },
      { name: "Scrollbar", key: "--monaco-scrollbar" },
      { name: "Scrollbar Hover", key: "--monaco-scrollbar-hover" },
    ]},
    { label: "EXECUTION — EXPLORER", vars: [
      { name: "Background", key: "--explorer-bg" },
      { name: "Section Label", key: "--explorer-label" },
      { name: "Search Result Path", key: "--explorer-result-path" },
      { name: "Tree Line", key: "--explorer-tree-line" },
      { name: "Expanded Folder", key: "--folder-expanded-bg" },
      { name: "Item Background", key: "--scriptitem-bg" },
      { name: "Item Hover", key: "--scriptitem-bg-hover" },
      { name: "Drop Target", key: "--scriptitem-bg-droptarget" },
      { name: "Item Text", key: "--scriptitem-text" },
      { name: "Drop Target Text", key: "--scriptitem-text-droptarget" },
      { name: "Item Icon", key: "--scriptitem-icon" },
      { name: "Rename Input", key: "--scriptitem-input-bg" },
      { name: "Rename Text", key: "--scriptitem-input-text" },
      { name: "Extension Button", key: "--scriptitem-ext-btn-bg" },
      { name: "Extension Hover", key: "--scriptitem-ext-btn-bg-hover" },
      { name: "Extension Text", key: "--scriptitem-ext-btn-text" },
      { name: "Open Button", key: "--open-bg" },
      { name: "Open Hover", key: "--open-hover" },
      { name: "Open Icon", key: "--open-icon" },
      { name: "Open Text", key: "--open-text" },
      { name: "Save Button", key: "--save-bg" },
      { name: "Save Hover", key: "--save-hover" },
      { name: "Save Icon", key: "--save-icon" },
      { name: "Save Text", key: "--save-text" },
    ]},
    { label: "EXECUTION — SEARCH BAR", vars: [
      { name: "Background", key: "--search-input-bg" },
      { name: "Hover", key: "--search-input-bg-hover" },
      { name: "Text", key: "--search-input-text" },
      { name: "Icon", key: "--search-input-icon" },
      { name: "Placeholder", key: "--search-input-placeholder" },
      { name: "Open Dir Button", key: "--search-opendir-bg" },
      { name: "Open Dir Hover", key: "--search-opendir-bg-hover" },
      { name: "Open Dir Icon", key: "--search-opendir-icon" },
    ]},
    { label: "SETTINGS — GENERAL", vars: [
      { name: "Sidebar Background", key: "--settings-sidebar-bg" },
      { name: "Sidebar Label", key: "--settings-sidebar-label" },
      { name: "Sidebar Item Hover", key: "--settings-sidebar-item-hover" },
      { name: "Content Background", key: "--settings-content-bg" },
      { name: "Tab Button Inactive", key: "--settings-btn-unselected" },
      { name: "Tab Button Active", key: "--settings-btn-selected-from", key2: "--settings-btn-selected-to" },
      { name: "Tab Icon Inactive", key: "--settings-icon-unselected" },
      { name: "Tab Icon Active", key: "--settings-icon-selected-top", key2: "--settings-icon-selected-bottom" },
      { name: "Tab Text Inactive", key: "--settings-text-unselected" },
      { name: "Tab Text Active", key: "--settings-text-selected-top", key2: "--settings-text-selected-bottom" },
      { name: "Interface Title", key: "--interface-title" },
      { name: "Interface Description", key: "--interface-desc" },
      { name: "Core Title", key: "--core-title" },
      { name: "Core Description", key: "--core-desc" },
      { name: "Editor Title", key: "--editor-title" },
      { name: "Editor Description", key: "--editor-desc" },
      { name: "Themes Title", key: "--themes-title" },
      { name: "Themes Description", key: "--themes-desc" },
    ]},
    { label: "SETTINGS — SEARCH", vars: [
      { name: "Background", key: "--search-bg" },
      { name: "Hover", key: "--search-hover" },
      { name: "Text", key: "--search-text" },
      { name: "Icon", key: "--search-icon" },
      { name: "Filter Button", key: "--search-filter-btn-bg" },
      { name: "Filter Hover", key: "--search-filter-btn-hover" },
      { name: "Filter Icon", key: "--search-filter-btn-icon" },
    ]},
    { label: "SETTINGS — ITEMS", vars: [
      { name: "Card Background", key: "--item-bg" },
      { name: "Icon", key: "--item-icon" },
      { name: "Title", key: "--item-title" },
      { name: "Description", key: "--item-desc" },
      { name: "Description Icon", key: "--item-desc-icon" },
      { name: "Checkbox Off", key: "--item-check-off" },
      { name: "Checkbox Hover", key: "--item-check-hover" },
      { name: "Checkbox On", key: "--item-check-on" },
      { name: "Checkbox Icon", key: "--item-check-icon" },
      { name: "Button", key: "--item-btn-bg" },
      { name: "Button Hover", key: "--item-btn-hover" },
      { name: "Button Text", key: "--item-btn-text" },
      { name: "Dropdown", key: "--item-dropdown-btn-bg" },
      { name: "Dropdown Hover", key: "--item-dropdown-btn-hover" },
      { name: "Dropdown Text", key: "--item-dropdown-btn-text" },
      { name: "Textbox", key: "--item-textbox-bg" },
      { name: "Textbox Hover", key: "--item-textbox-bg-hover" },
      { name: "Textbox Focus", key: "--item-textbox-bg-focus" },
      { name: "Textbox Text", key: "--item-textbox-text" },
      { name: "Textbox Placeholder", key: "--item-textbox-placeholder" },
      { name: "Slider Track", key: "--item-slider-track" },
      { name: "Slider Fill", key: "--item-slider-fill" },
      { name: "Slider Thumb", key: "--item-slider-thumb" },
      { name: "Slider Value Text", key: "--item-slider-value" },
    ]},
    { label: "SETTINGS — THEME ITEMS", vars: [
      { name: "Card Background", key: "--theme-item-bg" },
      { name: "Icon", key: "--theme-item-icon" },
      { name: "Title", key: "--theme-item-title" },
      { name: "Hex Label", key: "--theme-item-hex" },
      { name: "Top/Bottom Label", key: "--theme-item-label" },
      { name: "Swatch Outline", key: "--theme-item-swatch-outline" },
      { name: "Section Label", key: "--theme-section-label" },
      { name: "Section Divider", key: "--theme-section-divider" },
      { name: "Toolbar Button", key: "--theme-toolbar-btn-bg" },
      { name: "Toolbar Button Hover", key: "--theme-toolbar-btn-hover" },
      { name: "Toolbar Button Text", key: "--theme-toolbar-btn-text" },
    ]},
    { label: "DROPDOWN & CONTEXT MENU", vars: [
      { name: "Background", key: "--menu-bg" },
      { name: "Item Text", key: "--menu-item-text" },
      { name: "Item Hover", key: "--menu-item-hover" },
      { name: "Selected Item", key: "--menu-item-selected-bg" },
      { name: "Selected Text", key: "--menu-item-selected-text" },
      { name: "Selected Icon", key: "--menu-item-selected-icon" },
      { name: "Separator", key: "--menu-separator" },
      { name: "Disabled Item", key: "--menu-item-disabled" },
      { name: "Context Menu Background", key: "--ctx-bg" },
      { name: "Context Item Text", key: "--ctx-item-text" },
      { name: "Context Item Hover", key: "--ctx-item-hover" },
      { name: "Context Item Disabled", key: "--ctx-item-disabled" },
    ]},
    { label: "DIALOGS & PROMPTS", vars: [
      { name: "Overlay", key: "--prompt-overlay" },
      { name: "Background", key: "--prompt-bg" },
      { name: "Title", key: "--prompt-title" },
      { name: "Message", key: "--prompt-message" },
      { name: "Input", key: "--prompt-input-bg" },
      { name: "Input Hover", key: "--prompt-input-bg-hover" },
      { name: "Input Text", key: "--prompt-input-text" },
      { name: "Input Placeholder", key: "--prompt-input-placeholder" },
      { name: "Confirm Button", key: "--prompt-btn-confirm-bg" },
      { name: "Confirm Hover", key: "--prompt-btn-confirm-bg-hover" },
      { name: "Confirm Text", key: "--prompt-btn-confirm-text" },
      { name: "Cancel Button", key: "--prompt-btn-cancel-bg" },
      { name: "Cancel Hover", key: "--prompt-btn-cancel-bg-hover" },
      { name: "Cancel Text", key: "--prompt-btn-cancel-text" },
      { name: "Extension Button", key: "--prompt-ext-btn-bg" },
      { name: "Extension Hover", key: "--prompt-ext-btn-bg-hover" },
      { name: "Extension Text", key: "--prompt-ext-btn-text" },
    ]},
    { label: "PREVIEW CAROUSEL", vars: [
      { name: "Active Dot", key: "--preview-dot-active" },
      { name: "Inactive Dot", key: "--preview-dot-inactive" },
      { name: "Nav Button", key: "--preview-nav-bg" },
      { name: "Nav Hover", key: "--preview-nav-hover" },
      { name: "Nav Icon", key: "--preview-nav-icon" },
    ]},
    { label: "DRAG & DROP", vars: [
      { name: "Overlay", key: "--dragdrop-overlay-bg" },
      { name: "Card Background", key: "--dragdrop-card-bg" },
      { name: "Title", key: "--dragdrop-title" },
      { name: "Description", key: "--dragdrop-desc" },
      { name: "Invalid Overlay", key: "--dragdrop-invalid-bg" },
      { name: "Invalid Card", key: "--dragdrop-invalid-card" },
      { name: "Invalid Title", key: "--dragdrop-invalid-title" },
      { name: "Invalid Text", key: "--dragdrop-invalid-text" },
    ]},
    { label: "NEWS PAGE", vars: [
      { name: "Overlay", key: "--news-overlay" },
      { name: "Background", key: "--news-bg" },
    ]},
    { label: "CLOUD PAGE", vars: [
      { name: "Background", key: "--cloud-bg" },
    ]},
  ];

  function matchQ(t) { return !query || t.toLowerCase().includes(query.toLowerCase()); }
  function groupOk(g) { return !query || matchQ(g.label) || g.vars.some(v => matchQ(v.name) || matchQ(v.key)); }
  function varOk(v) { return !query || matchQ(v.name) || matchQ(v.key) || (v.key2 && matchQ(v.key2)); }

  function snap() {
    const out = {};
    for (const g of groups) for (const v of g.vars) {
      out[v.key] = pending[v.key] ?? dom(v.key);
      if (v.key2) out[v.key2] = pending[v.key2] ?? dom(v.key2);
    }
    return out;
  }

  function onScroll() {
    if (!scrollEl) return;
    const top = scrollEl.scrollTop;
    let cur = '';
    for (const [lbl, el] of Object.entries(sectionEls)) {
      if (el && el.offsetTop <= top + 8) cur = lbl;
    }
    sticky = cur;
  }
</script>

<div class="w-full h-full flex flex-col">
  <div class="mx-3 mt-3 w-full h-[42.5px] flex items-center gap-5 font-[565] text-[15px] flex-shrink-0">
    <span style="color:var(--themes-title)">THEME SETTINGS</span>
    <span style="color:var(--themes-desc)">Edit every color in the UI</span>
  </div>

  {#if sticky}
    <div class="px-3 py-1.5 flex-shrink-0" style="background:var(--settings-content-bg);border-bottom:1px solid var(--theme-section-divider)">
      <span class="font-bold text-[15px] uppercase" style="color:var(--theme-section-label)">{sticky}</span>
    </div>
  {/if}

  <div bind:this={scrollEl} onscroll={onScroll} class="flex-1 overflow-y-auto px-3 pb-1" style="scrollbar-width:thin;scrollbar-color:var(--scrollbar-thumb) transparent">
    <div class="flex flex-col gap-6 pt-1">
      {#each groups as group}
        {#if groupOk(group)}
          <div bind:this={sectionEls[group.label]}>
            <div class="flex items-center gap-3 mb-2">
              <span class="font-bold text-[15px] uppercase flex-shrink-0" style="color:var(--theme-section-label)">{group.label}</span>
              <div class="flex-1 h-px" style="background:var(--theme-section-divider)"></div>
            </div>
            <div class="flex flex-col rounded-md overflow-hidden" style="background:var(--theme-item-bg)">
              {#each group.vars.filter(v => varOk(v)) as v, i}
                {#key resetKey}
                  <ThemeItem name={v.name} cssKey={v.key} cssKey2={v.key2 ?? null} first={i===0}
                    {getPending} {setPending} onchange={() => changes++} />
                {/key}
              {/each}
            </div>
          </div>
        {/if}
      {/each}
    </div>
  </div>

  <div class="flex items-center gap-3 px-3 py-3 flex-shrink-0" style="background:var(--settings-content-bg)">
    {#each [['Import', onimport], ['Export', () => onexport(snap)], ['Reset', onreset]] as [lbl, fn]}
      <button class="rounded-md text-[15px] font-[565] px-4" style="height:36px;background:var(--theme-toolbar-btn-bg);color:var(--theme-toolbar-btn-text)"
        onmouseenter={(e)=>e.currentTarget.style.background='var(--theme-toolbar-btn-hover)'}
        onmouseleave={(e)=>e.currentTarget.style.background='var(--theme-toolbar-btn-bg)'}
        onclick={fn}>{lbl}</button>
    {/each}
    <div class="ml-auto relative">
      {#if changes > 0}
        <span class="absolute -top-1.5 -right-1.5 z-10 rounded-full text-[9px] font-bold flex items-center justify-center pointer-events-none"
          style="min-width:16px;height:16px;padding:0 4px;background:var(--badge-bg);color:var(--badge-text)">{changes}</span>
      {/if}
      <button onclick={() => onpreview(snap())}
        class="rounded-md text-[15px] font-[565] px-4"
        style="height:36px;background:var(--theme-toolbar-btn-bg);color:var(--theme-toolbar-btn-text)"
        onmouseenter={(e)=>e.currentTarget.style.background='var(--theme-toolbar-btn-hover)'}
        onmouseleave={(e)=>e.currentTarget.style.background='var(--theme-toolbar-btn-bg)'}>Preview</button>
    </div>
  </div>
</div>