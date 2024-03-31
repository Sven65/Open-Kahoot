/** @type {import('tailwindcss').Config} */
export default {
	content: {
		files: [
			'./index.html',
			'./src/**/*.{js,ts,jsx,tsx,css,scss}',
		],
	},
	darkMode: 'selector',
	theme: {
		extend: {},
	},
	plugins: [
		require('@tailwindcss/forms'),
	],
	safelist: [
		{
			pattern: /bg-.+/,
		},
	],
}

