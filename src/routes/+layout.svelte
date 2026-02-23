<script lang="ts">
  let { children } = $props();
  import "../app.css";
  import { onMount } from "svelte";
</script>

{@render children()}

