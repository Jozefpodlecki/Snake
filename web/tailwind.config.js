/** @type {import('tailwindcss').Config} */

export default {
	content: ['./src/**/*.{mjs,js,ts,jsx,tsx}'],
	theme: {
		extend: {
			fontFamily: {
				'sigmar': ['Sigmar', 'sans-serif'],
			}
		}
	},
	plugins: []
}
