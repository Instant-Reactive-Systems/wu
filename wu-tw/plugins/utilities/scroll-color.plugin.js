import flattenColorPalette from 'tailwindcss/lib/util/flattenColorPalette';
import { withAlphaVariable, withAlphaValue } from 'tailwindcss/lib/util/withAlphaVariable';

/*
Styles the colors for a scrollbar cross-browser.

# Usage

```html
<div class="scroll-light-1 thumb-light-2"/>
```
*/
export default ({ matchUtilities, theme }) => {
	matchUtilities(
		{
			'scroll': (value, options) => ({
				'--wu-scrollbar-track': typeof value === 'function' ? value(options) : value,
				'scrollbar-color': 'var(--wu-scrollbar-thumb) var(--wu-scrollbar-track)',
			}),
		},
		{ values: flattenColorPalette(theme('colors')), type: 'color' },
	);

	matchUtilities(
		{
			'thumb': (value, options) => ({
				'--wu-scrollbar-thumb': typeof value === 'function' ? value(options) : value,
				'scrollbar-color': 'var(--wu-scrollbar-thumb) var(--wu-scrollbar-track)',
			}),
		},
		{ values: flattenColorPalette(theme('colors')), type: 'color' },
	);
};
