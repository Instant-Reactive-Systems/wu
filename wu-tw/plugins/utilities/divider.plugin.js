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
export default ({ addUtilities }) => {
	addUtilities({
		'.hdivider': {
			'@apply flex flex-row items-center self-stretch': {},
			'&:before, &:after': {
				'content': '""',
				'@apply flex-grow h-0.5 w-full bg-surface-400 dark:bg-surface-500 bg-opacity-10': {},
			},
			'&:not(:empty)': {
				'@apply gap-2': {},
			},
		},
		'.vdivider': {
			'@apply flex flex-col items-center self-stretch': {},
			'&:before, &:after': {
				'content': '""',
				'@apply flex-grow w-0.5 h-full bg-surface-400 dark:bg-surface-500 bg-opacity-10': {},
			},
			'&:not(:empty)': {
				'@apply gap-2': {},
			},
		},
	});
};
