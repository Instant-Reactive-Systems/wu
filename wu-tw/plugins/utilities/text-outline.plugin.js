import flattenColorPalette from 'tailwindcss/lib/util/flattenColorPalette';

/*
Adds outline to text.

# Usage

```html
<h1 class="text-outline-8 text-outline-primary">{title}</h1>
```
*/
export default ({ addUtilities, matchUtilities, theme }) => {
	addUtilities({
		'.text-outline': {
			'-webkit-text-stroke-width': '1px',
		},
	});

	matchUtilities(
		{
			'text-outline': (value) => ({
				'-webkit-text-stroke-width': value,
			}), 
		},
		{ values: theme('textStrokeWidth'), type: 'length' },
	);

	matchUtilities(
		{
			'text-outline': (value) => ({
				'-webkit-text-stroke-color': value,
			}), 
		},
		{ values: flattenColorPalette(theme('colors')), type: 'color' },
	);
};
