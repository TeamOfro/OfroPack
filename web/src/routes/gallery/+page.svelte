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
  <meta name='description' content='OfroPack に含まれる全てのカスタムモデルをブラウズ' />
</svelte:head>

<main
  class='container mx-auto max-w-7xl rounded-lg bg-card-bg p-6 shadow-[0_8px_24px_rgba(0,0,0,0.4)]'
>
  <header
    class='header mb-8 flex flex-wrap items-center justify-between gap-5 border-b border-b-border pb-6'
  >
    <div class='header-title'>
      <h1 class='mb-2 text-4xl font-bold text-primary flex items-center gap-3'>
        <svg class='w-10 h-10' fill='currentColor' viewBox='0 0 20 20'>
          <path d='M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z'></path>
        </svg>
        <span>カスタムモデルギャラリー</span>
      </h1>
      <p class='text-base text-muted'>OfroPack に含まれる全てのカスタムモデル</p>
    </div>
    <nav class='nav' aria-label='ページナビゲーション'>
      <a
        href={assetUrl('/')}
        class='text-primary no-underline py-3 px-6 border-2 border-primary rounded-lg transition-all duration-300 whitespace-nowrap hover:bg-primary hover:text-white inline-flex items-center gap-2 font-semibold'
      >
        <svg class='w-4 h-4' fill='currentColor' viewBox='0 0 20 20'>
          <path d='M10.707 2.293a1 1 0 00-1.414 0l-7 7a1 1 0 001.414 1.414L4 10.414V17a1 1 0 001 1h2a1 1 0 001-1v-2a1 1 0 011-1h2a1 1 0 011 1v2a1 1 0 001 1h2a1 1 0 001-1v-6.586l.293.293a1 1 0 001.414-1.414l-7-7z'></path>
        </svg>
        <span>ホームに戻る</span>
      </a>
    </nav>
  </header>

  <section
    class='filter-panel mb-6 bg-[#373c47] p-5 rounded-lg border border-border'
    aria-label='フィルタと並び替え'
  >
    <div class='flex flex-wrap items-center justify-between gap-4'>
      <div class='filter-group flex flex-wrap items-center gap-3'>
        <label for='filter-material' class='text-sm text-muted font-semibold flex items-center gap-2'>
          <svg class='w-4 h-4' fill='currentColor' viewBox='0 0 20 20'>
            <path d='M3 4a1 1 0 011-1h12a1 1 0 011 1v2a1 1 0 01-1 1H4a1 1 0 01-1-1V4zM3 10a1 1 0 011-1h6a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6zM14 9a1 1 0 00-1 1v6a1 1 0 001 1h2a1 1 0 001-1v-6a1 1 0 00-1-1h-2z'></path>
          </svg>
          マテリアル:
        </label>
        <select id='filter-material' bind:value={materialFilter} class='bg-[#1a1d21] border-2 border-border rounded-lg px-4 py-2.5 text-white font-sans min-w-[200px] transition-all duration-300 appearance-none [background-image:url("data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%23e0e0e0%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E")] [background-repeat:no-repeat] [background-position:right_12px_center] [background-size:10px] pr-[35px] hover:border-primary focus:border-primary focus:outline-none'>
          <option value="">すべてのマテリアル</option>
          {#each allMaterials as material}
            <option value={material}>{material}</option>
          {/each}
        </select>

        <label for='filter-id' class='text-sm text-muted font-semibold flex items-center gap-2'>
          <svg class='w-4 h-4' fill='currentColor' viewBox='0 0 20 20'>
            <path fill-rule='evenodd' d='M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z' clip-rule='evenodd'></path>
          </svg>
          ID:
        </label>
        <input
          type='text'
          id='filter-id'
          placeholder='例: sennsyuuraku'
          bind:value={idFilter}
          class='bg-[#1a1d21] border-2 border-border rounded-lg px-4 py-2.5 text-white font-mono min-w-[200px] transition-all duration-300 hover:border-primary focus:border-primary focus:outline-none'
        />
      </div>

      <div class='filter-group flex flex-wrap items-center gap-3'>
        <label for='sort-order' class='text-sm text-muted font-semibold flex items-center gap-2'>
          <svg class='w-4 h-4' fill='currentColor' viewBox='0 0 20 20'>
            <path d='M3 3a1 1 0 000 2h11a1 1 0 100-2H3zM3 7a1 1 0 000 2h7a1 1 0 100-2H3zM3 11a1 1 0 100 2h4a1 1 0 100-2H3zM15 8a1 1 0 10-2 0v5.586l-1.293-1.293a1 1 0 00-1.414 1.414l3 3a1 1 0 001.414 0l3-3a1 1 0 00-1.414-1.414L15 13.586V8z'></path>
          </svg>
          並び順:
        </label>
        <select id='sort-order' bind:value={sortOrder} class='bg-[#1a1d21] border-2 border-border rounded-lg px-4 py-2.5 text-white font-sans min-w-[200px] transition-all duration-300 appearance-none [background-image:url("data:image/svg+xml;charset=US-ASCII,%3Csvg%20xmlns%3D%22http%3A%2F%2Fwww.w3.org%2F2000%2Fsvg%22%20width%3D%22292.4%22%20height%3D%22292.4%22%3E%3Cpath%20fill%3D%22%23e0e0e0%22%20d%3D%22M287%2069.4a17.6%2017.6%200%200%200-13-5.4H18.4c-5%200-9.3%201.8-12.9%205.4A17.6%2017.6%200%200%200%200%2082.2c0%205%201.8%209.3%205.4%2012.9l128%20127.9c3.6%203.6%207.8%205.4%2012.8%205.4s9.2-1.8%2012.8-5.4L287%2095c3.5-3.5%205.4-7.8%205.4-12.8%200-5-1.9-9.2-5.5-12.8z%22%2F%3E%3C%2Fsvg%3E")] [background-repeat:no-repeat] [background-position:right_12px_center] [background-size:10px] pr-[35px] hover:border-primary focus:border-primary focus:outline-none'>
          <option value='date_desc'>追加日が新しい順</option>
          <option value='date_asc'>追加日が古い順</option>
          <option value='name_asc'>名前 (A-Z)</option>
          <option value='name_desc'>名前 (Z-A)</option>
        </select>
      </div>
    </div>

    <div class='stats mt-4 pt-4 border-t border-border text-sm text-muted flex items-center gap-2' aria-live='polite'>
      <svg class='w-4 h-4' fill='currentColor' viewBox='0 0 20 20'>
        <path d='M9 2a1 1 0 000 2h2a1 1 0 100-2H9z'></path>
        <path fill-rule='evenodd' d='M4 5a2 2 0 012-2 3 3 0 003 3h2a3 3 0 003-3 2 2 0 012 2v11a2 2 0 01-2 2H6a2 2 0 01-2-2V5zm3 4a1 1 0 000 2h.01a1 1 0 100-2H7zm3 0a1 1 0 000 2h3a1 1 0 100-2h-3zm-3 4a1 1 0 100 2h.01a1 1 0 100-2H7zm3 0a1 1 0 100 2h3a1 1 0 100-2h-3z' clip-rule='evenodd'></path>
      </svg>
      {#if filteredModels.length === models.length}
        <span class='font-semibold'>全 <span class='text-primary'>{models.length}</span> 個のカスタムモデル</span>
      {:else}
        <span class='font-semibold'><span class='text-primary'>{filteredModels.length}</span> / {models.length} 個のカスタムモデルを表示中</span>
      {/if}
    </div>
  </section>

  <section class='gallery grid grid-cols-[repeat(auto-fill,minmax(280px,1fr))] gap-5'>
    {#if error}
      <div class='error col-span-full bg-red-900/20 border border-red-500/50 rounded-lg p-6 text-center' role='alert'>
        <svg class='w-12 h-12 mx-auto mb-3 text-red-400' fill='currentColor' viewBox='0 0 20 20'>
          <path fill-rule='evenodd' d='M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7 4a1 1 0 11-2 0 1 1 0 012 0zm-1-9a1 1 0 00-1 1v4a1 1 0 102 0V6a1 1 0 00-1-1z' clip-rule='evenodd'></path>
        </svg>
        <strong class='text-red-300 text-lg'>{error}</strong>
      </div>
    {:else}
      {#each filteredModels.models as model (model.name)}
        <ModelCard {model} />
      {:else}
        <div class='col-span-full text-center py-12 bg-[#373c47] rounded-lg border border-border' role='status'>
          <svg class='w-16 h-16 mx-auto mb-4 text-muted/50' fill='currentColor' viewBox='0 0 20 20'>
            <path fill-rule='evenodd' d='M8 4a4 4 0 100 8 4 4 0 000-8zM2 8a6 6 0 1110.89 3.476l4.817 4.817a1 1 0 01-1.414 1.414l-4.816-4.816A6 6 0 012 8z' clip-rule='evenodd'></path>
          </svg>
          <p class='text-lg text-muted mb-2'>条件に一致するカスタムモデルが見つかりませんでした</p>
          <p class='text-sm text-muted/70'>フィルタ条件を変更してみてください</p>
        </div>
      {/each}
    {/if}
  </section>
</main>
