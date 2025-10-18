import type { MetadataJson } from '$lib/types';
import type { PageServerLoad } from './$types';
import { readFile } from 'node:fs/promises';

export const load: PageServerLoad = async () => {
  try {
    const metadata = JSON.parse(await readFile('static/metadata.json', 'utf-8')) as MetadataJson;
    return { metadata };
  }
  catch (e) {
    console.error('Failed to load metadata:', e);
    return { metadata: null, error: 'メタデータの読み込みに失敗しました。' };
  }
};
