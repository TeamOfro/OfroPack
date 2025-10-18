import type { ModelsJson } from '$lib/types';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
  try {
    // Fetch from static files (will be available at build time)
    const response = await fetch('/models.json');
    if (!response.ok) {
      throw new Error(`Failed to fetch models.json: ${response.statusText}`);
    }
    const data = (await response.json()) as ModelsJson;
    return { models: data.models };
  }
  catch (e) {
    console.error('Failed to load models:', e);
    return { models: [], error: 'モデルデータの読み込みに失敗しました。' };
  }
};
