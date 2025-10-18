<script lang='ts'>
  import type { ModelData } from '$lib/types';
  import { assetUrl } from '$lib/url';
  import ModelImg from './ModelImg.svelte';

  const { model }: { model: ModelData } = $props();

  const addedDate = new Date(model.added_date).toLocaleDateString('ja-JP', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });

  const isAnimated = !!model.animation;
  const frameCount = model.animation?.frame_count;

  function copyToClipboard(text: string, element: HTMLElement) {
    navigator.clipboard.writeText(text).then(() => {
      element.classList.add('copied');
      setTimeout(() => {
        element.classList.remove('copied');
      }, 1500);
    });
  }
</script>

<div
  class='overflow-hidden rounded-lg border border-border bg-[#1f2328] transition-all hover:-translate-y-1 hover:border-primary hover:shadow-[0_10px_30px_rgba(0,0,0,0.4)]'
>
  <a href={assetUrl(`/model/${model.name}`)} class='block'>
    <div
      class='relative flex h-60 min-[1400px]:h-72 w-full items-center justify-center overflow-hidden border-b border-b-border bg-[#1a1d21]'
    >
      <ModelImg {model} />
    </div>
  </a>
  <div class='p-5'>
    <button
      class='relative cursor-pointer select-none pb-2 pr-8 font-mono text-lg font-bold text-primary w-full text-left bg-transparent border-none transition-colors duration-300 hover:text-primary-hover after:content-["📋"] after:absolute after:right-0 after:opacity-0 after:transition-opacity after:duration-300 hover:after:opacity-100 [&.copied]:after:content-["✓"] [&.copied]:after:text-primary [&.copied]:after:opacity-100'
      onclick={e => copyToClipboard(model.name, e.currentTarget)}
      onkeydown={e => e.key === 'Enter' && copyToClipboard(model.name, e.currentTarget)}
      aria-label={`${model.name}をコピー`}
    >
      {model.name}
    </button>
    <div class='mb-4 text-sm text-muted'>
      📅 {addedDate}
      {#if model.author !== 'Unknown'}<br />👤 {model.author}{/if}
      {#if isAnimated}<br />🎬 {frameCount}フレーム{/if}
    </div>
    <div class='mt-2 flex flex-wrap gap-1'>
      {#each model.materials as m}
        <span class='bg-border py-1 px-2.5 rounded text-[0.85em] text-white font-mono'>{m}</span>
      {/each}
    </div>
  </div>
</div>
