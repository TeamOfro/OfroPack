import type { ModelsJson } from '$lib/types';
import type { PageServerLoad } from './$types';
import { readFile } from 'node:fs/promises';

export const load: PageServerLoad = async () => {
  try {
    const data = JSON.parse(await readFile('static/models.json', 'utf-8')) as ModelsJson;
    return { models: data.models };
  }
  catch (e) {
    console.error('Failed to load models:', e);
    return { models: [], error: 'モデルデータの読み込みに失敗しました。' };
  }
};
