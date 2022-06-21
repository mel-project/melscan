import adapter from '@sveltejs/adapter-auto';
import preprocess from 'svelte-preprocess';
import path from 'path';
const config = {
	// Consult https://github.com/sveltejs/svelte-preprocess
	// for more information about preprocessors
	preprocess: preprocess(),

	kit: {
		adapter: adapter(),
		vite:{
			resolve: {
				alias: {
                    // these are the aliases and paths to them
					'@utils': path.resolve('./src/utils'),
				}
			}
		}
	}

};

export default config;
