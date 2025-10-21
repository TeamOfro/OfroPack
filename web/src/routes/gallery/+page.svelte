<script lang='ts'>
  import type { PageData } from './$types';
  import { replaceState } from '$app/navigation';
  import ModelCard from '$lib/components/ModelCard.svelte';
  import { assetUrl } from '$lib/url';
  import { onMount } from 'svelte';

  const { data }: { data: PageData } = $props();

  const { models, error } = data;

  // Filter and sort states
  let materialFilter = $state('');
  let idFilter = $state('');
  let sortOrder = $state('date_desc');

  // Derived states for select options
  const allMaterials = [
    ...new Set(models.flatMap(model => model.materials)),
  ].sort();

  // Reactive filtered and sorted models
  const filteredModels = $derived.by(() => {
    let filtered = models;

    if (materialFilter) {
      filtered = filtered.filter(model =>
        model.materials.some(m => m.toLowerCase() === materialFilter.toLowerCase()),
      );
    }

    if (idFilter) {
      filtered = filtered.filter(model =>
        model.name.toLowerCase().includes(idFilter.toLowerCase().trim()),
      );
    }

    const sorted = filtered.sort((a, b) => {
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
    return {
      get length() {
        return sorted.length;
      },
      get models() {
        return sorted;
      },
    };
  });

  onMount(() => {
    const urlParams = new URLSearchParams(window.location.search);
    materialFilter = urlParams.get('material') || '';
    idFilter = urlParams.get('id') || '';
    sortOrder = urlParams.get('sort') || 'date_desc';
  });

  $effect(() => {
    const urlParams = new URLSearchParams();
    if (materialFilter)
      urlParams.set('material', materialFilter);
    if (idFilter)
      urlParams.set('id', idFilter);
    if (sortOrder !== 'date_desc')
      urlParams.set('sort', sortOrder);

    let newUrl = window.location.pathname;

    if (urlParams.size !== 0) {
      newUrl += `?${urlParams.toString()}`;
    }

    try {
      replaceState(newUrl, {});
    }
    catch {
    }
  });

</script>

<svelte:head>
  <title>カスタムモデルギャラリー - OfroPack</title>
</svelte:head>

<main
  class='container mx-auto max-w-7xl rounded-lg bg-card-bg p-6 shadow-[0_8px_24px_rgba(0,0,0,0.4)]'
>
  <header
    class='header mb-8 flex flex-wrap items-center justify-between gap-5 border-b border-b-border pb-6'
  >
    <div class='header-title'>
      <h1 class='mb-1 text-3xl font-bold text-primary'>🎨 カスタムモデルギャラリー</h1>
      <p class='text-base text-muted'>OfroPack に含まれる全てのカスタムモデル</p>
    </div>
    <nav class='nav' aria-label='ページナビゲーション'>
      <a href={assetUrl('/')} class='text-primary no-underline py-2.5 px-5 border border-primary rounded-lg transition-all duration-300 whitespace-nowrap hover:bg-primary hover:text-white'>← ホームに戻る</a>
    </nav>
  </header>

  <section
    class='filter-panel mb-5 flex flex-wrap items-center justify-between gap-4 py-4'
    aria-label='フィルタと並び替え'
  >
    <div class='filter-group flex flex-wrap items-center gap-3'>
      <label for='filter-material' class='text-sm text-muted'>マテリアル:</label>
      <select id='filter-material' bind:value={materialFilter} class='bg-[#1a1d21] border border-border rounded px-3 py-2 text-white font-sans min-w-[180px] transition-colors duration-300 appearance-none [background-image:url("data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%23e0e0e0%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E")] [background-repeat:no-repeat] [background-position:right_12px_center] [background-size:10px] pr-[30px]'>
        <option value="">すべてのマテリアル</option>
        {#each allMaterials as material}
          <option value={material}>{material}</option>
        {/each}
      </select>
      <label for='filter-id' class='text-sm text-muted'>ID:</label>
      <input type='text' id='filter-id' placeholder='例: sennsyuuraku' bind:value={idFilter} class='bg-[#1a1d21] border border-border rounded px-3 py-2 text-white font-mono min-w-[180px] transition-colors duration-300' />
    </div>
    <div class='filter-group flex flex-wrap items-center gap-3'>
      <label for='sort-order' class='text-sm text-muted'>並び順:</label>
      <select id='sort-order' bind:value={sortOrder} class='bg-[#1a1d21] border border-border rounded px-3 py-2 text-white font-sans min-w-[180px] transition-colors duration-300 appearance-none [background-image:url("data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%23e0e0e0%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E")] [background-repeat:no-repeat] [background-position:right_12px_center] [background-size:10px] pr-[30px]'>
        <option value='date_desc'>追加日が新しい順</option>
        <option value='date_asc'>追加日が古い順</option>
        <option value='name_asc'>名前 (A-Z)</option>
        <option value='name_desc'>名前 (Z-A)</option>
      </select>
    </div>
    <div class='stats w-full text-right text-sm text-muted' aria-live='polite'>
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
    {:else}
      {#each filteredModels.models as model (model.name)}
        <ModelCard {model} />
      {:else}
        <div class='error col-span-full' role='status'>
          条件に一致するカスタムモデルが見つかりませんでした。<br />フィルタ条件を変更してみてください。
        </div>
      {/each}
    {/if}
  </section>
</main>
