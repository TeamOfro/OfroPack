<script lang='ts'>
  import type { PageData } from './$types';
  import InfoCard from '$lib/components/InfoCard.svelte';
  import { assetUrl } from '$lib/url';

  const { data }: { data: PageData } = $props();

  const { metadata, error } = data;

  // ファイルサイズを人間が読みやすい形式に変換
  const formatSize = (bytes: number): string => {
    if (bytes > 1024 * 1024) {
      return `${(bytes / (1024 * 1024)).toFixed(2)} MB`;
    }
    else if (bytes > 1024) {
      return `${(bytes / 1024).toFixed(2)} KB`;
    }
    return `${bytes} bytes`;
  };

  const size = metadata ? formatSize(metadata.size) : undefined;
  const updated = metadata
    ? new Date(metadata.updated_at).toLocaleString('ja-JP', {
      year: 'numeric',
      month: '2-digit',
      day: '2-digit',
      hour: '2-digit',
      minute: '2-digit',
    })
    : undefined;
  const sha1 = metadata?.sha1;
  const version = metadata?.version;

  function copyToClipboard(text: string, button: HTMLButtonElement) {
    navigator.clipboard
      .writeText(text)
      .then(() => {
        const originalText = button.textContent;
        button.textContent = '✓ コピー完了';
        button.style.background = '#3a8f3e';

        setTimeout(() => {
          button.textContent = originalText;
          button.style.background = '';
        }, 2000);
      })
      .catch((err) => {
        console.error('Copy failed:', err);
      });
  }
</script>

<svelte:head>
  <title>ホーム - OfroPack</title>
  <meta name='description' content='Ofro鯖のMinecraftリソースパック - カスタムモデルデータを簡単に管理' />
</svelte:head>

<main
  class='container mx-auto max-w-3xl rounded-lg bg-card-bg p-6 shadow-[0_8px_24px_rgba(0,0,0,0.4)]'
