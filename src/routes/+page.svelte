<script>
  import Leftbar from "../lib/leftbar.svelte";
  import Scriptitem from "../lib/scriptitem.svelte";
  import Search from "../lib/search.svelte";
  import Topbar from "../lib/topbar.svelte";
</script>

<Leftbar></Leftbar>
<Topbar></Topbar>

<div class="w-full fixed flex mt-15.5 ml-16 text-white h-full">
  <div
    class="h-full w-65 min-w-65 p-5 gap-4 border-r-[1.5px] border-[#A8E8EB]/12.5 bg-[#111111] flex flex-col"
  >
    <Search></Search>

    <h class="font-bold text-[15px] text-white/25">EXPLORER</h>
    <Scriptitem name="Infinite-Yield" icon="/file.svg"></Scriptitem>
    <div class="pl-3 gap-2 flex flex-col">
      <Scriptitem name="Infinite-Yield" icon="/file.svg"></Scriptitem>
      <Scriptitem name="ballz.lua" luafile={true} icon="/luafile.svg"
      ></Scriptitem>
      <Scriptitem
        name="ballz.lua"
        selected={true}
        luafile={true}
        icon="/luafileselected.svg"
      ></Scriptitem>
      <Scriptitem name="ballz.lua" luafile={true} icon="/luafile.svg"
      ></Scriptitem>
    </div>
  </div>

  <div
    class="w-full flex flex-col justify-between"
    style="height: calc(100% - 62px); width: calc(100% - 321px);"
  >
    <div
      class="w-full h-min py-3 flex items-center gap-5 px-4 bg-[#111111] border-b-[1.5px] border-[#A8E8EB]/12.5"
    >
      <div class="flex items-center gap-2">
        <button class="p-2 hover:bg-white/12.5 rounded-md"
          ><img
            class="size-3.75"
            alt="arrowleft"
            src="/arrowleft.svg"
          /></button
        >

        <button class="p-2 hover:bg-white/12.5 rounded-md"
          ><img
            class="size-3.75"
            alt="arrowright"
            src="/arrowright.svg"
          /></button
        >
      </div>

      <img alt="seperator" src="/seperator.svg" />

      <button
        class="p-2 hover:bg-white/12.5 text-[#FFFFFF]/50 font-medium w-42.5 px-2.5 justify-between flex items-center gap-2 bg-[#242424] rounded-md"
      >
        <div class="flex items-center gap-3.75">
          <img class="size-3.75" alt="arrowright" src="/text.svg" />
          Roblox.lua
        </div>

        <img alt="x" src="/x.svg" />
      </button>

      <button
        class="p-2 hover:bg-white/12.5 text-[#FFFFFF]/25 font-medium px-2.5 justify-between flex items-center gap-2 rounded-md"
      >
        <div class="flex items-center gap-3.75">
          <img class="size-3.75" alt="arrowright" src="/textunselected.svg" />
          Roblox.lua
        </div>
      </button>

      <button
        class="p-2 hover:bg-white/12.5 text-[#FFFFFF]/25 font-medium px-2.5 justify-between flex items-center gap-2 rounded-md"
      >
        <div class="flex items-center gap-3.75">
          <img class="size-3.75" alt="arrowright" src="/textunselected.svg" />
          Roblox.lua
        </div>
      </button>
    </div>

    <div
      class="w-full flex items-center justify-between gap-4 bg-[#111111] p-4 border-t-[1.5px] border-[#A8E8EB]/12.5"
    >
      <div class="flex gap-4">
        <button
          class="bg-[#242424] hover:bg-[#323232] duration-75 flex text-[#FFFFFF]/50 items-center font-semibold gap-3 rounded-md p-2"
        >
          <img alt="attach" class="size-5" src="/attach.svg" />
          Attach
        </button>

        <button
          class="bg-[#242424] hover:bg-[#323232] duration-75 flex text-[#FFFFFF]/50 items-center font-semibold gap-3 rounded-md p-2.5"
        >
          <img alt="attach" class="size-5" src="/openfile.svg" />
        </button>
        <button
          class="bg-[#242424] hover:bg-[#323232] duration-75 flex text-[#FFFFFF]/50 items-center font-semibold gap-3 rounded-md p-2.5"
        >
          <img alt="attach" class="size-5" src="/savefile.svg" />
        </button>
        <button
          class="bg-[#242424] hover:bg-[#323232] duration-75 flex text-[#FFFFFF]/50 items-center font-semibold gap-3 rounded-md p-2.5"
        >
          <img alt="attach" class="size-5" src="/openfile.svg" />
        </button>
        <button
          class="bg-[#242424] hover:bg-[#323232] duration-75 flex text-[#FFFFFF]/50 items-center font-semibold gap-3 rounded-md p-2"
        >
          <img alt="attach" class="size-5" src="/execute.svg" />
          Execute
        </button>
      </div>
      <div
        class="text-[#2CB0CD] text-nowrap items-center justify-center font-semibold flex gap-4"
      >
        Currently Attached
        <div
          class="size-6 flex items-center justify-center rounded-md bg-[#2CB0CD]"
        >
          <img alt="attached" class="size-5" src="/attached.svg" />
        </div>
      </div>
    </div>
  </div>
</div>

<style lang="postcss">
  @reference "tailwindcss";

  :global(html) {
    background-color: #0d0d0d;
  }
</style>
