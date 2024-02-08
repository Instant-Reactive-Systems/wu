import plugin from 'tailwindcss/plugin';
import utilitiesPlugin from './utilities.plugin.js';
import componentsPlugin from './components.plugin.js';

export default plugin(
	(params) => {
		utilitiesPlugin(params);
		componentsPlugin(params);
	},
);
