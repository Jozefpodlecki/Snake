import resolve from '@rollup/plugin-node-resolve';
import terser from '@rollup/plugin-terser';

const production = !process.env.ROLLUP_WATCH;

export default {
	input: 'index.js',
	output: {
		dir: 'public',
		format: 'es',
		sourcemap: true
	},
	plugins: [
		resolve(),
		production && terser()
	]
};