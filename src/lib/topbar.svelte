<script> import { Window } from "@tauri-apps/api/window";
  export async function close() {
    await Window.getCurrent().close();
  }

  export async function minimize() {
    await Window.getCurrent().minimize();
  }

  export async function maximize() {
    await Window.getCurrent().toggleMaximize();
  }
</script>
<div
  data-tauri-drag-region
  class="w-full bg-[#111111] flex items-center justify-between text-white pl-4 fixed py-2 rounded-5 ml-16 pr-19 border-b-[1.5px] border-[#A8E8EB]/12.5"
>
  <div class="flex items-center gap-4">
    <div
      class="flex rounded-[15px] hover:bg-white/6.25 p-2 gap-4 font-[450] items-center"
    >
      <img alt="lua" src="/lua.svg" class="size-7" />
      Folder2
      <img alt="arrowdown" src="/arrowdown.svg" class="size-3.5" />
    </div>
    <div
      class="flex rounded-[15px] hover:bg-white/6.25 p-2 gap-4 font-[450] items-center"
    >
      <img alt="lua" src="/branch.svg" class="size-7" />
      Folder2
      <img alt="arrowdown" src="/arrowdown.svg" class="size-3.5" />
    </div>
  </div>

  <div class="flex items-center gap-3 justify-center">
    <button class="p-3 rounded-md py-4.5 hover:bg-white/12.5" onclick={minimize}>
      <img alt="x" src="/min.svg" />
    </button>
    <button class="p-3 rounded-md hover:bg-white/12.5" onclick={maximize}>
      <img alt="x" src="/max.svg" />
    </button>
    <button class="p-3 rounded-md hover:bg-white/12.5" onclick={close}>
      <img alt="x" src="/x.svg" />
    </button>
  </div>
</div>
