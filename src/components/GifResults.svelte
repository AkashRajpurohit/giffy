<script lang="ts">
  import type { MediaEntity } from 'src/lib/types';
  import { search } from '../stores/search';
  import PoweredByTenor from './PoweredByTenor.svelte';
  import { writeText } from '@tauri-apps/api/clipboard';
  import { toast } from '@zerodevx/svelte-toast';

  function getAppropriateGif(media: MediaEntity) {
    // TODO: Take input from user regarding the quality
    // of gif to consider??

    if (media?.tinygif) return media.tinygif;
    if (media?.mediumgif) return media.mediumgif;
    return media?.gif;
  }

  function getHighResolutionGif(media: MediaEntity) {
    if (media?.gif) return media.gif;
    if (media?.mediumgif) return media.mediumgif;
    if (media?.tinygif) return media.tinygif;
    return media?.nanogif;
  }

  function getPreviewImageUrl(media: MediaEntity) {
    return getAppropriateGif(media).preview;
  }

  function getCardStyles(media: MediaEntity) {
    let className = `card `;
    const gif = getAppropriateGif(media);
    const [width, height] = gif.dims;

    if (height / width > 1) {
      className += 'card_large';
    } else if (height / width > 0.5) {
      className += 'card_medium';
    } else {
      className += 'card_small';
    }

    return className;
  }

  async function copyImageToClipboard(media: MediaEntity) {
    const gif = getHighResolutionGif(media);
    const url = gif.url;

    await writeText(url);
    toast.push('Copied to Clipboard', {
      theme: {
        '--toastBackground': '#48BB78',
        '--toastBarBackground': '#2F855A',
      },
    });
  }

  function swapWithGifUrl(
    element: (MouseEvent | FocusEvent) & {
      currentTarget: EventTarget & HTMLImageElement;
    },
    media: MediaEntity
  ) {
    const image = element.currentTarget;
    const gif = getAppropriateGif(media);

    image.src = gif.url;
  }

  function swapWithPreviewUrl(
    element: (MouseEvent | FocusEvent) & {
      currentTarget: EventTarget & HTMLImageElement;
    },
    media: MediaEntity
  ) {
    const image = element.currentTarget;
    const gif = getAppropriateGif(media);
    
    image.src = gif.preview;
  }
</script>

<div>
  <div class="results_container">
    {#each $search.gifs as item}
      <div
        class={getCardStyles((item?.media || [])[0])}
        on:click={() => copyImageToClipboard((item?.media || [])[0])}
        title="Click to copy the GIF to clipboard"
      >
        <img
          on:mouseover={(e) => swapWithGifUrl(e, (item?.media || [])[0])}
          on:focus={(e) => swapWithGifUrl(e, (item?.media || [])[0])}
          on:mouseout={(e) => swapWithPreviewUrl(e, (item?.media || [])[0])}
          on:blur={(e) => swapWithPreviewUrl(e, (item?.media || [])[0])}
          src={getPreviewImageUrl((item?.media || [])[0])}
          alt={item.content_description ?? item.h1_title}
        />
      </div>
    {/each}
  </div>
  <div>
    <PoweredByTenor />
  </div>
</div>

<style>
  :root {
    --card_width: 300px;
    --row_increment: 10px;
    --card_border_radius: 16px;
    --card_small: 26;
    --card_medium: 33;
    --card_large: 45;
  }

  .results_container {
    width: 100%;

    display: grid;
    grid-template-columns: repeat(auto-fill, var(--card_width));
    grid-auto-rows: var(--row_increment);
    justify-content: center;
  }

  .card {
    margin: 10px;
    border-radius: var(--card_border_radius);
  }

  .card:hover {
    cursor: pointer;
    transition: all 0.3s;
    transform: scale(1.05);
  }

  .card img {
    width: 100%;
    height: 100%;
    border-radius: var(--card_border_radius);
    object-fit: cover;
  }

  .card_small {
    grid-row-end: span var(--card_small);
  }

  .card_medium {
    grid-row-end: span var(--card_medium);
  }

  .card_large {
    grid-row-end: span var(--card_large);
  }
</style>
