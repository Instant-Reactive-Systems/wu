/*
Used to show the progress of a task or to show the passing of time.

# Usage

```html
<div class="radial radial-size-4" style:('--wu-radial-value', perc)>{perc}</div>
```
*/
export default ({ addComponents, matchComponents, theme }) => {
	addComponents({
		'.radial': {
			'--wu-radial-value': '0',
			'--wu-radial-size': '5rem',
			'--wu-radial-thickness': 'calc(var(--wu-radial-size) / 10)',
			'@apply w-[var(--wu-radial-size)] h-[var(--wu-radial-size)] bg-transparent rounded-full place-content-center inline-grid relative align-middle box-content': {},

			'&::-moz-progress-bar': {
				'@apply bg-transparent appearance-none': {},
			},
			'&::-webkit-progress-value': {
				'@apply bg-transparent appearance-none': {},
			},
			'&::-webkit-progress-bar': {
				'@apply bg-transparent appearance-none': {},
			},
			'&:before, &:after': {
				'@apply absolute rounded-full content-[""]': {},
			},
			'&:before': {
				'@apply inset-0': {},
				'background': 'radial-gradient(farthest-side, currentColor 98%, #0000) top/var(--wu-radial-thickness) var(--wu-radial-thickness) no-repeat, conic-gradient(currentColor calc(var(--wu-radial-value) * 1%), #0000 0)',
				'-webkit-mask': 'radial-gradient(farthest-side, #0000 calc(99% - var(--wu-radial-thickness)), #000 calc(100% - var(--wu-radial-thickness)))',
				'mask': 'radial-gradient(farthest-side, #0000 calc(99% - var(--wu-radial-thickness)), #000 calc(100% - var(--wu-radial-thickness)))',
			},
			'&:after': {
				'@apply bg-current': {},
				'inset': 'calc(50% - var(--wu-radial-thickness) / 2)',
				'transform': 'rotate(calc(var(--wu-radial-value) * 3.6deg - 90deg)) translate(calc(var(--wu-radial-size) / 2 - 50%))',
			},
		},
	});

	matchComponents(
		{
			'radial-size': (value) => ({
				'--wu-radial-size': value,
			}),
		},
		{ values: theme('width'), type: 'length' }
	);

	matchComponents(
		{
			'radial-thickness': (value) => ({
				'--wu-radial-thickness': value,
			}),
		},
		{ values: theme('width'), type: 'length' }
	);
};
