import flattenColorPalette from 'tailwindcss/lib/util/flattenColorPalette';
import { withAlphaVariable, withAlphaValue } from 'tailwindcss/lib/util/withAlphaVariable';

/*
Used to separate content vertically or horizontally.

# Usage

## Vertical divider (A | B)
```html
<div class="vdivider"/>
```

## Horizontal divider A_B
```html
<div class="hdivider"/>
```
*/
export default ({ addUtilities, matchUtilities, theme }) => {
	addUtilities({
		'.hdivider': {
			'@apply flex flex-row items-center self-stretch opacity-65': {},
			'&:before, &:after': {
				'content': '""',
				'background-color': 'var(--wu-divider-color)',
				'@apply flex-grow h-px w-full': {},
			},
			'&:not(:empty)': {
				'@apply gap-2': {},
			},
		},
		'.vdivider': {
			'@apply flex flex-col items-center self-stretch opacity-65': {},
			'&:before, &:after': {
				'content': '""',
				'background-color': 'var(--wu-divider-color)',
				'@apply flex-grow w-px h-full': {},
			},
			'&:not(:empty)': {
				'@apply gap-2': {},
			},
		},
	});

	matchUtilities(
		{
			'divider': (value, options) => ({
				'--wu-divider-color': typeof value === 'function' ? value(options) : value,
			}),
		},
		{ values: flattenColorPalette(theme('colors')), type: ['color'] },
	);
};
