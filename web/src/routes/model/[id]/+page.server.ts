import fs from 'node:fs/promises';
import path from 'node:path';
import type { PageServerLoad, EntryGenerator } from './$types';
import type { ModelsJson, ModelData } from '$lib/types';

export const load: PageServerLoad = async ({ params }) => {
	try {
		const filePath = path.resolve(process.cwd(), '../models.json');
		const content = await fs.readFile(filePath, 'utf-8');
		const data = JSON.parse(content) as ModelsJson;
		const model = data.models.find((m: ModelData) => m.name === params.id);

		if (!model) {
			return { status: 404, error: 'モデルが見つかりません' };
		}

		return { model };
	} catch (e) {
		console.error('Failed to load model:', e);
		return { status: 500, error: 'モデルデータの読み込みに失敗しました。' };
	}
};

export const entries: EntryGenerator = async () => {
	try {
		const filePath = path.resolve(process.cwd(), '../models.json');
		const content = await fs.readFile(filePath, 'utf-8');
		const data = JSON.parse(content) as ModelsJson;
		return data.models.map((model: ModelData) => ({ id: model.name }));
	} catch (e) {
		console.error('Failed to generate entries:', e);
		return [];
	}
};

export const prerender = true;
