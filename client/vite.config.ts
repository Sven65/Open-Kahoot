import { defineConfig } from 'vite'
import preact from '@preact/preset-vite'

import tailwindcss from 'tailwindcss'
import autoprefixer from 'autoprefixer'

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [
		preact(),
	],
	css: {
		postcss: {
		  plugins: [
				tailwindcss,
				// @ts-ignore
				autoprefixer,
		  ],
		},
	},
	server: {
		proxy: {
			'/api': {
				target: 'http://localhost:3000',
				changeOrigin: true,
				secure: false,
			},
			'/socket.io': {
				target: 'http://localhost:3000',
				changeOrigin: false,
				secure: false,
			},
		},
	},
})
