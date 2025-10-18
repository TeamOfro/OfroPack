<script lang='ts'>
  import type { PageData } from './$types';
  import { assetUrl } from '$lib/url';
  import { error } from '@sveltejs/kit';

  const { data }: { data: PageData } = $props();

  const { model, error: loadError } = data;

  if (loadError) {
    error(data.status || 500, loadError);
  }

  if (!model) {
    error(404, 'モデルが見つかりません');
  }

  const addedDate = new Date(model.added_date).toLocaleDateString('ja-JP', {
    year: 'numeric',
    month: 'short',
    day: 'numeric',
  });

  const isAnimated = !!model.animation;
  let imgStyle = $state('');
  if (isAnimated && model.animation) {
    const fps = 20 / model.animation.frametime;
    const duration = model.animation.frame_count / fps;
    imgStyle = `animation: sprite-anim ${duration}s steps(${model.animation.frame_count}) infinite; aspect-ratio: 1 / ${model.animation.frame_count};`;
  }

  function copyToClipboard(text: string, button: HTMLButtonElement) {
    navigator.clipboard.writeText(text).then(() => {
      const originalText = button.textContent;
      button.textContent = '✓ コピー完了';
      button.style.background = '#3a8f3e';
      setTimeout(() => {
        button.textContent = originalText;
        button.style.background = '';
      }, 2000);
    });
  }
</script>

<svelte:head>
  <title>{model.name} - OfroPack</title>
</svelte:head>

<main class='container mx-auto max-w-4xl rounded-lg bg-card-bg p-6 shadow-[0_8px_24px_rgba(0,0,0,0.4)]'>
  <header class='header mb-8 flex items-center justify-between border-b border-b-border pb-6'>
    <div class='header-title'>
      <h1 class='font-mono text-3xl font-bold text-primary'>{model.name}</h1>
    </div>
    <nav class='nav' aria-label='ページナビゲーション'>
      <a href={assetUrl('/gallery')} class='text-primary no-underline py-2.5 px-5 border border-primary rounded-lg transition-all duration-300 hover:bg-primary hover:text-white'>← ギャラリーに戻る</a>
    </nav>
  </header>

  <div class='model-details grid grid-cols-1 gap-8 md:grid-cols-2'>
    <div
      class='model-image-container flex items-center justify-center overflow-hidden rounded-lg border border-border bg-[#1a1d21] p-5'
    >
      <img
        src={assetUrl(model.texture_url)}
        alt={`${model.name}のテクスチャ`}
        class="w-full object-contain [image-rendering:pixelated] {isAnimated ? 'animated' : ''}"
        style={imgStyle}
      />
    </div>
    <div class='model-info'>
      <div class='model-meta'>
        <h2 class='mb-5 text-2xl text-muted'>詳細情報</h2>
        <p class='mb-2'><strong>📅 追加日:</strong> {addedDate}</p>
        <p class='mb-2'>
          <strong>👤 作者:</strong>
          <a href={assetUrl(`/gallery?author=${encodeURIComponent(model.author)}`)} class='text-primary no-underline hover:underline'>
            {model.author}
          </a>
        </p>
        <p class='mb-2'>
          <strong>🆔 ID:</strong>
          <code class='rounded bg-background p-1 font-mono'>{model.name}</code>
        </p>
        {#if isAnimated && model.animation}
          <p class='mb-2'>
            <strong>🎬 アニメーション:</strong>
            {model.animation.frame_count}フレーム (frametime: {model.animation.frametime})
          </p>
        {/if}
        <p class='mb-2'><strong>📦 対応マテリアル:</strong></p>
        <div class='materials mt-2 flex flex-wrap gap-1'>
          {#each model.materials as m}
            <span class='bg-border py-1 px-2.5 rounded text-[0.85em] text-white font-mono'>{m}</span>
          {/each}
        </div>
      </div>
      <div class='give-command mt-5 rounded-lg border border-border bg-[#1a1d21] p-4'>
        <h3 class='mb-3 text-muted'>/give コマンド</h3>
        {#each model.materials as material}
          {@const command = `/give @s ${material}[custom_model_data:{strings:['${model.name}']}]`}
          <div class='flex items-center justify-between rounded bg-background p-2 mb-2 last:mb-0'>
            <code class='overflow-hidden text-ellipsis whitespace-nowrap font-mono text-sm'
            >/give @s {material}[...]</code
            >
            <button class='bg-primary text-white border-none py-1.5 px-2.5 rounded-md cursor-pointer whitespace-nowrap transition-colors duration-300 hover:bg-primary-hover' onclick={e => copyToClipboard(command, e.currentTarget)}>コピー</button>
          </div>
        {/each}
      </div>
    </div>
  </div>
</main>

<style>
  .animated {
    object-fit: cover;
    object-position: 0 0;
    width: 100%;
    height: auto;
  }
  @keyframes sprite-anim {
    from {
      object-position: 0 0;
    }
    to {
      object-position: 0 100%;
    }
  }
</style>
