<script>
  let { icon, name, luafile = false, selected = false } = $props();
</script>

<div
  class={`w-full  ${selected ? "bg-[#01B8BA]/6.25 text-[#01B8BA] hover:bg-[#2ce8ec]/16" : "text-white/50 hover:bg-white/12.5"} p-2 rounded-md font-[565] flex gap-3`}
>
  {#if !luafile}
    <img class="w-4.5" alt="arrowdown" src="/arrowdown.svg" />{/if}

  <img class="w-5" alt="file" src={icon} />
  {name}
</div>