>
  <header>
    <h1 class='mb-2 text-center text-4xl font-bold text-primary'>🎨 OfroPack</h1>
    <p class='mb-4 text-center text-lg text-muted'>Ofro鯖のMinecraftリソースパック</p>
    {#if version}
      <p class='mb-6 text-center text-sm text-muted/70'>
        <span class='font-mono bg-background/50 px-2 py-1 rounded'>v{version}</span>
      </p>
    {/if}
  </header>

  {#if error}
    <div class='error mb-4 bg-red-900/20 border border-red-500/50 rounded-lg p-4' role='alert'>
      <strong class='text-red-400'>⚠️ エラーが発生しました</strong><br />
      <span class='text-red-300'>{error}</span><br />
      <small class='text-red-400/70'>ページを再読み込みしてください。</small>
    </div>
  {/if}

  <nav class='mb-6 text-center' aria-label='主要アクション'>
    <a
      href={assetUrl('/OfroPack.zip')}
      class='inline-block bg-primary text-white py-3 px-8 no-underline rounded-lg text-lg font-bold transition-all duration-300 hover:bg-primary-hover hover:-translate-y-0.5 hover:shadow-lg m-1'
      download
    >
      ⬇️ ダウンロード
    </a>
    <a
      href={assetUrl('/gallery')}
      class='inline-block bg-transparent border-2 border-primary text-primary py-3 px-8 no-underline rounded-lg text-lg font-bold transition-all duration-300 hover:bg-primary hover:text-white m-1'
    >
      🎨 ギャラリー
    </a>
  </nav>

  <section class='info-grid mb-6 grid grid-cols-1 gap-4 sm:grid-cols-2 md:grid-cols-4' aria-label='パック情報'>
    <InfoCard title='📦 サイズ'>
      <span class='text-primary font-bold text-lg'>{size || 'N/A'}</span>
    </InfoCard>
    <InfoCard title='🕒 最終更新'>
      <span class='text-sm'>{updated || 'N/A'}</span>
    </InfoCard>
    <InfoCard title='🔖 バージョン'>
      <span class='font-mono text-sm'>{version || 'N/A'}</span>
    </InfoCard>
    <InfoCard title='🚀 最新 PR'>
      {#if metadata?.latest_pr}
        <a
          href={metadata.latest_pr.url}
          target='_blank'
          rel='noopener'
          class='text-primary no-underline font-bold transition-colors duration-300 hover:text-primary-hover hover:underline flex items-center gap-1'
          title={metadata.latest_pr.title}
        >
          <span>#{metadata.latest_pr.number}</span>
          <svg class='w-3 h-3' fill='currentColor' viewBox='0 0 20 20'>
            <path d='M11 3a1 1 0 100 2h2.586l-6.293 6.293a1 1 0 101.414 1.414L15 6.414V9a1 1 0 102 0V4a1 1 0 00-1-1h-5z'></path>
            <path d='M5 5a2 2 0 00-2 2v8a2 2 0 002 2h8a2 2 0 002-2v-3a1 1 0 10-2 0v3H5V7h3a1 1 0 000-2H5z'></path>
          </svg>
        </a>
      {:else}
        <span class='text-muted'>N/A</span>
      {/if}
    </InfoCard>
  </section>

  {#if metadata?.latest_pr}
    <section class='mb-6 bg-[#373c47] p-4 rounded-lg border-l-4 border-primary'>
      <h3 class='mt-0 mb-2 text-sm text-muted uppercase flex items-center gap-2'>
        <svg class='w-4 h-4' fill='currentColor' viewBox='0 0 20 20'>
          <path fill-rule='evenodd' d='M6 2a1 1 0 00-1 1v1H4a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V6a2 2 0 00-2-2h-1V3a1 1 0 10-2 0v1H7V3a1 1 0 00-1-1zm0 5a1 1 0 000 2h8a1 1 0 100-2H6z' clip-rule='evenodd'></path>
        </svg>
        最新の変更
      </h3>
      <p class='m-0 text-base'>
        <a href={metadata.latest_pr.url} target='_blank' rel='noopener' class='text-white no-underline hover:text-primary transition-colors'>
          {metadata.latest_pr.title}
        </a>
      </p>
    </section>
  {/if}

  <section aria-label='ダウンロードリンク'>
    <div class='bg-[#373c47] p-4 rounded-lg'>
      <h3 class='mt-0 mb-2 text-sm text-muted uppercase flex items-center gap-2'>
        <svg class='w-4 h-4' fill='currentColor' viewBox='0 0 20 20'>
          <path fill-rule='evenodd' d='M2.166 4.999A11.954 11.954 0 0010 1.944 11.954 11.954 0 0017.834 5c.11.65.166 1.32.166 2.001 0 5.225-3.34 9.67-8 11.317C5.34 16.67 2 12.225 2 7c0-.682.057-1.35.166-2.001zm11.541 3.708a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z' clip-rule='evenodd'></path>
        </svg>
        SHA1 ハッシュ
      </h3>
      <div class='flex items-center justify-between bg-background p-3 rounded-md'>
        <code class='font-mono text-sm whitespace-nowrap overflow-hidden text-ellipsis flex-1 select-all'>{sha1 || 'N/A'}</code>
        {#if sha1}
          <button
            class='bg-primary text-white border-none py-2 px-4 rounded-md cursor-pointer text-sm transition-all duration-300 whitespace-nowrap hover:bg-primary-hover hover:shadow-md ml-2'
            onclick={e => copyToClipboard(sha1, e.currentTarget)}
          >
            📋 コピー
          </button>
        {/if}
      </div>
    </div>
  </section>
</main>

<footer class='mt-8 text-center text-sm text-muted/70'>
  <p class='mb-1'>
    <a href='https://github.com/TeamOfro/OfroPack' target='_blank' rel='noopener' class='text-primary no-underline hover:underline'>
      GitHub リポジトリ
    </a>
  </p>
  <p>&copy; {new Date().getFullYear()} OfroPack. All Rights Reserved.</p>
</footer>
