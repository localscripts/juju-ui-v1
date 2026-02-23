<script>
  import { Window } from "@tauri-apps/api/window";
  import { invoke } from "@tauri-apps/api/core";
  import Dropdown from "./dropdown.svelte";
  import Prompt from "./prompt.svelte";

  let { onmenu = () => {}, getTab = () => null } = $props();
  
  let fileOpen = $state(false);
  let viewOpen = $state(false);
  let helpOpen = $state(false);
  let prompt = $state(null);
  
  async function tryClose() {
    prompt = {
      title: "Exit",
      message: "Are you sure you want to exit?",
      onconfirm: async () => { prompt = null; await Window.getCurrent().close(); },
      oncancel: () => prompt = null
    };
  }

  async function minimize() { await Window.getCurrent().minimize(); }
  async function maximize() { await Window.getCurrent().toggleMaximize(); }

  async function openFile() {
    const result = await invoke("open_file_dialog");
    if (result) {
      const [name, content, path] = result;
      onmenu("openfile", { name, content, path });
    }
  }

  async function save() {
    const tab = getTab();
    if (!tab) return;
    await invoke("save_to_scripts", { name: tab.name, content: tab.content });
  }

  async function saveAs() {
    const tab = getTab();
    if (!tab) return;
    await invoke("save_file_dialog", { name: tab.name, content: tab.content });
  }

  async function discord() {}
</script>

<svelte:window 
  onclick={() => { fileOpen = false; viewOpen = false; helpOpen = false; }}
/>

