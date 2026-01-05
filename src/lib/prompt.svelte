<script>
  let { title = "", message = "", input = false, placeholder = "", defaultValue = "", onconfirm = () => {}, oncancel = () => {} } = $props();
  let value = $state(defaultValue);
  let inputEl;

  $effect(() => {
    if (input) {
      inputEl?.focus();
      inputEl?.select();
    }
  });
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onclick={oncancel}>
  <div class="bg-[#111111] rounded-lg p-6 min-w-96 max-w-md" onclick={(e) => e.stopPropagation()}>
    <h2 class="text-lg font-semibold text-white/50 mb-2">{title}</h2>
    {#if message}
      <p class="font-bold text-[15px] text-white/25 mb-6">{message}</p>
    {/if}
    
    {#if input}
      <input
        bind:this={inputEl}
        bind:value
        {placeholder}
        class="w-full px-4 py-2 rounded-md text-[15px] font-[565] text-white/50 bg-[#242424] outline-none mb-6"
        onkeydown={(e) => e.key === 'Enter' && onconfirm(value)}
      />
    {/if}
    
    <div class="flex gap-3 justify-end">
      <button
        onclick={oncancel}
        class="px-4 py-2 rounded-md bg-[#242424] hover:bg-[#323232] text-[#FFFFFF]/50 items-center font-semibold"
      >
        Cancel
      </button>
      <button
        onclick={() => input ? onconfirm(value) : onconfirm()}
        class="px-4 py-2 rounded-md bg-[#323232] text-[#FFFFFF]/50 items-center font-semibold"
      >
        Confirm
      </button>
    </div>
  </div>
</div>