<script>
  import Item from './item.svelte';
  import Prompt from '../prompt.svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { get } from 'svelte/store';
  import { s, exportSettings, importSettings } from '../data.ts';

  let { query = "", filters = [] } = $props();

  const icon = `<path d="M17.5 1.25C17.771 1.25 18.0208 1.39615 18.1537 1.6323L22.6537 9.6323C22.7843 9.86453 22.7819 10.1486 22.6474 10.3786C22.5129 10.6086 22.2664 10.75 22 10.75H13C12.7336 10.75 12.4871 10.6086 12.3526 10.3786C12.2181 10.1486 12.2157 9.86453 12.3463 9.6323L16.8463 1.6323C16.9792 1.39615 17.229 1.25 17.5 1.25Z"/><path d="M1.25 2C1.25 1.58579 1.58579 1.25 2 1.25H10C10.4142 1.25 10.75 1.58579 10.75 2V10C10.75 10.4142 10.4142 10.75 10 10.75H2C1.58579 10.75 1.25 10.4142 1.25 10V2Z"/><path d="M12.75 18C12.75 15.3766 14.8766 13.25 17.5 13.25C20.1234 13.25 22.25 15.3766 22.25 18C22.25 20.6234 20.1234 22.75 17.5 22.75C14.8766 22.75 12.75 20.6234 12.75 18Z"/><path fill-rule="evenodd" clip-rule="evenodd" d="M5.95711 16.6286L9.24994 13.3358L10.6642 14.75L7.37132 18.0429L10.6642 21.3358L9.25 22.75L5.95711 19.4571L2.66422 22.75L1.25 21.3358L4.5429 18.0429L1.25007 14.75L2.66428 13.3358L5.95711 16.6286Z"/>`;

  let confirmImport = $state(false);

  function show(title, desc, type) {
    const q = query.toLowerCase();
    const matchQ = !q || title.toLowerCase().includes(q) || desc.toLowerCase().includes(q);
    const matchF = filters.length === 0 || filters.includes(type);
    return matchQ && matchF;
  }
</script>

{#if confirmImport}
  <Prompt
    title="Import Settings"
    message="This will overwrite your current settings. Are you sure?"
    onconfirm={async () => { await importSettings(); confirmImport = false; }}
    oncancel={() => confirmImport = false}
  />
{/if}

<div class="w-full h-full flex flex-col">
  <div class="mx-3 mt-3 w-full h-[42.5px] flex items-center gap-5 font-[565] text-[15px] flex-shrink-0">
    <span style="color: var(--core-title);">CORE SETTINGS</span>
    <span style="color: var(--core-desc);">Configure app behavior and updates</span>
  </div>
  <div class="flex-1 overflow-y-auto" style="scrollbar-width: thin; scrollbar-color: var(--scrollbar-thumb) transparent;">
    <div class="flex flex-wrap gap-3 p-3 content-start">
      {#if show("Auto Inject", "Automatically inject to Roblox", "checkbox")}
        <Item {icon} title="Auto Inject" desc="Automatically inject to Roblox" type="checkbox" bind:value={$s.autoInject} />
      {/if}
      {#if show("Save Tabs", "Restore tabs on next launch", "checkbox")}
        <Item {icon} title="Save Tabs" desc="Restore tabs on next launch" type="checkbox" bind:value={$s.saveTabs} />
      {/if}
      {#if show("Auto Update", "Download updates automatically", "checkbox")}
        <Item {icon} title="Auto Update" desc="Download updates automatically" type="checkbox" bind:value={$s.autoUpdate} />
      {/if}
      {#if show("Save Settings", "Persist settings between sessions", "checkbox")}
        <Item {icon} title="Save Settings" desc="Persist settings between sessions" type="checkbox" bind:value={$s.saveSettings} />
      {/if}
      {#if show("Export Config", "Save your settings to a file", "button")}
        <Item {icon} title="Export Config" desc="Save your settings to a file" type="button" value="Export" onclick={() => exportSettings(get(s))} />
      {/if}
      {#if show("Import Config", "Load settings from a file", "button")}
        <Item {icon} title="Import Config" desc="Load settings from a file" type="button" value="Import" onclick={() => confirmImport = true} />
      {/if}
      {#if show("Update", "Check for a new version", "button")}
        <Item {icon} title="Update" desc="Check for a new version" type="button" value="Check" onclick={() => {}} />
      {/if}
      {#if show("Help", "Join our Discord server for support", "button")}
        <Item {icon} title="Help" desc="Join our Discord server for support" type="button" value="Open" onclick={() => invoke("plugin:opener|open_url", { url: "https://discord.gg/Fb3fQXcv6y" })} />
      {/if}
    </div>
  </div>
</div>