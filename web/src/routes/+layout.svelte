<script lang="ts">
	import '../app.css';
	let { children } = $props();
</script>

<svelte:head>
	<link rel="preconnect" href="https://fonts.googleapis.com" />
	<link rel="preconnect" href="https://fonts.gstatic.com" crossorigin="anonymous" />
	<link
		href="https://fonts.googleapis.com/css2?family=Noto+Sans+JP:wght@400;700&family=Roboto+Mono:wght@400;700&display=swap"
		rel="stylesheet"
	/>
</svelte:head>

<a
	href="https://github.com/TeamOfro/OfroPack"
	target="_blank"
	rel="noopener noreferrer"
	class="github-nav"
	aria-label="GitHubリポジトリを開く"></a>
<div class="github-icon" aria-hidden="true">
	<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg" class="fill-white w-full h-full">
		<path
			d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"
		></path>
	</svg>
</div>

<div class="p-2 sm:p-5">
	{@render children()}
</div>

<style>
	.github-nav {
		position: fixed;
		top: 0;
		right: 0;
		width: 0;
		height: 0;
		border-style: solid;
		border-width: 0 80px 80px 0;
		border-color: transparent var(--primary-color) transparent transparent;
		cursor: pointer;
		transition: border-color 0.3s;
		z-index: 1000;
	}

	.github-nav:hover {
		border-color: transparent var(--primary-hover-color) transparent transparent;
	}

	.github-icon {
		position: fixed;
		top: 8px;
		right: 8px;
		width: 32px;
		height: 32px;
		z-index: 1001;
		pointer-events: none;
	}
</style>
