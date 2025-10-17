import fs from 'node:fs/promises';
import path from 'node:path';
import type { PageServerLoad } from './$types';
import type { MetadataJson } from '$lib/types';

export const load: PageServerLoad = async () => {
	try {
		// Resolve from project root, not web directory
		const filePath = path.resolve(process.cwd(), '../metadata.json');
		const content = await fs.readFile(filePath, 'utf-8');
		const metadata = JSON.parse(content) as MetadataJson;
		return { metadata };
	} catch (e) {
		console.error('Failed to load metadata:', e);
		return { metadata: null, error: 'メタデータの読み込みに失敗しました。' };
	}
};
