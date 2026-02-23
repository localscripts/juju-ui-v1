<script>
  import Item from './item.svelte';
  import { s, saveSettings } from '../data.ts';

  let { query = "", filters = [] } = $props();

  const icon = `<g transform="scale(0.82) translate(10, 10)"><g transform="translate(-6,-6)"><path d="M 5,5 L 55,5 L 55,22 C 43,22 43,38 55,38 L 55,55 L 38,55 C 38,67 22,67 22,55 L 5,55 Z"/></g><g transform="translate(6,-6)"><path d="M 55,5 L 105,5 L 105,55 L 88,55 C 88,43 72,43 72,55 L 55,55 L 55,38 C 43,38 43,22 55,22 Z"/></g><g transform="translate(-6,6)"><path d="M 5,55 L 22,55 C 22,67 38,67 38,55 L 55,55 L 55,72 C 67,72 67,88 55,88 L 55,105 L 5,105 Z"/></g><g transform="translate(6,6)"><path d="M 55,55 L 72,55 C 72,43 88,43 88,55 L 105,55 L 105,105 L 55,105 L 55,88 C 67,88 67,72 55,72 Z"/></g></g>`;
  const iconVb = "0 0 110 110";

  function show(title, desc, type) {
    const q = query.toLowerCase();
    const matchQ = !q || title.toLowerCase().includes(q) || desc.toLowerCase().includes(q);
    const matchF = filters.length === 0 || filters.includes(type);
    return matchQ && matchF;
  }
</script>

<div class="w-full h-full flex flex-col">
  <div class="mx-3 mt-3 w-full h-[42.5px] flex items-center gap-5 font-[565] text-[15px] flex-shrink-0">
    <span style="color: var(--editor-title);">EDITOR SETTINGS</span>
    <span style="color: var(--editor-desc);">Customize the code editor</span>
  </div>
  <div class="flex-1 overflow-y-auto" style="scrollbar-width: thin; scrollbar-color: var(--scrollbar-thumb) transparent;">
    <div class="flex flex-wrap gap-3 p-3 content-start">
      {#if show("Minimap", "Show minimap in the editor", "checkbox")}
        <Item {icon} {iconVb} title="Minimap" desc="Show minimap in the editor" type="checkbox" bind:value={$s.minimap} onchange={saveSettings} />
      {/if}
      {#if show("Font Size", "Editor font size", "slider")}
        <Item {icon} {iconVb} title="Font Size" desc="Editor font size" type="slider" bind:value={$s.fontSize} min={1} max={30} step={1} onchange={saveSettings} />
      {/if}
      {#if show("LSP", "Language server support", "checkbox")}
        <div class="select-none opacity-50 pointer-events-none">
          <Item {icon} {iconVb} title="LSP" desc="Language server support" type="checkbox" value={false} />
        </div>
      {/if}
    </div>
  </div>
</div>