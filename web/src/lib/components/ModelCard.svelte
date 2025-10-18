<script lang='ts'>
  import type { ModelData } from '$lib/types';
  import { assetUrl } from '$lib';

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
  class='model-card overflow-hidden rounded-lg border border-border bg-[#1f2328] transition-all hover:-translate-y-1 hover:border-primary hover:shadow-[0_10px_30px_rgba(0,0,0,0.4)]'
>
  <a href={`/model/${model.name}`} class='model-texture-link block'>
    <div
      class='model-texture relative flex h-72 w-full items-center justify-center overflow-hidden border-b border-b-border bg-[#1a1d21]'
    >
      <img
        src={assetUrl(model.texture_url)}
        alt={model.name}
        loading='lazy'
        decoding='async'
        class='h-4/5 w-4/5 object-contain [image-rendering:pixelated]'
      />
    </div>
  </a>
  <div class='model-info p-5'>
    <button
      class='model-name relative cursor-pointer select-none pb-2 pr-8 font-mono text-lg font-bold text-primary w-full text-left bg-transparent border-none transition-colors duration-300 hover:text-primary-hover after:content-["📋"] after:absolute after:right-0 after:opacity-0 after:transition-opacity after:duration-300 hover:after:opacity-100 [&.copied]:after:content-["✓"] [&.copied]:after:text-primary [&.copied]:after:opacity-100'
      onclick={e => copyToClipboard(model.name, e.currentTarget)}
      onkeydown={e => e.key === 'Enter' && copyToClipboard(model.name, e.currentTarget)}
      aria-label={`${model.name}をコピー`}
    >
      {model.name}
    </button>
    <div class='model-meta mb-4 text-sm text-muted'>
      📅 {addedDate}
      {#if model.author !== 'Unknown'}<br />👤 {model.author}{/if}
      {#if isAnimated}<br />🎬 {frameCount}フレーム{/if}
    </div>
    <div class='materials mt-2 flex flex-wrap gap-1'>
      {#each model.materials as m}
        <span class='bg-border py-1 px-2.5 rounded text-[0.85em] text-white font-mono'>{m}</span>
      {/each}
    </div>
  </div>
</div>
