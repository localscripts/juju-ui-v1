<script>
  import { get } from "svelte/store";
  import { s } from "./data.ts";

  let {
    title = "",
    message = "",
    input = false,
    dual = false,
    placeholder = "",
    placeholder1 = "",
    placeholder2 = "",
    defaultValue = "",
    onconfirm = () => {},
    oncancel = () => {},
    isFile = false,
    defaultExt = ".lua",
    preview = false,
    slides = [],
    previewColors = {}
  } = $props();

  let value = $state(defaultValue);
  let value1 = $state("");
  let value2 = $state("");
  let ext = $state(defaultExt);
  let extOpen = $state(false);
  let inputEl;
  let input1El;
  let slide = $state(0);

  const confirmSound = new Audio('/confirm.mp3');
  const cancelSound = new Audio('/cancel.mp3');

  function confirm(...args) {
    if (get(s).sounds) { confirmSound.currentTime = 0; confirmSound.play(); }
    onconfirm(...args);
  }

  function cancel() {
    if (get(s).sounds) { cancelSound.currentTime = 0; cancelSound.play(); }
    oncancel();
  }

  $effect(() => {
    if (input) { inputEl?.focus(); inputEl?.select(); }
    if (dual) { input1El?.focus(); }
  });
</script>

<svelte:window onclick={() => extOpen = false} oncontextmenu={() => extOpen = false} />

