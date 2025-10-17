import fs from 'node:fs/promises';
import path from 'node:path';
import type { PageServerLoad } from './$types';
import type { ModelsJson } from '$lib/types';

export const load: PageServerLoad = async () => {
	try {
		const filePath = path.resolve(process.cwd(), '../models.json');
		const content = await fs.readFile(filePath, 'utf-8');
		const data = JSON.parse(content) as ModelsJson;
		return { models: data.models };
	} catch (e) {
		console.error('Failed to load models:', e);
		return { models: [], error: 'モデルデータの読み込みに失敗しました。' };
	}
};
