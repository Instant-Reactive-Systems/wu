import flattenColorPalette from 'tailwindcss/lib/util/flattenColorPalette';
import { withAlphaValue } from 'tailwindcss/lib/util/withAlphaVariable';

/*
Fancy styling for links.

# Usage

```html
<a href="#" class="link link-primary">{link}</a>
```
*/
export default ({ matchUtilities, addUtilities, theme }) => {
	addUtilities({
		'.link': {
			'@apply cursor-pointer decoration-2 underline-offset-1': {},
			'@apply hover:brightness-[1.15] focus:brightness-[1.15]': {},
			'@apply no-underline [@media(hover:hover)]:hover:underline focus-within:underline': {},
		},
	});

	matchUtilities(
		{
			'link': (value, options) => ({
				textDecorationColor: withAlphaValue(typeof value === 'function' ? value(options) : value, 1),
				color: withAlphaValue(typeof value === 'function' ? value(options) : value, 1),
			}),
		},
		{ values: flattenColorPalette(theme('colors')), type: ['color'] },
	);
};
