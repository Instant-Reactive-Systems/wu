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
			'@apply cursor-pointer decoration-2 underline-offset-1 no-underline [@media(hover:hover)]:hover:underline': {},
		},
	});

	matchUtilities(
		{
			'link': (value) => ({
				textDecorationColor: withAlphaValue(value, 1),
				color: withAlphaValue(value, 1),
			}),
		},
		{ values: flattenColorPalette(theme('colors')), type: ['color'] },
	);
};
