<script lang="ts">
  import type { MediaEntity } from 'src/lib/types';
  import { search } from '../stores/search';
  import PoweredByTenor from './PoweredByTenor.svelte';

  function getAppropriateGif(media: MediaEntity) {
    // TODO: Take input from user regarding the quality
    // of gif to consider??

    // if (media?.nanogif) return media.nanogif;
    if (media?.tinygif) return media.tinygif;
    if (media?.mediumgif) return media.mediumgif;
    return media?.gif;
  }

  function getImageUrl(media: MediaEntity) {
    return getAppropriateGif(media).url;
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
</script>

<div>
  <div class="results_container">
    {#each $search.gifs as item}
      <div class={getCardStyles((item?.media || [])[0])}>
        <img
          src={getImageUrl((item?.media || [])[0])}
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
