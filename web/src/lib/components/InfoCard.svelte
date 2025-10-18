<script lang='ts'>
  import type { Snippet } from 'svelte';

  const { title, children }: { title: string; children: Snippet } = $props();
</script>

<div class='bg-[#373c47] p-4 rounded-lg'>
  <h3 class='mt-0 mb-2 text-sm text-muted uppercase'>{title}</h3>
  <p class='m-0 text-base font-bold break-words'>
    {@render children()}
  </p>
</div>