{#if prompt}
  <Prompt {...prompt} />
{/if}

<div data-tauri-drag-region class="w-full flex items-center justify-between text-white pl-4 fixed py-2 rounded-5 ml-16 pr-19 h-[62px] overflow-visible z-50" style="background: var(--topbar-bg);">
  <div data-tauri-drag-region class="flex items-center pointer-events-none gap-4">
    <img data-tauri-drag-region src="nigger.svg" class="select-none shrink-0 h-35 w-35 mt-3 ml-1"/>
    
    <div data-tauri-drag-region class="flex items-center gap-0 overflow-visible" style="pointer-events: auto;">
      <div class="relative overflow-visible">
        <button 
          onclick={(e) => { e.stopPropagation(); fileOpen = !fileOpen; viewOpen = false; helpOpen = false; }}
          class="px-3 py-1.5 font-[565] rounded"
          style="color: var(--topbar-menu-text); background: var(--topbar-menu-bg);"
          onmouseenter={(e) => { e.currentTarget.style.background = 'var(--topbar-menu-hover-bg)'; }}
          onmouseleave={(e) => { e.currentTarget.style.background = 'var(--topbar-menu-bg)'; }}
        >File</button>
        {#if fileOpen}
          <Dropdown 
            x={0} y={36}
            options={[
              { label: "Open File", action: openFile },
              { label: "Save", action: save },
              { label: "Save as", action: saveAs },
              { separator: true },
              { label: "Exit", action: tryClose }
            ]}
            onclose={() => fileOpen = false}
          />
        {/if}
      </div>
      
      <div class="relative overflow-visible">
        <button 
          onclick={(e) => { e.stopPropagation(); viewOpen = !viewOpen; fileOpen = false; helpOpen = false; }}
          class="px-3 py-1.5 font-[565] rounded"
          style="color: var(--topbar-menu-text); background: var(--topbar-menu-bg);"
          onmouseenter={(e) => { e.currentTarget.style.background = 'var(--topbar-menu-hover-bg)'; }}
          onmouseleave={(e) => { e.currentTarget.style.background = 'var(--topbar-menu-bg)'; }}
        >View</button>
        {#if viewOpen}
          <Dropdown 
            x={0} y={36}
            options={[
              { label: "Explorer", action: () => onmenu("explorer") },
              { separator: true },
              { label: "Execution", action: () => onmenu("execution") },
              { label: "Script Hub", action: () => onmenu("scripthub") },
              { label: "Settings", action: () => onmenu("settings") }
            ]}
            onclose={() => viewOpen = false}
          />
        {/if}
      </div>
      
      <div class="relative overflow-visible">
        <button 
          onclick={(e) => { e.stopPropagation(); helpOpen = !helpOpen; fileOpen = false; viewOpen = false; }}
          class="px-3 py-1.5 font-[565] rounded"
          style="color: var(--topbar-menu-text); background: var(--topbar-menu-bg);"
          onmouseenter={(e) => { e.currentTarget.style.background = 'var(--topbar-menu-hover-bg)'; }}
          onmouseleave={(e) => { e.currentTarget.style.background = 'var(--topbar-menu-bg)'; }}
        >Help</button>
        {#if helpOpen}
          <Dropdown 
            x={0} y={36}
            options={[{ label: "Discord", action: () => { invoke("plugin:opener|open_url", { url: "https://discord.gg/Fb3fQXcv6y" }) } }]}
            onclose={() => helpOpen = false}
          />
        {/if}
      </div>
    </div>
  </div>

  <div class="flex items-center gap-3 justify-center">
    <button
      class="p-3 select-none rounded-md py-4.5"
            style="background: var(--topbar-btn-bg);"

      onmouseenter={(e) => e.currentTarget.style.background = 'var(--topbar-btn-hover)'}
      onmouseleave={(e) => e.currentTarget.style.background = 'var(--topbar-btn-bg)'}
      onclick={minimize}
    >
      <svg width="14" height="5" viewBox="0 0 14 3" fill="none">
        <path d="M1.3125 0.150391H12.6875C12.9889 0.150391 13.2829 0.286617 13.5029 0.538086C13.7235 0.790186 13.8496 1.13602 13.8496 1.5C13.8496 1.86398 13.7235 2.20981 13.5029 2.46191C13.2829 2.71338 12.9889 2.84961 12.6875 2.84961H1.3125C1.01114 2.84961 0.717105 2.71338 0.49707 2.46191C0.27653 2.20981 0.150391 1.86398 0.150391 1.5C0.150391 1.13602 0.27653 0.790186 0.49707 0.538086C0.717105 0.286617 1.01114 0.150391 1.3125 0.150391Z" fill="var(--topbar-icon)" stroke-width="0.3"/>
      </svg>
    </button>
    <button
      class="p-3 select-none rounded-md"
      style="background: var(--topbar-btn-bg);"

      onmouseenter={(e) => e.currentTarget.style.background = 'var(--topbar-btn-hover)'}
      onmouseleave={(e) => e.currentTarget.style.background = 'var(--topbar-btn-bg)'}
      onclick={maximize}
    >
      <svg width="14" height="14" viewBox="0 0 13 13" fill="none">
        <path d="M1 3.75V2.375C1 2.01033 1.14487 1.66059 1.40273 1.40273C1.66059 1.14487 2.01033 1 2.375 1H3.75M9.25 1H10.625C10.9897 1 11.3394 1.14487 11.5973 1.40273C11.8551 1.66059 12 2.01033 12 2.375V3.75M12 9.25V10.625C12 10.9897 11.8551 11.3394 11.5973 11.5973C11.3394 11.8551 10.9897 12 10.625 12H9.25M3.75 12H2.375C2.01033 12 1.66059 11.8551 1.40273 11.5973C1.14487 11.3394 1 10.9897 1 10.625V9.25" stroke="var(--topbar-icon)" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
    </button>
    <button
      class="p-3 select-none rounded-md"
      style="background: var(--topbar-btn-bg);"
      onmouseenter={(e) => e.currentTarget.style.background = 'var(--topbar-btn-hover)'}
      onmouseleave={(e) => e.currentTarget.style.background = 'var(--topbar-btn-bg)'}
      onclick={tryClose}
    >
      <svg width="13" height="13" viewBox="0 0 13 13" fill="none">
        <path fill-rule="evenodd" clip-rule="evenodd" d="M0.316867 0.330729C0.519817 0.127635 0.795041 0.0135436 1.08201 0.0135436C1.36899 0.0135436 1.64421 0.127635 1.84716 0.330729L6.49323 4.98148L11.1393 0.330729C11.2391 0.22726 11.3586 0.144729 11.4906 0.0879524C11.6226 0.031176 11.7647 0.00129087 11.9084 4.09035e-05C12.0521 -0.00120906 12.1946 0.0262009 12.3276 0.0806718C12.4606 0.135143 12.5814 0.215584 12.683 0.317301C12.7846 0.419018 12.865 0.539974 12.9194 0.673112C12.9738 0.80625 13.0012 0.948903 13 1.09275C12.9987 1.23659 12.9689 1.37875 12.9121 1.51092C12.8554 1.64309 12.773 1.76263 12.6696 1.86256L8.02353 6.51331L12.6696 11.1641C12.8667 11.3684 12.9758 11.642 12.9734 11.9261C12.9709 12.2101 12.8571 12.4818 12.6564 12.6827C12.4558 12.8836 12.1843 12.9975 11.9006 13C11.6168 13.0024 11.3434 12.8932 11.1393 12.6959L6.49323 8.04515L1.84716 12.6959C1.64305 12.8932 1.36967 13.0024 1.08591 13C0.802147 12.9975 0.530707 12.8836 0.33005 12.6827C0.129393 12.4818 0.0155741 12.2101 0.0131083 11.9261C0.0106425 11.642 0.119727 11.3684 0.316867 11.1641L4.96294 6.51331L0.316867 1.86256C0.113977 1.65941 0 1.38391 0 1.09665C0 0.809384 0.113977 0.533884 0.316867 0.330729Z" fill="var(--topbar-icon)"/>
      </svg>
    </button>
  </div>
</div>