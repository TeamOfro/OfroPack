import type { MetadataJson } from '$lib/types';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
  try {
    const response = await fetch('/metadata.json');
    if (!response.ok) {
      throw new Error(`Failed to fetch metadata.json: ${response.statusText}`);
    }
    const metadata = (await response.json()) as MetadataJson;
    return { metadata };
  }
  catch (e) {
    console.error('Failed to load metadata:', e);
    return { metadata: null, error: 'メタデータの読み込みに失敗しました。' };
  }
};
