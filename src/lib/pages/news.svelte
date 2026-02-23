<script>
  let { onclose = () => {} } = $props();
</script>

<div class="fixed inset-0 backdrop-blur-sm z-50 flex items-center justify-center" style="background: var(--news-overlay);" onclick={onclose}>
  <div class="rounded-md w-[500px] max-h-[600px] h-[450px] p-5" style="background: var(--news-bg);" onclick={(e) => e.stopPropagation()}>
  </div>
</div>