<script lang='ts'>
  import type { ModelData } from '$lib/types';
  import { assetUrl } from '$lib/url';

  const { model }: { model: ModelData } = $props();

  const isAnimated = !!model.animation;
  const frameCount = model.animation?.frame_count || 0;
</script>

<img
  src={assetUrl(model.texture_url)}
  alt={model.name}
  loading='lazy'
  decoding='async'
  class='h-4/5 w-4/5 object-contain [image-rendering:pixelated]'
/>
