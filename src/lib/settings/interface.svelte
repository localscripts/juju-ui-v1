<script>
  import Item from './item.svelte';
  import { s, saveSettings } from '../data.ts';
  import { invoke } from '@tauri-apps/api/core';

  let { query = "", filters = [], ongoto } = $props();

  const icon = `<path d="M3 2.25C2.58579 2.25 2.25 2.58579 2.25 3V8H21.75V3C21.75 2.58579 21.4142 2.25 21 2.25H3Z"/><path d="M21.75 10H13V21.75H21C21.4142 21.75 21.75 21.4142 21.75 21V10Z"/><path d="M11 21.75V10H2.25V21C2.25 21.4142 2.58579 21.75 3 21.75H11Z"/>`;

  let themeOptions = $state(['Default']);
  let ddOpen = $state(false);
  let pollInterval;

  async function loadThemes() {
    try {
      const list = await invoke('list_themes');
      themeOptions = ['Default', ...list.map(t => t.name)];
    } catch {
      themeOptions = ['Default'];
    }
  }

  async function watchThemes() {
    await loadThemes();
    pollInterval = setInterval(async () => {
      const prev = [...themeOptions];
      await loadThemes();
      const cur = $s.theme;
      if (cur !== 'Default' && !themeOptions.includes(cur)) {
        $s.theme = 'Default';
        resetToDefault();
        saveSettings($s);
      }
    }, 2000);
  }

  $effect(() => {
    watchThemes();
    return () => clearInterval(pollInterval);
  });

  function resetToDefault() {
    const root = document.documentElement;
    const toRemove = [];
    for (const prop of root.style) toRemove.push(prop);
    for (const p of toRemove) root.style.removeProperty(p);
  }

  async function onThemeChange(val) {
    if (val === 'Default') {
      await invoke('unset_theme').catch(() => {});
      resetToDefault();
    } else {
      try {
        const data = await invoke('load_theme', { name: val });
        if (data?.colors) {
          for (const [k, v] of Object.entries(data.colors)) {
            document.documentElement.style.setProperty(k, v);
          }
        }
      } catch (e) {
        console.error('Failed to load theme:', e);
        $s.theme = 'Default';
        saveSettings($s);
      }
    }
  }

  $effect(() => {
    if ($s.theme && $s.theme !== 'Default') {
      onThemeChange($s.theme);
    }
  });

  function show(title, desc, type) {
    const q = query.toLowerCase();
    const matchQ = !q || title.toLowerCase().includes(q) || desc.toLowerCase().includes(q);
    const matchF = filters.length === 0 || filters.includes(type);
    return matchQ && matchF;
  }

  function save() {
    saveSettings($s);
  }
</script>

<div class="w-full h-full flex flex-col">
  <div class="mx-3 mt-3 w-full h-[42.5px] flex items-center gap-5 font-[565] text-[15px] flex-shrink-0">
    <span style="color: var(--interface-title);">INTERFACE SETTINGS</span>
    <span style="color: var(--interface-desc);">Customize appearance and layout</span>
  </div>
  <div class="flex-1 overflow-y-auto" style="scrollbar-width: thin; scrollbar-color: var(--scrollbar-thumb) transparent;">
    <div class="flex flex-wrap gap-3 p-3 content-start">
      {#if show("TopMost", "Keep window above all others", "checkbox")}
        <Item {icon} title="TopMost" desc="Keep window above all others" type="checkbox" bind:value={$s.topmost} onchange={save} />
      {/if}
      {#if show("Sound Effects", "Play audio on click interactions", "checkbox")}
        <Item {icon} title="Sound Effects" desc="Play audio on click interactions" type="checkbox" bind:value={$s.sounds} onchange={save} />
      {/if}
      {#if show("Language", "Set the display language", "dropdown")}
        <div class="select-none opacity-50 pointer-events-none">
          <Item {icon} title="Language" desc="Set the display language" type="dropdown" bind:value={$s.lang} options={["English", "French"]} onchange={save} />
        </div>
      {/if}
      {#if show("Theme", "Preferred color theme", "dropdown")}
        <Item
          {icon}
          title="Theme"
          desc="Preferred color theme"
          type="dropdown"
          bind:value={$s.theme}
          options={themeOptions}
          onchange={(val) => { save(); onThemeChange(val ?? $s.theme); }}
          onddopen={loadThemes}
          scrollable={true}
        />
      {/if}
    </div>
  </div>
</div>