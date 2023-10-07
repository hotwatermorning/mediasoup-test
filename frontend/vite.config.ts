import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite'
import svelteSVG from 'vite-plugin-svelte-svg';
import StringReplace from 'vite-plugin-string-replace';

export default defineConfig({
  plugins: [
    sveltekit(),
    svelteSVG({
      svgoConfig: {}, // See https://github.com/svg/svgo#configuration
      requireSuffix: true // Set false to accept '.svg' without the '?component'
    }),
    StringReplace([
      {
        search: 'css-breakpoint-sp', // search this string in content
        replace: '650px' // replace search string with this
      }
    ])
  ],
  resolve: {},
});