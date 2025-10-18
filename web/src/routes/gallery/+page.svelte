<script lang='ts'>
  import type { PageData } from './$types';
  import { assetUrl } from '$lib';
  import { onMount } from 'svelte';

  const { data }: { data: PageData } = $props();

  const { models, error } = data;

  // Filter and sort states
  let materialFilter = $state('');
  let authorFilter = $state('');
  let idFilter = $state('');
  let sortOrder = $state('date_desc');

  // Derived states for select options
  const allMaterials = [
    ...new Set(models.flatMap(model => model.materials)),
  ].sort();
  const allAuthors = [
    ...new Set(
      models
        .map(model => model.author)
        .filter(author => author && author !== 'Unknown'),
    ),
  ].sort();

  // Reactive filtered and sorted models
  const filteredModels = $derived.by(() => {
    let filtered = models;

    if (materialFilter) {
      filtered = filtered.filter(model =>
        model.materials.some(m => m.toLowerCase() === materialFilter.toLowerCase()),
      );
    }

    if (authorFilter) {
      filtered = filtered.filter(model => model.author.toLowerCase() === authorFilter.toLowerCase());
    }

    if (idFilter) {
      filtered = filtered.filter(model =>
        model.name.toLowerCase().includes(idFilter.toLowerCase().trim()),
      );
    }

    return filtered.sort((a, b) => {
      switch (sortOrder) {
        case 'date_asc':
          return new Date(a.added_date).getTime() - new Date(b.added_date).getTime();
        case 'name_asc':
          return a.name.localeCompare(b.name);
        case 'name_desc':
          return b.name.localeCompare(a.name);
        case 'date_desc':
        default:
          return new Date(b.added_date).getTime() - new Date(a.added_date).getTime();
      }
    });
  });

  onMount(() => {
    const urlParams = new URLSearchParams(window.location.search);
    materialFilter = urlParams.get('material') || '';
    authorFilter = urlParams.get('author') || '';
    idFilter = urlParams.get('id') || '';
    sortOrder = urlParams.get('sort') || 'date_desc';
  });

  function copyToClipboard(text: string, element: HTMLElement) {
    navigator.clipboard.writeText(text).then(() => {
      element.classList.add('copied');
      setTimeout(() => {
        element.classList.remove('copied');
      }, 1500);
    });
  }
</script>

<svelte:head>
  <title>カスタムモデルギャラリー - OfroPack</title>
</svelte:head>

<main
  class='container mx-auto max-w-7xl rounded-lg bg-[--card-bg-color] p-6 shadow-[0_8px_24px_rgba(0,0,0,0.4)]'
