import { Writable, writable } from 'svelte/store'

export const search: Writable<{
  text: string
  gifs: any[]
  errorMessage: string
  isLoading: boolean
}> = writable({
  text: '',
  gifs: [],
  errorMessage: '',
  isLoading: false,
})
