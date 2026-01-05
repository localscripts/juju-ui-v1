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

      <div
        class="text-[#2CB0CD] text-nowrap items-center justify-center font-semibold flex gap-4 h-[44px]"
      >
        Currently Attached
        <div
          class="size-6 flex items-center justify-center rounded-md bg-[#2CB0CD]"
        >
          <img alt="attached" class="size-5" src="/attached.svg" />
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