<div class="fixed inset-0 backdrop-blur-sm flex items-center justify-center z-[9999]" style="background:var(--prompt-overlay)" onclick={cancel}>
  <div class="rounded-md p-5 flex flex-col" style="background:var(--prompt-bg);{preview?'width:560px;height:420px':'min-width:384px;max-width:448px'}" onclick={(e)=>e.stopPropagation()}>

    <h2 class="text-xl font-[565] mb-4 flex-shrink-0" style="color:var(--prompt-title)">{title}</h2>

    {#if preview && slides.length}
      <div class="flex-1 rounded-md overflow-hidden mb-3" style="min-height:0">
        {#each slides as sl, i}
          <div class:hidden={slide!==i} class="w-full h-full">
            <svelte:component this={sl.component} colors={previewColors} />
          </div>
        {/each}
      </div>
      <div class="flex items-center justify-center gap-2 flex-shrink-0">
        <button onclick={()=>slide=Math.max(0,slide-1)} disabled={slide===0}
          class="w-7 h-7 rounded-md flex items-center justify-center font-bold text-sm"
          style="background:var(--preview-nav-bg);color:var(--preview-nav-icon)">‹</button>
        {#each slides as _,i}
          <div onclick={()=>slide=i} class="rounded-full cursor-pointer"
            style="width:{slide===i?20:8}px;height:8px;background:{slide===i?'var(--preview-dot-active)':'var(--preview-dot-inactive)'}"></div>
        {/each}
        <button onclick={()=>slide=Math.min(slides.length-1,slide+1)} disabled={slide===slides.length-1}
          class="w-7 h-7 rounded-md flex items-center justify-center font-bold text-sm"
          style="background:var(--preview-nav-bg);color:var(--preview-nav-icon)">›</button>
      </div>

    {:else}
      {#if message}
        <p class="font-[565] text-[15px] mb-6" style="color:var(--prompt-message)">{message}</p>
      {/if}

      {#if dual}
        <div class="flex flex-col gap-3 mb-6">
          <input bind:this={input1El} bind:value={value1} placeholder={placeholder1}
            class="w-full px-4 py-2 rounded-md text-[15px] font-[565] outline-none"
            style="color:var(--prompt-input-text);background:var(--prompt-input-bg)"
            onmouseenter={(e)=>e.currentTarget.style.background='var(--prompt-input-bg-hover)'}
            onmouseleave={(e)=>e.currentTarget.style.background='var(--prompt-input-bg)'}
            onkeydown={(e)=>e.key==='Enter'&&confirm(value1,value2)} />
          <input bind:value={value2} placeholder={placeholder2}
            class="w-full px-4 py-2 rounded-md text-[15px] font-[565] outline-none"
            style="color:var(--prompt-input-text);background:var(--prompt-input-bg)"
            onmouseenter={(e)=>e.currentTarget.style.background='var(--prompt-input-bg-hover)'}
            onmouseleave={(e)=>e.currentTarget.style.background='var(--prompt-input-bg)'}
            onkeydown={(e)=>e.key==='Enter'&&confirm(value1,value2)} />
        </div>
      {:else if input}
        {#if isFile}
          <div class="flex items-center gap-2 mb-6">
            <input bind:this={inputEl} bind:value {placeholder}
              class="flex-1 px-4 py-2 rounded-md text-[15px] font-[565] outline-none"
              style="color:var(--prompt-input-text);background:var(--prompt-input-bg)"
              onmouseenter={(e)=>e.currentTarget.style.background='var(--prompt-input-bg-hover)'}
              onmouseleave={(e)=>e.currentTarget.style.background='var(--prompt-input-bg)'}
              onkeydown={(e)=>{if(e.key==='.')e.preventDefault();if(e.key==='Enter')confirm(value,ext)}} />
            <div class="relative">
              <button onclick={(e)=>{e.stopPropagation();extOpen=!extOpen}}
                class="px-4 py-2 rounded-md font-[565]"
                style="background:var(--prompt-ext-btn-bg);color:var(--prompt-ext-btn-text)"
                onmouseenter={(e)=>e.currentTarget.style.background='var(--prompt-ext-btn-bg-hover)'}
                onmouseleave={(e)=>e.currentTarget.style.background='var(--prompt-ext-btn-bg)'}>
                {ext}
              </button>
              {#if extOpen}
                <div class="absolute top-full right-0 mt-1 rounded shadow-lg z-50 min-w-20 overflow-hidden" style="background:var(--menu-bg)" onclick={(e)=>e.stopPropagation()}>
                  {#each ['.lua','.txt'] as o}
                    <button onclick={()=>{ext=o;extOpen=false}} class="block w-full px-4 py-2 text-left font-[565]"
                      style="color:var(--menu-item-text)"
                      onmouseenter={(e)=>e.currentTarget.style.background='var(--menu-item-hover)'}
                      onmouseleave={(e)=>e.currentTarget.style.background=''}>{o}</button>
                  {/each}
                </div>
              {/if}
            </div>
          </div>
        {:else}
          <input bind:this={inputEl} bind:value {placeholder}
            class="w-full px-4 py-2 rounded-md text-[15px] font-[565] outline-none mb-6"
            style="color:var(--prompt-input-text);background:var(--prompt-input-bg)"
            onmouseenter={(e)=>e.currentTarget.style.background='var(--prompt-input-bg-hover)'}
            onmouseleave={(e)=>e.currentTarget.style.background='var(--prompt-input-bg)'}
            onkeydown={(e)=>e.key==='Enter'&&confirm(value)} />
        {/if}
      {/if}

      <div class="flex gap-3 justify-end">
        <button onclick={cancel} class="px-4 py-2 rounded-md font-[565]"
          style="background:var(--prompt-btn-cancel-bg);color:var(--prompt-btn-cancel-text)"
          onmouseenter={(e)=>e.currentTarget.style.background='var(--prompt-btn-cancel-bg-hover)'}
          onmouseleave={(e)=>e.currentTarget.style.background='var(--prompt-btn-cancel-bg)'}>Cancel</button>
        <button onclick={()=>dual?confirm(value1,value2):isFile?confirm(value,ext):confirm(input?value:undefined)}
          class="px-4 py-2 rounded-md font-[565]"
          style="background:var(--prompt-btn-confirm-bg);color:var(--prompt-btn-confirm-text)"
          onmouseenter={(e)=>e.currentTarget.style.background='var(--prompt-btn-confirm-bg-hover)'}
          onmouseleave={(e)=>e.currentTarget.style.background='var(--prompt-btn-confirm-bg)'}>Confirm</button>
      </div>
    {/if}

  </div>
</div>