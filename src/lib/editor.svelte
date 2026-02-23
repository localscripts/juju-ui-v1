<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { untrack } from "svelte";
  import { get } from "svelte/store";
  import { s } from "./data.ts";
  import Monaco from './monaco.svelte';
  import Tab from './tab.svelte';

  let { file = $bindable(null), selectedTabData = $bindable(null) } = $props();
  let tabs = $state([]);
  let selectedTab = $state(null);
  let tabContainer;
  let editingTab = $state(null);
  let editVal = $state('');
  let tabRefs = $state({});
  let monacoRefs = $state({});

  onMount(async () => {
    if (get(s).saveTabs) {
      const loaded = await invoke("load_tabs");
      tabs = loaded.map(t => ({...t, content: t.content || ''}));
      if (tabs.length > 0) selectedTab = tabs[0].id;
    }
  });

  $effect(() => {
    selectedTabData = tabs.find(t => t.id === selectedTab) ?? null;
  });

  $effect(() => {
    if (!selectedTab) return;
    requestAnimationFrame(() => monacoRefs[selectedTab]?.forceLayout());
  });

  $effect(() => {
    if (file) {
      const exists = tabs.find(t => t.path === file.path);
      if (!exists) {
        const newTab = { id: Date.now(), name: file.name, content: file.content || '', path: file.path };
        tabs = [...tabs, newTab];
        selectedTab = newTab.id;
        saveTabs();
        setTimeout(() => tabRefs[newTab.id]?.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'center' }), 50);
      } else {
        selectedTab = exists.id;
        setTimeout(() => tabRefs[exists.id]?.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'center' }), 50);
      }
      file = null;
    }
  });

  async function saveTabs() {
    const t = untrack(() => tabs);
    if (get(s).saveTabs) await invoke("save_tabs", { tabs: t });
    for (const tab of t) {
      if (tab.path) await invoke("write", { path: tab.path, content: tab.content });
    }
  }

  function addTab() {
    const newTab = { id: Date.now(), name: `Untitled-${tabs.length + 1}`, content: '', path: null };
    tabs = [...tabs, newTab];
    selectedTab = newTab.id;
    saveTabs();
    setTimeout(() => { if (tabContainer) tabContainer.scrollLeft = tabContainer.scrollWidth; }, 50);
  }

  function closeTab(id, e) {
    e?.stopPropagation();
    tabs = tabs.filter(t => t.id !== id);
    if (selectedTab === id) selectedTab = tabs.length > 0 ? tabs[tabs.length - 1].id : null;
    saveTabs();
  }

  function renameTab(id, newName) {
    if (newName.trim() && newName.trim() !== tabs.find(t => t.id === id)?.name) {
      tabs = tabs.map(t => t.id === id ? {...t, name: newName.trim()} : t);
      saveTabs();
    }
    editingTab = null;
  }

  function clear() {
    const idx = tabs.findIndex(t => t.id === selectedTab);
    if (idx !== -1) {
      tabs[idx].content = '';
      tabs = [...tabs];
      saveTabs();
    }
  }

  let saveTimeout;
  $effect(() => {
    tabs.map(t => t.content).join('|');
    clearTimeout(saveTimeout);
    saveTimeout = setTimeout(() => saveTabs(), 500);
  });
</script>


