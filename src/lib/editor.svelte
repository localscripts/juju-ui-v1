<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Monaco from './monaco.svelte';
  
  let { file = $bindable(null) } = $props();
  let tabs = $state([]);
  let selectedTab = $state(null);
  let tabContainer;
  let editingTab = $state(null);
  let editVal = $state('');
  let renameInp;
  let tabRefs = {};

  $effect(() => {
    if (editingTab !== null) {
      setTimeout(() => {
        renameInp?.focus();
        renameInp?.select();
      }, 0);
    }
  });

  onMount(async () => {
    const loaded = await invoke("load_tabs");
    tabs = loaded;
    if (tabs.length > 0) {
      selectedTab = tabs[0].id;
    }
  });

  $effect(() => {
    if (file) {
      const exists = tabs.find(t => t.path === file.path);
      if (!exists) {
        const newTab = {
          id: Date.now(),
          name: file.name,
          content: file.content,
          path: file.path
        };
        tabs = [...tabs, newTab];
        selectedTab = newTab.id;
        saveTabs();
        setTimeout(() => {
          tabRefs[newTab.id]?.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'center' });
        }, 50);
      } else {
        selectedTab = exists.id;
        setTimeout(() => {
          tabRefs[exists.id]?.scrollIntoView({ behavior: 'smooth', block: 'nearest', inline: 'center' });
        }, 50);
      }
      file = null;
    }
  });

  async function saveTabs() {
    await invoke("save_tabs", { tabs });
  }

  function addTab() {
    const newTab = {
      id: Date.now(),
      name: `Untitled-${tabs.length + 1}`,
      content: '',
      path: null
    };
    tabs = [...tabs, newTab];
    selectedTab = newTab.id;
    saveTabs();
    setTimeout(() => {
      if (tabContainer) {
        tabContainer.scrollLeft = tabContainer.scrollWidth;
      }
    }, 50);
  }

  function closeTab(id, e) {
    e?.stopPropagation();
    tabs = tabs.filter(t => t.id !== id);
    if (selectedTab === id) {
      selectedTab = tabs.length > 0 ? tabs[tabs.length - 1].id : null;
    }
    saveTabs();
  }

  function selectTab(id) {
    selectedTab = id;
  }

  function renameTab(id, newName) {
    if (newName.trim() && newName.trim() !== tabs.find(t => t.id === id)?.name) {
      tabs = tabs.map(t => t.id === id ? {...t, name: newName.trim()} : t);
      saveTabs();
    }
    editingTab = null;
  }

  $effect(() => {
    const current = tabs.find(t => t.id === selectedTab);
    if (current) {
      saveTabs();
    }
  });
</script>

<svelte:head>
  <script src="https://cdnjs.cloudflare.com/ajax/libs/monaco-editor/0.45.0/min/vs/loader.min.js"></script>
</svelte:head>

<div class="flex flex-col h-full w-full overflow-hidden">
  <div class="flex-shrink-0 py-3 flex items-center gap-5 px-4 bg-[#111111] border-b-[1.5px] border-[#A8E8EB]/12.5">
    <div 
      bind:this={tabContainer}
      onwheel={(e) => {
        tabContainer.scrollLeft += e.deltaY;
      }}
      class="flex items-center gap-5 overflow-x-scroll scrollbar-hide scroll-smooth flex-1 min-w-0"
    >
      {#each tabs as tab (tab.id)}
        <button
          bind:this={tabRefs[tab.id]}
          onclick={() => selectTab(tab.id)}
          class="p-2 hover:bg-white/12.5 font-medium min-w-42.5 px-2.5 justify-between flex items-center gap-2 rounded-md transition-all duration-200 flex-shrink-0 {selectedTab === tab.id ? 'bg-[#242424] text-[#FFFFFF]/50' : 'text-[#FFFFFF]/25'}"
        >
          <div class="flex items-center gap-3.75 max-w-32">
            <img class="size-3.75 flex-shrink-0" src={selectedTab === tab.id ? '/text.svg' : '/textunselected.svg'} />
            {#if editingTab === tab.id}
              <input
                bind:this={renameInp}
                type="text"
                bind:value={editVal}
                onblur={() => {
                  if (editVal.trim()) renameTab(tab.id, editVal);
                  else editingTab = null;
                }}
                onkeydown={(e) => {
                  if (e.key === 'Enter') {
                    if (editVal.trim()) renameTab(tab.id, editVal);
                    else editingTab = null;
                  }
                  if (e.key === 'Escape') editingTab = null;
                }}
                onclick={(e) => e.stopPropagation()}
                class="bg-transparent outline-none border-none text-inherit w-full"
              />
            {:else}
              <span 
                ondblclick={() => {
                  editingTab = tab.id;
                  editVal = tab.name;
                  setTimeout(() => renameInp?.focus(), 0);
                }}
                class="truncate"
              >
                {tab.name}
              </span>
            {/if}
          </div>

          {#if selectedTab === tab.id}
            <button onclick={(e) => closeTab(tab.id, e)}>
              <img src="/x.svg" />
            </button>
          {/if}
        </button>
      {/each}
    </div>

    <button
      onclick={addTab}
      class="h-10 w-10 flex-shrink-0 p-2 hover:bg-white/12.5 text-[#FFFFFF]/25 font-medium rounded-md transition-all duration-200 flex items-center justify-center"
    >
      <img class="rotate-45" src="/x.svg" />
    </button>
  </div>

  <div class="flex-1 w-full overflow-hidden relative">
    {#each tabs as tab (tab.id)}
      <div class="absolute inset-0 {selectedTab === tab.id ? 'block' : 'hidden'}">
        <Monaco bind:value={tab.content} />
      </div>
    {/each}
  </div>

  <div class="w-full flex items-center justify-between gap-4 bg-[#111111] p-4 border-t-[1.5px] border-[#A8E8EB]/12.5">
    <div class="flex gap-3">
      <button class="bg-[#242424] hover:bg-[#323232] duration-75 flex text-[#FFFFFF]/50 items-center font-semibold gap-3 rounded-md p-2">
        <img class="size-5" src="/attach.svg" />
        Attach
      </button>
      <button class="bg-[#242424] hover:bg-[#323232] duration-75 flex text-[#FFFFFF]/50 items-center font-semibold gap-3 rounded-md p-2.5">
        <img alt="attach" class="size-5" src="/savefile.svg" />
      </button>
      <button class="bg-[#242424] hover:bg-[#323232] duration-75 flex text-[#FFFFFF]/50 items-center font-semibold gap-3 rounded-md p-2">
        <img class="size-5" src="/execute.svg" />
        Execute
      </button>
    </div>
  </div>
</div>

<style>
  .scrollbar-hide::-webkit-scrollbar {
    display: none;
  }
  .scrollbar-hide {
    -ms-overflow-style: none;
    scrollbar-width: none;
  }
</style>