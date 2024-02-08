import { plugins, presets } from './wu-tw/index';

/** @type {import('tailwindcss').Config} */
export default {
	content: {
		files: ["*.html", "./src/**/*.rs"],
	},
	presets: [
		...presets,
	],
	theme: {
		extend: {},
	},
	plugins: [
		plugins,
	],
};
