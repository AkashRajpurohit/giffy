<script lang="ts">
  import type { MediaEntity } from 'src/lib/types'

  import { search } from '../stores/search'

  function getCardStyles(gif: MediaEntity['gif']) {
    let className = `card `
    const [width, height] = gif.dims

    if (height / width > 1) {
      className += 'card_large'
    } else if (height / width > 0.5) {
      className += 'card_medium'
    } else {
      className += 'card_small'
    }

    return className
  }
</script>

<div class="results_container">
  {#each $search.gifs as item}
    <div class={getCardStyles((item?.media || [])[0]?.gif)}>
      <img src={(item?.media || [])[0]?.gif?.url} alt={item.content_description ?? item.h1_title} />
    </div>
  {/each}
</div>

<style>
  :root {
    --card_width: 250px;
    --row_increment: 10px;
    --card_border_radius: 16px;
    --card_small: 26;
    --card_medium: 33;
    --card_large: 45;
  }

  .results_container {
    width: 100vw;
    position: absolute;
    left: 50%;
    transform: translateX(-50%);

    display: grid;
    grid-template-columns: repeat(auto-fill, var(--card_width));
    grid-auto-rows: var(--row_increment);
    justify-content: center;
  }

  .card {
    padding: 0;
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
