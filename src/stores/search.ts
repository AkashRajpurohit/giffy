import type { ResultsEntity } from 'src/lib/types';
import { Writable, writable } from 'svelte/store';

export const search: Writable<{
  text: string;
  gifs: ResultsEntity[];
  errorMessage: string;
  isLoading: boolean;
}> = writable({
  text: '',
  gifs: [],
  errorMessage: '',
  isLoading: false,
});
