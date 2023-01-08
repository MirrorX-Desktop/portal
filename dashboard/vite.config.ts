import { sveltekit } from '@sveltejs/kit/vite';
import type { UserConfig } from 'vite';

const config: UserConfig = {
	plugins: [sveltekit()],

	server: {
		proxy: {
			'/api': {
				target: 'http://localhost:6500',
				changeOrigin: true
			}
		}
	}
};

export default config;
