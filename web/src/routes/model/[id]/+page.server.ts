import type { ModelData, ModelsJson } from '$lib/types';
import type { EntryGenerator, PageServerLoad } from './$types';
import { readFile } from 'node:fs/promises';

export const load: PageServerLoad = async ({ params }) => {
  try {
    const data = JSON.parse(await readFile('static/models.json', 'utf-8')) as ModelsJson;
    const model = data.models.find((m: ModelData) => m.name === params.id);

    if (!model) {
      return { status: 404, error: 'モデルが見つかりません' };
    }

    return { model };
  }
  catch (e) {
    console.error('Failed to load model:', e);
    return { status: 500, error: 'モデルデータの読み込みに失敗しました。' };
  }
};

export const entries: EntryGenerator = async () => {
  try {
    // During build, read from static directory
    const fs = await import('node:fs/promises');
    const path = await import('node:path');
    const process = await import('node:process');
    const filePath = path.resolve(process.cwd(), 'static', 'models.json');
    const content = await fs.readFile(filePath, 'utf-8');
    const data = JSON.parse(content) as ModelsJson;
    return data.models.map((model: ModelData) => ({ id: model.name }));
  }
  catch (e) {
    console.error('Failed to generate entries:', e);
    return [];
  }
};

export const prerender = true;
