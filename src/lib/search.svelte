<script>
  import { invoke } from "@tauri-apps/api/core";
  let { value = $bindable(""), filterOpen = $bindable(false), filters = $bindable([]), toggleFilter = () => {}, filterOpts = [], showFilters = false } = $props();
  async function openDir() { await invoke("open_dir"); }
</script>

<div class="w-full flex items-center gap-2">
  <div class="flex-1 flex items-center gap-3 text-[15px] font-[565] p-2.5 px-3 rounded-md"
    style="color: var(--search-input-text); background: var(--search-input-bg);"
    onmouseenter={(e) => e.currentTarget.style.background = 'var(--search-input-bg-hover)'}
    onmouseleave={(e) => e.currentTarget.style.background = 'var(--search-input-bg)'}
  >
    <svg xmlns="http://www.w3.org/2000/svg" class="size-5 flex-shrink-0" viewBox="0 0 24 24" fill="var(--search-input-icon)">
      <path fill-rule="evenodd" clip-rule="evenodd" d="M10.7498 1.75C5.77919 1.75 1.74976 5.77944 1.74976 10.75C1.74976 15.7206 5.77919 19.75 10.7498 19.75C12.8747 19.75 14.8277 19.0135 16.3674 17.7819L20.8355 22.25L22.2498 20.8358L17.7816 16.3677C19.0133 14.8281 19.7498 12.875 19.7498 10.75C19.7498 5.77944 15.7203 1.75 10.7498 1.75ZM3.74976 10.75C3.74976 6.88401 6.88376 3.75 10.7498 3.75C14.6157 3.75 17.7498 6.88401 17.7498 10.75C17.7498 14.616 14.6157 17.75 10.7498 17.75C6.88376 17.75 3.74976 14.616 3.74976 10.75Z"></path>
    </svg>
    <input bind:value class="w-full  outline-none" style=" color: var(--search-input-text);" placeholder="Search..." />
  </div>

  <button onclick={openDir} class="flex items-center justify-center rounded-md aspect-square h-[42px]"
    style="background: var(--search-opendir-bg);"
    onmouseenter={(e) => e.currentTarget.style.background = 'var(--search-opendir-bg-hover)'}
    onmouseleave={(e) => e.currentTarget.style.background = 'var(--search-opendir-bg)'}
  >
    <svg class="size-6" viewBox="0 0 24 24" fill="none">
      <path d="M18.25 14.5a3.75 3.75 0 0 1 .202 7.495l-.199.005-1 .005a.75.75 0 0 1-.108-1.493l.102-.007 1.003-.005a2.25 2.25 0 0 0 .154-4.495L18.25 16h-1a.75.75 0 0 1-.102-1.493l.102-.007h1Zm-4.5 0a.75.75 0 0 1 .102 1.493L13.75 16h-1a2.25 2.25 0 0 0-.154 4.495l.154.005h1a.75.75 0 0 1 .102 1.493L13.75 22h-1a3.75 3.75 0 0 1-.2-7.495l.2-.005h1Zm6-8a2.25 2.25 0 0 1 2.229 1.938l.016.158.005.154v6.585a4.742 4.742 0 0 0-3.75-1.835h-5.5a4.75 4.75 0 0 0-4.417 6.501L4.25 20a2.25 2.25 0 0 1-2.245-2.096L2 17.75v-7.251l6.207.001.196-.009a2.25 2.25 0 0 0 1.088-.393l.156-.12L13.821 6.5h5.929Zm-1.5 11a.75.75 0 0 1 .102 1.493L18.25 19h-5.5a.75.75 0 0 1-.102-1.493l.102-.007h5.5ZM8.207 4c.46 0 .908.141 1.284.402l.156.12 2.103 1.751-3.063 2.553-.085.061a.75.75 0 0 1-.29.106L8.206 9 2 8.999V6.25a2.25 2.25 0 0 1 2.096-2.245L4.25 4h3.957Z" fill="var(--search-opendir-icon)"/>
    </svg>
  </button>
</div>