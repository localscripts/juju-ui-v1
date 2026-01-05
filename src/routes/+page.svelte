<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import Leftbar from "../lib/leftbar.svelte";
  import Scriptitem from "../lib/scriptitem.svelte";
  import Search from "../lib/search.svelte";
  import Topbar from "../lib/topbar.svelte";
  import Editor from "../lib/editor.svelte";
  import Context from "../lib/context.svelte";
  import Prompt from "../lib/prompt.svelte";

  let folders = $state([]);
  let expanded = $state({});
  let openFile = $state(null);
  let query = $state("");
  let searchResults = $state([]);
  let contextMenu = $state(null);
  let prompt = $state(null);
  let renamingPath = $state(null);

  onMount(async () => await refresh());

  async function refresh() {
    folders = await invoke("get_folders");
    for (let path in expanded) {
      if (expanded[path]) expanded[path] = await invoke("get_items", { path });
    }
  }

  $effect(() => {
    if (query.trim()) {
      invoke("search_files", { query }).then(r => searchResults = r);
    } else {
      searchResults = [];
    }
  });

  async function toggle(folder) {
    expanded[folder.path] = expanded[folder.path] ? null : await invoke("get_items", { path: folder.path });
  }

  async function opn(file) {
    const content = await invoke("read_file", { path: file.path });
    openFile = { name: file.name, content, path: file.path };
  }

  function context(e, item) {
    e.preventDefault();
    e.stopPropagation();
    
    const h = item.is_folder ? 160 : 120;
    const up = window.innerHeight - e.clientY < h;
    
    const opts = item.is_folder 
      ? [
          { label: expanded[item.path] ? "Close" : "Open", action: () => toggle(item), disabled: false },
          { label: "New File", action: () => newFile(item), disabled: false },
          { label: "Rename", action: () => renamingPath = item.path, disabled: item.is_root },
          { label: "Delete", action: () => deleteItem(item), disabled: item.is_root }
        ]
      : [
          { label: "Open", action: () => opn(item), disabled: false },
          { label: "Rename", action: () => renamingPath = item.path, disabled: false },
          { label: "Delete", action: () => deleteItem(item), disabled: false }
        ];
    
    contextMenu = { x: e.clientX, y: up ? e.clientY - h : e.clientY, options: opts };
  }

  function newFile(folder) {
    prompt = {
      title: "New File",
      input: true,
      placeholder: "newfile.lua",
      defaultValue: "newfile.lua",
      onconfirm: async (name) => {
        if (name) {
          await invoke("create_file", { path: folder.path, name });
          await refresh();
        }
        prompt = null;
      },
      oncancel: () => prompt = null
    };
  }

  async function rename(item, newName) {
    if (!newName || newName === item.name) {
      renamingPath = null;
      return;
    }
    await invoke("rename_item", { oldPath: item.path, newName });
    await refresh();
    renamingPath = null;
  }

  function deleteItem(item) {
    prompt = {
      title: "Delete",
      message: `Are you sure you want to delete "${item.name}"?`,
      onconfirm: async () => {
        await invoke("delete_item", { path: item.path });
        await refresh();
        prompt = null;
      },
      oncancel: () => prompt = null
    };
  }
</script>

<svelte:window 
  onkeydown={(e) => {
    if (e.key === 'Escape' && renamingPath) {
      renamingPath = null;
    }
  }}
/>

<Leftbar />
<Topbar />

{#if contextMenu}
  <Context {...contextMenu} onclose={() => contextMenu = null} />
{/if}

{#if prompt}
  <Prompt {...prompt} />
{/if}

<div class="fixed inset-0 flex text-white" style="top: 62px; left: 64px;">
  <div class="w-65 min-w-65 border-r-[1.5px] border-[#A8E8EB]/12.5 bg-[#111111] flex flex-col" style="height: 100%;">
    <div class="p-5 pb-3 flex flex-col gap-4">
      <Search bind:value={query} />
      <h class="font-bold text-[15px] text-white/25">EXPLORER</h>
    </div>
    
    <div class="flex flex-col gap-2 px-5 pb-5 scrollbar overflow-y-auto" style="flex: 1;">
      {#if query.trim()}
        {#each searchResults as item}
          <button onclick={() => opn(item)} class="w-full">
            <div class="flex flex-col gap-1">
              <Scriptitem name={item.name} luafile={!item.is_folder} icon={item.is_folder ? "/file.svg" : "/luafile.svg"} />
              <div class="text-xs text-white/30 px-2 truncate">{item.path}</div>
            </div>
          </button>
        {/each}
      {:else}
        {#each folders as folder}
          <div>
            <button onclick={() => toggle(folder)} oncontextmenu={(e) => context(e, folder)} class="w-full">
              <Scriptitem name={folder.name} icon="/file.svg" expanded={expanded[folder.path]} renaming={renamingPath === folder.path} onrename={(n) => rename(folder, n)} />
            </button>
            
            {#if expanded[folder.path]}
              <div class="pl-3 gap-2 flex flex-col mt-2">
                {#each expanded[folder.path] as item}
                  {#if item.is_folder}
                    <div>
                      <button onclick={() => toggle(item)} oncontextmenu={(e) => context(e, item)} class="w-full">
                        <Scriptitem name={item.name} icon="/file.svg" expanded={expanded[item.path]} renaming={renamingPath === item.path} onrename={(n) => rename(item, n)} />
                      </button>
                      
                      {#if expanded[item.path]}
                        <div class="pl-3 gap-2 flex flex-col mt-2">
                          {#each expanded[item.path] as file}
                            <button onclick={() => opn(file)} oncontextmenu={(e) => context(e, file)} class="w-full">
                              <Scriptitem name={file.name} luafile={true} icon="/luafile.svg" renaming={renamingPath === file.path} onrename={(n) => rename(file, n)} />
                            </button>
                          {/each}
                        </div>
                      {/if}
                    </div>
                  {:else}
                    <button onclick={() => opn(item)} oncontextmenu={(e) => context(e, item)} class="w-full">
                      <Scriptitem name={item.name} luafile={true} icon="/luafile.svg" renaming={renamingPath === item.path} onrename={(n) => rename(item, n)} />
                    </button>
                  {/if}
                {/each}
              </div>
            {/if}
          </div>
        {/each}
      {/if}
    </div>
  </div>
  <Editor file={openFile} />
</div>

<style>
  :global(html) {
    background-color: #0d0d0d;
  }

  .scrollbar {
    scrollbar-width: thin;
    scrollbar-color: #242424 transparent;
  }
</style>