<script>
  import { invoke } from "@tauri-apps/api/core";
  
  let { value = $bindable("") } = $props();
  
  async function openDir() {
    await invoke("open_dir");
  }
</script>

<div class="w-full flex items-center gap-2">
  <div class="flex-1 flex items-center gap-3 text-[15px] font-[565] text-white/50 p-2.5 px-4 rounded-md bg-[#242424]">
    <img alt="search" class="size-4.5" src="/search.svg" />
    <input bind:value class="w-full bg-transparent outline-none" placeholder="Search for Files..." />
  </div>
  
  <button onclick={openDir} class="bg-[#242424] hover:bg-[#323232] duration-75 flex items-center justify-center rounded-md aspect-square h-[42px]">
    <img alt="open" class="size-5" src="/file.svg" />
  </button>
</div>