<div class="flex flex-col h-full w-full overflow-hidden">
  <div class="flex-shrink-0 py-3 flex items-center gap-5 px-4" style="background: var(--editor-tabbar-bg);">
    <div
      bind:this={tabContainer}
      onwheel={(e) => tabContainer.scrollLeft += e.deltaY}
      class="flex items-center gap-5 overflow-x-scroll scrollbar-hide scroll-smooth flex-1 min-w-0"
    >
      {#each tabs as tab (tab.id)}
        <div bind:this={tabRefs[tab.id]}>
          <Tab
            {tab}
            selected={selectedTab === tab.id}
            editing={editingTab === tab.id}
            bind:editVal
            onselect={() => selectedTab = tab.id}
            onclose={(e) => closeTab(tab.id, e)}
            onedit={(val) => { if (val) { editingTab = tab.id; editVal = tab.name; } else { editingTab = null; } }}
            onrename={(newName) => renameTab(tab.id, newName)}
          />
        </div>
      {/each}
    </div>

    <button
      onclick={addTab}
      class="h-10 w-10 flex-shrink-0 p-2 font-medium rounded-md flex items-center justify-center"
      style="color: var(--editor-newtab-icon); background: var(--editor-newtab-bg);"
      onmouseenter={(e) => e.currentTarget.style.background = 'var(--editor-newtab-bg-hover)'}
      onmouseleave={(e) => e.currentTarget.style.background = 'var(--editor-newtab-bg)'}
    >
      <svg class="rotate-45 w-[13px] h-[13px]" viewBox="0 0 13 13" fill="none">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M0.316867 0.330729C0.519817 0.127635 0.795041 0.0135436 1.08201 0.0135436C1.36899 0.0135436 1.64421 0.127635 1.84716 0.330729L6.49323 4.98148L11.1393 0.330729C11.2391 0.22726 11.3586 0.144729 11.4906 0.0879524C11.6226 0.031176 11.7647 0.00129087 11.9084 4.09035e-05C12.0521 -0.00120906 12.1946 0.0262009 12.3276 0.0806718C12.4606 0.135143 12.5814 0.215584 12.683 0.317301C12.7846 0.419018 12.865 0.539974 12.9194 0.673112C12.9738 0.80625 13.0012 0.948903 13 1.09275C12.9987 1.23659 12.9689 1.37875 12.9121 1.51092C12.8554 1.64309 12.773 1.76263 12.6696 1.86256L8.02353 6.51331L12.6696 11.1641C12.8667 11.3684 12.9758 11.642 12.9734 11.9261C12.9709 12.2101 12.8571 12.4818 12.6564 12.6827C12.4558 12.8836 12.1843 12.9975 11.9006 13C11.6168 13.0024 11.3434 12.8932 11.1393 12.6959L6.49323 8.04515L1.84716 12.6959C1.64305 12.8932 1.36967 13.0024 1.08591 13C0.802147 12.9975 0.530707 12.8836 0.33005 12.6827C0.129393 12.4818 0.0155741 12.2101 0.0131083 11.9261C0.0106425 11.642 0.119727 11.3684 0.316867 11.1641L4.96294 6.51331L0.316867 1.86256C0.113977 1.65941 0 1.38391 0 1.09665C0 0.809384 0.113977 0.533884 0.316867 0.330729Z" fill="var(--editor-newtab-icon)"/>
      </svg>
    </button>
  </div>

  <div class="flex-1 w-full overflow-hidden relative" style="background: var(--editor-canvas-bg);">
    {#each tabs as tab (tab.id)}
      <div class="absolute inset-0 mr-8 {selectedTab === tab.id ? 'block' : 'hidden'}">
        <Monaco bind:this={monacoRefs[tab.id]} bind:value={tab.content} />
      </div>
    {/each}
  </div>

  <div class="flex gap-5 py-3 px-4" style="background: var(--editor-toolbar-bg);">
  
    <button
      class="flex items-center font-semibold gap-2 rounded-md p-2"
      style="background: var(--editor-execute-bg); color: var(--editor-execute-text);"
      onmouseenter={(e) => e.currentTarget.style.background = 'var(--editor-execute-bg-hover)'}
      onmouseleave={(e) => e.currentTarget.style.background = 'var(--editor-execute-bg)'}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="var(--editor-execute-icon)" class="size-5.5">
        <path d="M4.62355 3.35132C4.85479 3.21713 5.13998 3.21617 5.3721 3.34882L19.3721 11.3488C19.6058 11.4824 19.75 11.7309 19.75 12C19.75 12.2691 19.6058 12.5177 19.3721 12.6512L5.3721 20.6512C5.13998 20.7838 4.85479 20.7829 4.62355 20.6487C4.39232 20.5145 4.25 20.2674 4.25 20V4C4.25 3.73265 4.39232 3.48551 4.62355 3.35132Z"></path>
      </svg>
      Execute
    </button>
    
    <button
      class="flex items-center font-semibold gap-2 rounded-md p-2"
      style="background: var(--editor-inject-bg); color: var(--editor-inject-text);"
      onmouseenter={(e) => e.currentTarget.style.background = 'var(--editor-inject-bg-hover)'}
      onmouseleave={(e) => e.currentTarget.style.background = 'var(--editor-inject-bg)'}
    >
      <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="var(--editor-inject-icon)" class="size-6.5">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M5 8C5 7.44772 5.44772 7 6 7H14C14.5523 7 15 7.44772 15 8V13.5H13V9H7V20H13V18.5H15V21C15 21.5523 14.5523 22 14 22H6C5.44772 22 5 21.5523 5 21V8Z"></path>
        <path fill-rule="evenodd" clip-rule="evenodd" d="M9 3C9 2.44772 9.44772 2 10 2H18C18.5523 2 19 2.44772 19 3V16C19 16.5523 18.5523 17 18 17H10C9.44772 17 9 16.5523 9 16V10.5H11V15H17V4H11V5.5H9V3Z"></path>
      </svg>
      Inject
    </button>


    <button
      onclick={clear}
      class="h-[40px] w-[40px] flex justify-center items-center rounded-md pl-1 ml-auto"
      style="background: var(--editor-clear-bg);"
      onmouseenter={(e) => e.currentTarget.style.background = 'var(--editor-clear-bg-hover)'}
      onmouseleave={(e) => e.currentTarget.style.background = 'var(--editor-clear-bg)'}
    >
      <svg xmlns="http://www.w3.org/2000/svg" class="size-5.5" fill="var(--editor-clear-icon)" viewBox="0 0 24 24">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M1.25 4C1.25 3.58579 1.58579 3.25 2 3.25H17.4157L22.636 11.6025C22.788 11.8457 22.788 12.1543 22.636 12.3975L17.4157 20.75H2C1.58579 20.75 1.25 20.4142 1.25 20V4ZM12.293 8.29285L9.99986 10.5858L7.70714 8.29324L6.29297 9.7075L8.5856 12L6.29297 14.2925L7.70714 15.7067L9.99986 13.4141L12.293 15.7071L13.7071 14.2928L11.4141 12L13.7071 9.70711L12.293 8.29285Z"></path>
      </svg>
    </button>

    
  </div>
</div>

<style>
  .scrollbar-hide::-webkit-scrollbar { display: none; }
  .scrollbar-hide { -ms-overflow-style: none; scrollbar-width: none; }
</style>