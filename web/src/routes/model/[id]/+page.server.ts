import type { ModelData, ModelsJson } from '$lib/types';
import type { EntryGenerator, PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ params, fetch }) => {
  try {
    const response = await fetch('/models.json');
    if (!response.ok) {
      throw new Error(`Failed to fetch models.json: ${response.statusText}`);
    }
    const data = (await response.json()) as ModelsJson;
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
