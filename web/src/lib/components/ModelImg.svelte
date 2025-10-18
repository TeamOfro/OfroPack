<script lang='ts'>
  import type { ModelData } from '$lib/types';
  import { assetUrl } from '$lib/url';

  const { model }: { model: ModelData } = $props();

  const isAnimated = !!model.animation;
  const frameCount = model.animation?.frame_count || 0;
  const frametime = model.animation?.frametime || 0;

  const style = isAnimated
    ? `
      animation: sprite-anim ${frametime / 20}s steps(${frameCount}) infinite;
    `
    : `
    `;
</script>

<div class='h-full mx-auto aspect-square relative overflow-y-hidden'>
  <img
    src={assetUrl(model.texture_url)}
    alt={model.name}
    loading='lazy'
    decoding='async'
    class='absolute w-full object-contain [image-rendering:pixelated] transition-none transform-none'
    style={style}
  />
</div>
