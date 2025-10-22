<script lang='ts'>
  import type { Snippet } from 'svelte';

  const { title, children }: { title: string; children: Snippet } = $props();
</script>

<div class='bg-[#373c47] p-5 rounded-lg transition-all duration-300 hover:shadow-lg hover:bg-[#3f4551] border border-transparent hover:border-primary/30'>
  <h3 class='mt-0 mb-3 text-xs text-muted uppercase tracking-wider font-semibold'>{title}</h3>
  <div class='text-base font-bold break-words'>
    {@render children()}
  </div>
</div>
