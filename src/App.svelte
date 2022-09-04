<script lang="ts">
  import { SvelteToast } from '@zerodevx/svelte-toast'
  import GifResults from './components/GifResults.svelte';
  import { gifSearchBaseUrl } from './lib/constants';
  import type { GifApiResponse } from './lib/types';
  import { search } from './stores/search';

  const handleKeydown = async (event: KeyboardEvent) => {
    if (event.code === 'Enter') {
      search.update((state) => ({
        ...state,
        gifs: [],
        errorMessage: '',
        isLoading: true,
      }));

      const response = await fetch(`${gifSearchBaseUrl}/${encodeURIComponent($search.text)}`, {
        method: 'GET',
      });

      const { results, error } = (await response.json()) as GifApiResponse;

      if (error) {
        search.update((state) => ({
          ...state,
          gifs: [],
          errorMessage: error,
          isLoading: false,
        }));
      } else if (results) {
        search.update((state) => ({
          ...state,
          errorMessage: '',
          gifs: results,
          isLoading: false,
        }));
      }
    }
  };
</script>

<div class="input-container">
  <div class="search-field">
    <input
      type="text"
      placeholder="Hit enter to search"
      bind:value={$search.text}
      on:keydown={handleKeydown}
    />
  </div>
</div>

{#if $search.isLoading}
  <p>loading...</p>
{/if}

{#if $search.errorMessage !== ''}
  <strong>{$search.errorMessage}</strong>
{/if}

{#if $search.gifs.length > 0}
  <GifResults />
{/if}

<SvelteToast  />

<style lang="sass">
  :global(body)
    margin: 20px 10px
    font-family: Arial, Helvetica, sans-serif
    font-size: 18px
    background-color: #111318
    color: #f2f2f2
    text-align: center
  .input-container
    display: flex
    flex-direction: row
    margin: 0 30px
    align-items: center
    gap: 0.75rem
    .search-field
      flex-grow: 1
      input[type=text]
        background-color: #1f222b
        color: white
        border: 1px solid #0b0c0f
        outline: none
        padding: 6px 10px
        font-size: 14px
        border-radius: 6px
        width: 100%
  strong
    color: red
</style>
