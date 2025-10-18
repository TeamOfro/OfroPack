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
  const hashUrl = metadata ? metadata.download_url.replace('OfroPack.zip', 'hash.txt') : errorText;

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
  class='container mx-auto max-w-3xl rounded-lg bg-[--card-bg-color] p-6 shadow-[0_8px_24px_rgba(0,0,0,0.4)]'
>
  <header>
    <h1 class='mb-2 text-center text-3xl font-bold text-[--primary-color]'>🎨 OfroPack</h1>
    <p class='mb-6 text-center text-[--muted-text-color]'>Ofro鯖のMinecraftリソースパック</p>
  </header>

  {#if error}
    <div class='error mb-4' role='alert'>
      <strong>エラーが発生しました</strong><br />
      {error}<br />
      <small>ページを再読み込みしてください。</small>
    </div>
  {/if}

  <nav class='mb-6 text-center' aria-label='主要アクション'>
    <a href='/OfroPack.zip' class='download-btn' download>⬇️ ダウンロード</a>
    <a href='/gallery' class='download-btn secondary'>🎨 ギャラリー</a>
  </nav>

  <section class='info-grid mb-5 grid grid-cols-1 gap-4 sm:grid-cols-3' aria-label='パック情報'>
    <div class='info-box'>
      <h3>サイズ</h3>
      <p>{size}</p>
    </div>
    <div class='info-box'>
      <h3>最終更新</h3>
      <p>{updated}</p>
    </div>
    <div class='info-box'>
      <h3>最新 PR</h3>
      <p>
        {#if latestPr?.number}
          <a href={latestPr.url} target='_blank' rel='noopener'>#{latestPr.number}</a>
        {:else}
          {errorText}
        {/if}
      </p>
    </div>
  </section>

  <section aria-label='ダウンロードリンク'>
    <div class='url-box'>
      <h3>SHA1 ハッシュ</h3>
      <div class='copyable-field'>
        <code>{sha1}</code>
        <button class='copy-btn' onclick={e => copyToClipboard(sha1, e.currentTarget)}>コピー</button
        >
      </div>
    </div>

    <div class='url-box'>
      <h3>リソースパック URL</h3>
      <div class='copyable-field'>
        <code>{packUrl}</code>
        <button class='copy-btn' onclick={e => copyToClipboard(packUrl, e.currentTarget)}
        >コピー</button
        >
      </div>
    </div>

    <div class='url-box'>
      <h3>ハッシュ URL</h3>
      <div class='copyable-field'>
        <code>{hashUrl}</code>
        <button class='copy-btn' onclick={e => copyToClipboard(hashUrl, e.currentTarget)}
        >コピー</button
        >
      </div>
    </div>
  </section>
</main>

<div class='mt-6 text-center text-xs text-[--muted-text-color]'>
  <p>&copy; {new Date().getFullYear()} OfroPack. All Rights Reserved.</p>
</div>

<style>
	.download-btn {
		display: inline-block;
		background: var(--primary-color);
		color: white;
		padding: 12px 25px;
		text-decoration: none;
		border-radius: var(--border-radius);
		font-size: 1.1em;
		font-weight: bold;
		transition: background-color 0.3s, transform 0.2s;
		margin: 5px;
	}

	.download-btn:hover {
		background: var(--primary-hover-color);
		transform: translateY(-2px);
	}

	.download-btn.secondary {
		background: transparent;
		border: 1px solid var(--primary-color);
		color: var(--primary-color);
	}

	.download-btn.secondary:hover {
		background: var(--primary-color);
		color: white;
	}

	.info-box {
		background: #373c47;
		padding: 15px;
		border-radius: var(--border-radius);
	}

	.info-box h3 {
		margin-top: 0;
		margin-bottom: 8px;
		font-size: 0.9em;
		color: var(--muted-text-color);
		text-transform: uppercase;
	}

	.info-box p {
		margin: 0;
		font-size: 1em;
		font-weight: bold;
		word-wrap: break-word;
	}

	.info-box a {
		color: var(--primary-color);
		text-decoration: none;
		font-weight: bold;
		transition: color 0.3s;
	}

	.info-box a:hover {
		color: var(--primary-hover-color);
		text-decoration: underline;
	}

	.url-box {
		background: #373c47;
		padding: 15px;
		border-radius: var(--border-radius);
		margin-bottom: 10px;
	}

	.url-box h3 {
		margin-top: 0;
		margin-bottom: 8px;
		font-size: 0.9em;
		color: var(--muted-text-color);
		text-transform: uppercase;
	}

	.copyable-field {
		display: flex;
		align-items: center;
		justify-content: space-between;
		background: var(--bg-color);
		padding: 8px 12px;
		border-radius: 5px;
	}

	.copyable-field code {
		font-family: var(--font-family-mono);
		font-size: 0.9em;
		white-space: nowrap;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.copy-btn {
		background: var(--primary-color);
		color: white;
		border: none;
		padding: 7px 10px;
		border-radius: 5px;
		cursor: pointer;
		font-size: 0.85em;
		transition: background-color 0.3s;
		white-space: nowrap;
	}

	.copy-btn:hover {
		background: var(--primary-hover-color);
	}
</style>