>
  <header
    class='header mb-8 flex flex-wrap items-center justify-between gap-5 border-b border-b-[--border-color] pb-6'
  >
    <div class='header-title'>
      <h1 class='mb-1 text-3xl font-bold text-[--primary-color]'>🎨 カスタムモデルギャラリー</h1>
      <p class='text-base text-[--muted-text-color]'>OfroPack に含まれる全てのカスタムモデル</p>
    </div>
    <nav class='nav' aria-label='ページナビゲーション'>
      <a href='/' class='nav-link'>← ホームに戻る</a>
    </nav>
  </header>

  <section
    class='filter-panel mb-5 flex flex-wrap items-center justify-between gap-4 py-4'
    aria-label='フィルタと並び替え'
  >
    <div class='filter-group flex flex-wrap items-center gap-3'>
      <label for='filter-material' class='text-sm text-[--muted-text-color]'>マテリアル:</label>
      <select id='filter-material' bind:value={materialFilter}>
        <option value="">すべてのマテリアル</option>
        {#each allMaterials as material}
          <option value={material}>{material}</option>
        {/each}
      </select>
      <label for='filter-author' class='text-sm text-[--muted-text-color]'>作者:</label>
      <select id='filter-author' bind:value={authorFilter}>
        <option value="">すべての作者</option>
        {#each allAuthors as author}
          <option value={author}>{author}</option>
        {/each}
      </select>
      <label for='filter-id' class='text-sm text-[--muted-text-color]'>ID:</label>
      <input type='text' id='filter-id' placeholder='例: sennsyuuraku' bind:value={idFilter} />
    </div>
    <div class='filter-group flex flex-wrap items-center gap-3'>
      <label for='sort-order' class='text-sm text-[--muted-text-color]'>並び順:</label>
      <select id='sort-order' bind:value={sortOrder}>
        <option value='date_desc'>追加日が新しい順</option>
        <option value='date_asc'>追加日が古い順</option>
        <option value='name_asc'>名前 (A-Z)</option>
        <option value='name_desc'>名前 (Z-A)</option>
      </select>
    </div>
    <div class='stats w-full text-right text-sm text-[--muted-text-color]' aria-live='polite'>
      {#if filteredModels.length === models.length}
        全 {models.length} 個のカスタムモデル
      {:else}
        {filteredModels.length} / {models.length} 個のカスタムモデルを表示中
      {/if}
    </div>
  </section>

  <section class='gallery grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-5'>
    {#if error}
      <div class='error col-span-full' role='alert'>
        <strong>{error}</strong>
      </div>
    {:else if filteredModels.length === 0}
      <div class='error col-span-full' role='status'>
        条件に一致するカスタムモデルが見つかりませんでした。<br />フィルタ条件を変更してみてください。
      </div>
    {:else}
      {#each filteredModels as model (model.name)}
        {@const addedDate = new Date(model.added_date).toLocaleDateString('ja-JP', { year: 'numeric', month: 'short', day: 'numeric' })}
        {@const isAnimated = !!model.animation}
        {@const frameCount = model.animation?.frame_count}

        <div
          class='model-card overflow-hidden rounded-lg border border-[--border-color] bg-[#1f2328] transition-all hover:-translate-y-1 hover:border-[--primary-color] hover:shadow-[0_10px_30px_rgba(0,0,0,0.4)]'
        >
          <a href={`/model/${model.name}`} class='model-texture-link block'>
            <div
              class='model-texture relative flex h-72 w-full items-center justify-center overflow-hidden border-b border-b-[--border-color] bg-[#1a1d21]'
            >
              <img
                src={assetUrl(model.texture_url)}
                alt={model.name}
                loading='lazy'
                decoding='async'
                class='h-4/5 w-4/5 object-contain [image-rendering:pixelated]'
              />
            </div>
          </a>
          <div class='model-info p-5'>
            <button
              class='model-name relative cursor-pointer select-none pb-2 pr-8 font-mono text-lg font-bold text-[--primary-color] hover:text-[--primary-hover-color] w-full text-left bg-transparent border-none'
              onclick={e => copyToClipboard(model.name, e.currentTarget)}
              onkeydown={e => e.key === 'Enter' && copyToClipboard(model.name, e.currentTarget)}
              aria-label={`${model.name}をコピー`}
            >
              {model.name}
            </button>
            <div class='model-meta mb-4 text-sm text-[--muted-text-color]'>
              📅 {addedDate}
              {#if model.author !== 'Unknown'}<br />👤 {model.author}{/if}
              {#if isAnimated}<br />🎬 {frameCount}フレーム{/if}
            </div>
            <div class='materials mt-2 flex flex-wrap gap-1'>
              {#each model.materials as m}
                <span class='material-tag'>{m}</span>
              {/each}
            </div>
          </div>
        </div>
      {/each}
    {/if}
  </section>
</main>

<style>
	.nav-link {
		color: var(--primary-color);
		text-decoration: none;
		padding: 10px 20px;
		border: 1px solid var(--primary-color);
		border-radius: var(--border-radius);
		transition: background-color 0.3s, color 0.3s;
		white-space: nowrap;
	}
	.nav-link:hover {
		background: var(--primary-color);
		color: white;
	}
	.filter-group input[type='text'],
	.filter-group select {
		background: #1a1d21;
		border: 1px solid var(--border-color);
		border-radius: 4px;
		padding: 8px 12px;
		color: var(--text-color);
		font-family: var(--font-family-mono);
		min-width: 180px;
		transition: border-color 0.3s;
	}
	.filter-group select {
		font-family: var(--font-family-sans);
		-webkit-appearance: none;
		-moz-appearance: none;
		appearance: none;
		background-image: url('data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%23e0e0e0%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E');
		background-repeat: no-repeat;
		background-position: right 12px center;
		background-size: 10px;
		padding-right: 30px;
	}
	.model-name::after {
		content: '📋';
		position: absolute;
		right: 0;
		opacity: 0;
		transition: opacity 0.3s;
	}
	.model-name:hover::after {
		opacity: 1;
	}
	.model-name:global(.copied)::after {
		content: '✓';
		color: var(--primary-color);
		opacity: 1;
	}
	.material-tag {
		background: var(--border-color);
		padding: 4px 10px;
		border-radius: 4px;
		font-size: 0.85em;
		color: var(--text-color);
		font-family: var(--font-family-mono);
	}
</style>
