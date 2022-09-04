<script lang="ts">
  import { search } from '../stores/search'
</script>

<h1>Results for {$search.text}</h1>
<ul>
  {#each $search.gifs as item}
    <li>
      <img src={item.media[0]?.gif?.url} alt={item.content_description ?? item.h1_title} />
    </li>
  {/each}
</ul>
