<script>
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/core";
  import { get } from "svelte/store";
  import { listen } from '@tauri-apps/api/event';
  import Leftbar from "../lib/leftbar.svelte";
  import Scriptitem from "../lib/scriptitem.svelte";
  import Search from "../lib/search.svelte";
  import Topbar from "../lib/topbar.svelte";
  import Editor from "../lib/editor.svelte";
  import Context from "../lib/context.svelte";
  import Prompt from "../lib/prompt.svelte";
  import Settings from "../lib/pages/settings.svelte";
  import Cloud from "../lib/pages/cloud.svelte";
  import News from "../lib/pages/news.svelte";
  import { s, loadSettings, saveSettings } from "../lib/data.ts";
  import Loader from "../lib/loader.svelte";
  import PreviewEditor from "../lib/previews/preview-editor.svelte";
  import PreviewSettings from "../lib/previews/preview-settings.svelte";

  let loading = $state(true);
  let activePage = $state('editor');
  let cursor = new Audio('/cursor.mp3');

  function click() {
    if (!get(s).sounds) return;
    cursor.volume = 0.5;
    cursor.currentTime = 0;
    cursor.play();
  }

  let folders = $state([]);
  let expanded = $state({});
  let openFile = $state(null);
  let query = $state("");
  let results = $state([]);
  let ctxMenu = $state(null);
  let prompt = $state(null);
  let news = $state(false);
  let renaming = $state(null);
  let show = $state(true);
  let dragging = $state(false);
  let validDrag = $state(false);
  let dropTarget = $state(null);
  let mouseX = 0, mouseY = 0;
  let folderEls = $state({});
  let selectedTabData = $state(null);

  let previewColors = $state(null);
  let importP = $state(false);
  let exportP = $state(false);
  let resetP = $state(false);

  // callbacks set by themes.svelte so page can trigger actions on it
  let themeExportSnap = $state(() => ({}));
  let themeDoReset = $state(() => {});    // clears pending in themes
  let themeDoImport = $state(() => {});   // sets pending in themes from imported colors

  const previewSlides = [
    { label: 'Editor', component: PreviewEditor },
    { label: 'Settings', component: PreviewSettings },
  ];

  onMount(async () => {
    await refresh();
    await loadSettings();

    let first = true;
    s.subscribe((val) => {
      if (first) { first = false; return; }
      saveSettings(val);
      invoke("set_topmost", { value: val.topmost }).catch(() => {});
    });

    await listen('tauri://drag-enter', (e) => {
      dragging = true;
      const paths = e.payload.paths || [];
      validDrag = paths.every(p => /\.(lua|txt)$/i.test(p));
      if (e.payload.position) {
        const dpr = window.devicePixelRatio || 1;
        mouseX = e.payload.position.x / dpr;
        mouseY = e.payload.position.y / dpr;
      }
      updateDrop();
    });

    await listen('tauri://drag-over', (e) => {
      if (!dragging) {
        dragging = true;
        const paths = e.payload.paths || [];
        validDrag = paths.every(p => /\.(lua|txt)$/i.test(p));
      }
      if (e.payload.position) {
        const dpr = window.devicePixelRatio || 1;
        mouseX = e.payload.position.x / dpr;
        mouseY = e.payload.position.y / dpr;
        updateDrop();
      }
    });

    await listen('tauri://drag-drop', async (e) => {
      const paths = e.payload.paths || [];
      const target = dropTarget;
      const ok = validDrag;
      reset();
      if (ok && target) {
        for (const p of paths) {
          await invoke("copy", { sourcePath: p, targetFolder: target, fileName: p.split(/[\\/]/).pop() });
        }
        await refresh();
        expanded[target] = await invoke("files", { path: target });
      }
    });

    await listen('tauri://drag-leave', reset);
  });

  function reset() { dragging = false; validDrag = false; dropTarget = null; }

  function updateDrop() {
    if (!dragging || !validDrag) { dropTarget = null; return; }
    for (const [path, el] of Object.entries(folderEls)) {
      if (el) {
        const r = el.getBoundingClientRect();
        if (mouseX >= r.left && mouseX <= r.right && mouseY >= r.top && mouseY <= r.bottom) {
          dropTarget = path; return;
        }
      }
    }
    dropTarget = null;
  }

  async function refresh() {
    folders = await invoke("folders");
    for (const path in expanded) {
      if (expanded[path]) expanded[path] = await invoke("files", { path });
    }
  }

  $effect(() => {
    if (query.trim()) invoke("search", { query }).then(r => results = r);
    else results = [];
  });

  async function toggle(folder) {
    expanded[folder.path] = expanded[folder.path] ? null : await invoke("files", { path: folder.path });
  }

  async function open(file) {
    const content = await invoke("read", { path: file.path });
    openFile = { name: file.name, content, path: file.path };
  }

  function ctx(e, item) {
    e.preventDefault();
    e.stopPropagation();
    const h = item.is_folder ? 160 : 120;
    const up = window.innerHeight - e.clientY < h;
    const opts = item.is_folder
      ? [
          { label: expanded[item.path] ? "Close" : "Open", action: () => toggle(item), disabled: false },
          { label: "New File", action: () => newFile(item), disabled: false },
          { label: "Rename", action: () => renaming = item.path, disabled: item.is_root },
          { label: "Delete", action: () => del(item), disabled: item.is_root }
        ]
      : [
          { label: "Open", action: () => open(item), disabled: false },
          { label: "Rename", action: () => renaming = item.path, disabled: false },
          { label: "Delete", action: () => del(item), disabled: false }
        ];
    ctxMenu = { x: e.clientX, y: up ? e.clientY - h : e.clientY, options: opts };
  }

  function newFile(folder) {
    prompt = {
      title: "New File", input: true, placeholder: "newfile", defaultValue: "newfile", isFile: true, defaultExt: ".lua",
      onconfirm: async (name, ext) => {
        if (name) { await invoke("create", { path: folder.path, name: name + ext }); await refresh(); }
        prompt = null;
      },
      oncancel: () => prompt = null
    };
  }

  async function ren(item, newName) {
    if (newName && newName !== item.name) {
      await invoke("rename", { oldPath: item.path, newName });
      await refresh();
    }
    renaming = null;
  }

  function del(item) {
    prompt = {
      title: "Delete", message: `Delete "${item.name}"?`,
      onconfirm: async () => { await invoke("delete", { path: item.path }); await refresh(); prompt = null; },
      oncancel: () => prompt = null
    };
  }

  function onmenu(action, data) {
    if (action === "explorer") show = !show;
    if (action === "openfile" && data) openFile = data;
    if (action === "news") news = !news;
  }

  async function doImport() {
    importP = false;
    try {
      const data = await invoke('import_theme');
      // only update pending in themes, NOT the DOM
      if (data?.colors) themeDoImport(data.colors);
    } catch (e) { console.error(e); }
  }

  async function doExport(name, author) {
    const n = (name ?? '').trim(), a = (author ?? '').trim();
    if (!n) return;
    try {
      await invoke('export_theme', { name: n, author: a, colors: themeExportSnap() });
      exportP = false;
    } catch (e) { console.error(e); }
  }
