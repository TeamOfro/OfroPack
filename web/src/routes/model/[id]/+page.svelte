<script lang="ts">
	import { error } from '@sveltejs/kit';
	import type { PageData } from './$types';

	let { data }: { data: PageData } = $props();

	const { model, error: loadError } = data;

	if (loadError) {
		error(data.status || 500, loadError);
	}

	if (!model) {
		error(404, 'モデルが見つかりません');
	}

	const addedDate = new Date(model.added_date).toLocaleDateString('ja-JP', {
		year: 'numeric',
		month: 'short',
		day: 'numeric'
	});

	const isAnimated = !!model.animation;
	let imgStyle = $state('');
	if (isAnimated && model.animation) {
		const fps = 20 / model.animation.frametime;
		const duration = model.animation.frame_count / fps;
		imgStyle = `animation: sprite-anim ${duration}s steps(${model.animation.frame_count}) infinite; aspect-ratio: 1 / ${model.animation.frame_count};`;
	}

	function copyToClipboard(text: string, button: HTMLButtonElement) {
		navigator.clipboard.writeText(text).then(() => {
			const originalText = button.textContent;
			button.textContent = '✓ コピー完了';
			button.style.background = '#3a8f3e';
			setTimeout(() => {
				button.textContent = originalText;
				button.style.background = '';
			}, 2000);
		});
	}
</script>

<svelte:head>
	<title>{model.name} - OfroPack</title>
</svelte:head>

<main class="container mx-auto max-w-4xl rounded-lg bg-[--card-bg-color] p-6 shadow-[0_8px_24px_rgba(0,0,0,0.4)]">
	<header class="header mb-8 flex items-center justify-between border-b border-b-[--border-color] pb-6">
		<div class="header-title">
			<h1 class="font-mono text-3xl font-bold text-[--primary-color]">{model.name}</h1>
		</div>
		<nav class="nav" aria-label="ページナビゲーション">
			<a href="/gallery" class="nav-link">← ギャラリーに戻る</a>
		</nav>
	</header>

	<div class="model-details grid grid-cols-1 gap-8 md:grid-cols-2">
		<div
			class="model-image-container flex items-center justify-center overflow-hidden rounded-lg border border-[--border-color] bg-[#1a1d21] p-5"
		>
			<img
				src={`/${model.texture_url}`}
				alt={`${model.name}のテクスチャ`}
				class="w-full object-contain [image-rendering:pixelated] {isAnimated ? 'animated' : ''}"
				style={imgStyle}
			/>
		</div>
		<div class="model-info">
			<div class="model-meta">
				<h2 class="mb-5 text-2xl text-[--muted-text-color]">詳細情報</h2>
				<p class="mb-2"><strong>📅 追加日:</strong> {addedDate}</p>
				<p class="mb-2">
					<strong>👤 作者:</strong>
					<a href={`/gallery?author=${encodeURIComponent(model.author)}`} class="text-[--primary-color] no-underline hover:underline">
						{model.author}
					</a>
				</p>
				<p class="mb-2">
					<strong>🆔 ID:</strong>
					<code class="rounded bg-[--bg-color] p-1 font-mono">{model.name}</code>
				</p>
				{#if isAnimated && model.animation}
					<p class="mb-2">
						<strong>🎬 アニメーション:</strong>
						{model.animation.frame_count}フレーム (frametime: {model.animation.frametime})
					</p>
				{/if}
				<p class="mb-2"><strong>📦 対応マテリアル:</strong></p>
				<div class="materials mt-2 flex flex-wrap gap-1">
					{#each model.materials as m}
						<span class="material-tag">{m}</span>
					{/each}
				</div>
			</div>
			<div class="give-command mt-5 rounded-lg border border-[--border-color] bg-[#1a1d21] p-4">
				<h3 class="mb-3 text-[--muted-text-color]">/give コマンド</h3>
				{#each model.materials as material}
					{@const command = `/give @s ${material}[custom_model_data: {strings: [${model.name}]}]`}
					<div class="copyable-field mb-2 flex items-center justify-between rounded bg-[--bg-color] p-2 last:mb-0">
						<code class="overflow-hidden text-ellipsis whitespace-nowrap font-mono text-sm"
							>/give @s {material}[...]</code
						>
						<button class="copy-btn" onclick={(e) => copyToClipboard(command, e.currentTarget)}>コピー</button>
					</div>
				{/each}
			</div>
		</div>
	</div>
</main>

<style>
	.nav-link {
		color: var(--primary-color);
		text-decoration: none;
		padding: 10px 20px;
		border: 1px solid var(--primary-color);
		border-radius: var(--border-radius);
		transition: background-color 0.3s, color 0.3s;
	}
	.nav-link:hover {
		background: var(--primary-color);
		color: white;
	}
	.animated {
		object-fit: cover;
		object-position: 0 0;
		width: 100%;
		height: auto;
	}
	@keyframes sprite-anim {
		from {
			object-position: 0 0;
		}
		to {
			object-position: 0 100%;
		}
	}
	.material-tag {
		background: var(--border-color);
		padding: 4px 10px;
		border-radius: 4px;
		font-size: 0.85em;
		color: var(--text-color);
		font-family: var(--font-family-mono);
	}
	.copy-btn {
		background: var(--primary-color);
		color: white;
		border: none;
		padding: 7px 10px;
		border-radius: 5px;
		cursor: pointer;
		white-space: nowrap;
		transition: background-color 0.3s;
	}
	.copy-btn:hover {
		background: var(--primary-hover-color);
	}
</style>
