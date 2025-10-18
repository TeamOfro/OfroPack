<script lang='ts'>
  import type { PageData } from './$types';

  const { data }: { data: PageData } = $props();

  const { metadata, error } = data;

  const errorText = 'N/A';
  const size = metadata ? `${(metadata.size / 1024).toFixed(2)} KB` : errorText;
  const updated = metadata ? new Date(metadata.updated_at).toLocaleString('ja-JP') : errorText;
  const latestPr = metadata?.latest_pr;
  const sha1 = metadata?.sha1 || errorText;
  const packUrl = metadata?.download_url || errorText;
  const hashUrl = metadata?.download_url ? metadata.download_url.replace('OfroPack.zip', 'hash.txt') : errorText;

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
</svelte:head>

<main
  class='container mx-auto max-w-3xl rounded-lg bg-card-bg p-6 shadow-[0_8px_24px_rgba(0,0,0,0.4)]'
>
  <header>
    <h1 class='mb-2 text-center text-3xl font-bold text-primary'>🎨 OfroPack</h1>
    <p class='mb-6 text-center text-muted'>Ofro鯖のMinecraftリソースパック</p>
  </header>

  {#if error}
    <div class='error mb-4' role='alert'>
      <strong>エラーが発生しました</strong><br />
      {error}<br />
      <small>ページを再読み込みしてください。</small>
    </div>
  {/if}

  <nav class='mb-6 text-center' aria-label='主要アクション'>
    <a href='/OfroPack.zip' class='inline-block bg-primary text-white py-3 px-6 no-underline rounded-lg text-lg font-bold transition-all duration-300 hover:bg-primary-hover hover:-translate-y-0.5 m-1' download>⬇️ ダウンロード</a>
    <a href='/gallery' class='inline-block bg-transparent border border-primary text-primary py-3 px-6 no-underline rounded-lg text-lg font-bold transition-all duration-300 hover:bg-primary hover:text-white m-1'>🎨 ギャラリー</a>
  </nav>

  <section class='info-grid mb-5 grid grid-cols-1 gap-4 sm:grid-cols-3' aria-label='パック情報'>
    <div class='bg-[#373c47] p-4 rounded-lg'>
      <h3 class='mt-0 mb-2 text-sm text-muted uppercase'>サイズ</h3>
      <p class='m-0 text-base font-bold break-words'>{size}</p>
    </div>
    <div class='bg-[#373c47] p-4 rounded-lg'>
      <h3 class='mt-0 mb-2 text-sm text-muted uppercase'>最終更新</h3>
      <p class='m-0 text-base font-bold break-words'>{updated}</p>
    </div>
    <div class='bg-[#373c47] p-4 rounded-lg'>
      <h3 class='mt-0 mb-2 text-sm text-muted uppercase'>最新 PR</h3>
      <p class='m-0 text-base font-bold break-words'>
        {#if latestPr?.number}
          <a href={latestPr.url} target='_blank' rel='noopener' class='text-primary no-underline font-bold transition-colors duration-300 hover:text-primary-hover hover:underline'>#{latestPr.number}</a>
        {:else}
          {errorText}
        {/if}
      </p>
    </div>
  </section>

  <section aria-label='ダウンロードリンク'>
    <div class='bg-[#373c47] p-4 rounded-lg mb-2.5'>
      <h3 class='mt-0 mb-2 text-sm text-muted uppercase'>SHA1 ハッシュ</h3>
      <div class='flex items-center justify-between bg-background p-2 rounded-md'>
        <code class='font-mono text-sm whitespace-nowrap overflow-hidden text-ellipsis'>{sha1}</code>
        <button class='bg-primary text-white border-none py-1.5 px-2.5 rounded-md cursor-pointer text-sm transition-colors duration-300 whitespace-nowrap hover:bg-primary-hover' onclick={e => copyToClipboard(sha1, e.currentTarget)}>コピー</button>
      </div>
    </div>

    <div class='bg-[#373c47] p-4 rounded-lg mb-2.5'>
      <h3 class='mt-0 mb-2 text-sm text-muted uppercase'>リソースパック URL</h3>
      <div class='flex items-center justify-between bg-background p-2 rounded-md'>
        <code class='font-mono text-sm whitespace-nowrap overflow-hidden text-ellipsis'>{packUrl}</code>
        <button class='bg-primary text-white border-none py-1.5 px-2.5 rounded-md cursor-pointer text-sm transition-colors duration-300 whitespace-nowrap hover:bg-primary-hover' onclick={e => copyToClipboard(packUrl, e.currentTarget)}>コピー</button>
      </div>
    </div>

    <div class='bg-[#373c47] p-4 rounded-lg mb-2.5'>
      <h3 class='mt-0 mb-2 text-sm text-muted uppercase'>ハッシュ URL</h3>
      <div class='flex items-center justify-between bg-background p-2 rounded-md'>
        <code class='font-mono text-sm whitespace-nowrap overflow-hidden text-ellipsis'>{hashUrl}</code>
        <button class='bg-primary text-white border-none py-1.5 px-2.5 rounded-md cursor-pointer text-sm transition-colors duration-300 whitespace-nowrap hover:bg-primary-hover' onclick={e => copyToClipboard(hashUrl, e.currentTarget)}>コピー</button>
      </div>
    </div>
  </section>
</main>

<div class='mt-6 text-center text-xs text-muted'>
  <p>&copy; {new Date().getFullYear()} OfroPack. All Rights Reserved.</p>
</div>
