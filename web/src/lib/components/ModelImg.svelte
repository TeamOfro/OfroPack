<script lang='ts'>
  import type { ModelData } from '$lib/types';
  import { assetUrl } from '$lib/url';

  const { model }: { model: ModelData } = $props();

  const isAnimated = !!model.animation;
  const frameCount = model.animation?.frame_count || 0;
  const frametime = model.animation?.frametime || 0;

  const style = isAnimated
    ? `
      animation: sprite-anim ${frametime / (20 / frameCount)}s steps(${frameCount}) infinite;
    `
    : `
    `;
</script>

<div class='mx-auto aspect-square relative overflow-y-hidden h-full'>
  {#if model.texture_path}
    <img
      src={assetUrl(model.texture_path)}
      alt={model.name}
      loading='lazy'
      decoding='async'
      class='absolute w-full object-contain [image-rendering:pixelated] transition-none'
      style={style}
    />
  {:else}
    <div class='absolute w-full h-full flex items-center justify-center bg-gray-200 dark:bg-gray-700 text-gray-500'>
      3D Model
    </div>
  {/if}
</div>