</script>

<svelte:window
  onkeydown={(e) => {
    if (e.key === 'Escape' && renaming) renaming = null;
    if (e.key === 'Escape' && news) news = false;
    if (e.key === 'Escape' && previewColors !== null) previewColors = null;
  }}
  onmousedown={click}
/>

{#if loading}
  <Loader ondone={() => loading = false} />
{/if}

{#if dragging && !validDrag}
  <div class="fixed inset-0 backdrop-blur-sm z-[9999] flex items-center justify-center pointer-events-none" style="background:var(--dragdrop-overlay-bg)">
    <div class="rounded-md p-5 text-center" style="background:var(--dragdrop-card-bg)">
      <h2 class="text-xl font-bold mb-2" style="color:var(--dragdrop-title)">Invalid File Type</h2>
      <p class="text-[15px]" style="color:var(--dragdrop-desc)">Only .lua and .txt files are allowed</p>
    </div>
  </div>
{/if}

{#if previewColors !== null}
  <Prompt title="Theme Preview" preview={true} slides={previewSlides} previewColors={previewColors} oncancel={() => previewColors = null} />
{/if}

{#if importP}
  <Prompt title="Import Theme" message="Select a valid .json theme file from your Themes folder."
    onconfirm={doImport} oncancel={() => importP = false} />
{/if}

{#if exportP}
  <Prompt title="Export Theme" dual={true}
    placeholder1="Theme name (max 32 chars)" placeholder2="Author (max 20 chars, optional)"
    onconfirm={(n, a) => doExport(n, a)} oncancel={() => exportP = false} />
{/if}

{#if resetP}
  <Prompt title="Reset Theme" message="Reset all colors to the default theme. Are you sure?"
    onconfirm={() => { themeDoReset(); resetP = false; }}
    oncancel={() => resetP = false} />
{/if}

<Leftbar onpage={(p) => activePage = p} {activePage} onnews={() => news = !news} />
<Topbar {onmenu} getTab={() => selectedTabData} />

{#if ctxMenu}
  <Context {...ctxMenu} onclose={() => ctxMenu = null} />
{/if}

{#if prompt}
  <Prompt {...prompt} />
{/if}

{#if news}
  <News onclose={() => news = false} />
{/if}

<div class="fixed inset-0" style="top:62px;left:64px">
  <div class="w-full h-full overflow-hidden">

    <div class:hidden={activePage !== 'editor'} class="w-full h-full flex overflow-hidden">
      {#if show}
        <div class="w-50 min-w-50 flex flex-col" style="height:100%;background:var(--explorer-bg)">
          <div class="p-3 pb-3 flex flex-col gap-4">
            <Search bind:value={query} />
            <div class="font-bold text-[15px]" style="color:var(--explorer-label)">EXPLORER</div>
          </div>
          <div class="flex flex-col gap-2 px-3 overflow-y-auto" style="flex:1;scrollbar-width:thin;scrollbar-color:var(--scrollbar-thumb) transparent">
            {#if query.trim()}
              {#each results as item}
                <button onclick={() => open(item)} class="w-full">
                  <Scriptitem name={item.name} luafile={!item.is_folder} icon={item.is_folder ? "/file.svg" : "/luafile.svg"} />
                  <div class="text-xs px-2 truncate" style="color:var(--explorer-result-path)">{item.path}</div>
                </button>
              {/each}
            {:else}
              {#each folders as folder}
                <div bind:this={folderEls[folder.path]} class="{dropTarget === folder.path ? 'rounded-md' : ''}" style="{dropTarget === folder.path ? 'background:var(--explorer-droptarget-bg)' : ''}">
                  <button onclick={() => toggle(folder)} oncontextmenu={(e) => ctx(e, folder)} class="w-full rounded-md" style={expanded[folder.path] ? 'background:var(--folder-expanded-bg)' : ''}>
                    <Scriptitem
                      name={folder.name} icon="/file.svg"
                      expanded={expanded[folder.path]}
                      renaming={renaming === folder.path}
                      onrename={(n) => ren(folder, n)}
                      ondoubleclickrename={folder.is_root ? null : () => renaming = folder.path}
                      isDropTarget={dropTarget === folder.path}
                    />
                  </button>
                  {#if expanded[folder.path]}
                    <div class="pl-5 flex flex-col mt-0 relative">
                      <div class="absolute left-3 top-0 bottom-0 w-[3px]" style="background:var(--explorer-tree-line)"></div>
                      {#each expanded[folder.path] as file}
                        <button onclick={() => open(file)} oncontextmenu={(e) => ctx(e, file)} class="mt-2 w-full">
                          <Scriptitem
                            name={file.name} luafile={true} icon="/luafile.svg"
                            renaming={renaming === file.path}
                            onrename={(n) => ren(file, n)}
                            ondoubleclickrename={() => renaming = file.path}
                          />
                        </button>
                      {/each}
                    </div>
                  {/if}
                </div>
              {/each}
            {/if}
          </div>

          <div class="flex items-center justify-center gap-5 py-3 overflow-hidden">
            <button class="rounded-md flex items-center font-semibold gap-2 p-2"
              style="background:var(--open-bg);color:var(--open-text)"
              onmouseenter={(e)=>e.currentTarget.style.background='var(--open-hover)'}
              onmouseleave={(e)=>e.currentTarget.style.background='var(--open-bg)'}>
              <svg viewBox="0 0 13 14" class="size-5.5" fill="var(--open-icon)" xmlns="http://www.w3.org/2000/svg">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M1.61 0 H11.39 V4.53 L6.5 7.87 L1.61 4.52 Z M5.04 4.22 H7.95 V5.47 H5.04 Z M5.04 1.61 H7.95 V2.86 H5.04 Z"/>
                <path d="M0 5.25 L6.5 9.5 L13 5.25 V13 H0 Z"/>
              </svg>
              Open
            </button>
            <button class="rounded-md flex items-center font-semibold gap-2 p-2"
              style="background:var(--save-bg);color:var(--save-text)"
              onmouseenter={(e)=>e.currentTarget.style.background='var(--save-hover)'}
              onmouseleave={(e)=>e.currentTarget.style.background='var(--save-bg)'}>
              <svg class="size-5.5" viewBox="0 0 13 13" fill="var(--save-icon)" xmlns="http://www.w3.org/2000/svg">
                <path fill-rule="evenodd" clip-rule="evenodd" d="M0 7.4021H3.60253L5.22318 9.33375H7.77682L9.39747 7.4021H13V13H0Z"/>
                <path fill-rule="evenodd" clip-rule="evenodd" d="M5.65481 0H7.345V4.2L9.5 4.2L6.5 6.9L3.5 4.2L5.65481 4.2V0Z"/>
              </svg>
              Save
            </button>
          </div>
        </div>
      {/if}
      <div class="flex-1 h-full overflow-hidden" style="background:var(--editor-canvas-bg)">
        <Editor bind:file={openFile} bind:selectedTabData />
      </div>
    </div>

    <div class:hidden={activePage !== 'settings'} class="w-full h-full overflow-hidden">
      <Settings
        onpreview={(c) => previewColors = c}
        onimport={() => importP = true}
        onexport={(snapFn) => { themeExportSnap = snapFn; exportP = true; }}
        onreset={() => resetP = true}
        bindreset={(fn) => themeDoReset = fn}
        bindimport={(fn) => themeDoImport = fn}
      />
    </div>

    <div class:hidden={activePage !== 'cloud'} class="w-full h-full overflow-hidden">
      <Cloud />
    </div>
  </div>
</div>

<style>
  :global(html) { background-color: var(--bg-app); }
</style>