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
  const frametime = model.animation?.frametime;

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
  class='overflow-hidden rounded-lg border border-border bg-[#1f2328] transition-all duration-300 hover:-translate-y-1 hover:border-primary hover:shadow-[0_10px_30px_rgba(0,0,0,0.4)] group'
>
  <a href={assetUrl(`/model/${model.name}`)} class='block'>
    <div
      class='relative flex h-60 min-[1400px]:h-72 w-full items-center justify-center overflow-hidden border-b border-b-border bg-[#1a1d21] group-hover:bg-[#20252b] transition-colors'
    >
      <ModelImg {model} />
      {#if isAnimated}
        <div class='absolute top-2 right-2 bg-primary/90 text-white px-2 py-1 rounded text-xs font-bold flex items-center gap-1'>
          <svg class='w-3 h-3' fill='currentColor' viewBox='0 0 20 20'>
            <path d='M2 6a2 2 0 012-2h6a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V6zM14.553 7.106A1 1 0 0014 8v4a1 1 0 00.553.894l2 1A1 1 0 0018 13V7a1 1 0 00-1.447-.894l-2 1z'></path>
          </svg>
          <span>アニメーション</span>
        </div>
      {/if}
    </div>
  </a>
  <div class='p-5'>
    <button
      class='relative cursor-pointer select-none pb-2 pr-8 font-mono text-lg font-bold text-primary w-full text-left bg-transparent border-none transition-colors duration-300 hover:text-primary-hover after:content-["📋"] after:absolute after:right-0 after:opacity-0 after:transition-opacity after:duration-300 hover:after:opacity-100 [&.copied]:after:content-["✓"] [&.copied]:after:text-green-400 [&.copied]:after:opacity-100'
      onclick={e => copyToClipboard(model.name, e.currentTarget)}
      onkeydown={e => e.key === 'Enter' && copyToClipboard(model.name, e.currentTarget)}
      aria-label={`${model.name}をコピー`}
      title='クリックでコピー'
    >
      {model.name}
    </button>
    <div class='mb-4 text-sm text-muted space-y-1'>
      <div class='flex items-center gap-2'>
        <svg class='w-4 h-4' fill='currentColor' viewBox='0 0 20 20'>
          <path fill-rule='evenodd' d='M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z' clip-rule='evenodd'></path>
        </svg>
        <span>{addedDate}</span>
      </div>
      {#if isAnimated}
        <div class='flex items-center gap-2 text-primary'>
          <svg class='w-4 h-4' fill='currentColor' viewBox='0 0 20 20'>
            <path d='M2 6a2 2 0 012-2h6a2 2 0 012 2v8a2 2 0 01-2 2H4a2 2 0 01-2-2V6zM14.553 7.106A1 1 0 0014 8v4a1 1 0 00.553.894l2 1A1 1 0 0018 13V7a1 1 0 00-1.447-.894l-2 1z'></path>
          </svg>
          <span>{frameCount}フレーム ({frametime}tick)</span>
        </div>
      {/if}
    </div>
    <div class='mt-3'>
      <div class='text-xs text-muted uppercase tracking-wider mb-2 flex items-center gap-1'>
        <svg class='w-3 h-3' fill='currentColor' viewBox='0 0 20 20'>
          <path d='M7 3a1 1 0 000 2h6a1 1 0 100-2H7zM4 7a1 1 0 011-1h10a1 1 0 110 2H5a1 1 0 01-1-1zM2 11a2 2 0 012-2h12a2 2 0 012 2v4a2 2 0 01-2 2H4a2 2 0 01-2-2v-4z'></path>
        </svg>
        <span>マテリアル</span>
      </div>
      <div class='flex flex-wrap gap-1.5'>
        {#each model.materials as m}
          <span class='bg-border/60 hover:bg-border py-1.5 px-3 rounded text-[0.85em] text-white/90 font-mono transition-colors border border-transparent hover:border-primary/30'>{m}</span>
        {/each}
      </div>
    </div>
  </div>
</div>
