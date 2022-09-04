<script lang="ts">
  import GifResults from './components/GifResults.svelte'
  import { gifSearchBaseUrl } from './lib/constants'
  import { search } from './stores/search'

  const handleKeydown = async (event: KeyboardEvent) => {
    if (event.code === 'Enter') {
      search.update((state) => ({
        ...state,
        gifs: [],
        errorMessage: '',
        isLoading: true,
      }))

      const response = await fetch(`${gifSearchBaseUrl}/${encodeURIComponent($search.text)}`, {
        method: 'GET',
      })

      const { results, error } = await response.json()

      if (error) {
        search.update((state) => ({
          ...state,
          gifs: [],
          errorMessage: error,
          isLoading: false,
        }))
      } else {
        search.update((state) => ({
          ...state,
          errorMessage: '',
          gifs: results,
          isLoading: false,
        }))
      }
    }
  }
</script>

<h1>Giffy</h1>
<input type="search" bind:value={$search.text} on:keydown={handleKeydown} />

{#if $search.isLoading}
  <p>loading...</p>
{/if}

{#if $search.errorMessage !== ''}
  <strong>{$search.errorMessage}</strong>
{/if}

{#if $search.gifs.length > 0}
  <GifResults />
{/if}

<style lang="sass">
  :global(body)
    margin: 100px 0px
    font-family: Arial, Helvetica, sans-serif
    font-size: 18px
    background-color: #111318
    color: #f2f2f2
    text-align: center
  h1
    color: #ff3700
  code
    background: #282b33
    padding: 2px 5px
    border-radius: 4px
  strong
    color: red
</style>
