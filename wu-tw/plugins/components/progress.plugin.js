/*
Used to show the progress of a task or to show the passing of time.

# Usage

```html
<div class="radial" style:value=perc>{perc}</div>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.radial': {
			'--value': '0',
			'--size': '5rem',
			'--thickness': 'calc(var(--size) / 10)',
			'@apply w-[var(--size)] h-[var(--size)] bg-transparent rounded-full place-content-center inline-grid relative align-middle box-content': {},

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
				'background': 'radial-gradient(farthest-side, currentColor 98%, #0000) top/var(--thickness) var(--thickness) no-repeat, conic-gradient(currentColor calc(var(--value) * 1%), #0000 0)',
				'-webkit-mask': 'radial-gradient(farthest-side, #0000 calc(99% - var(--thickness)), #000 calc(100% - var(--thickness)))',
				'mask': 'radial-gradient(farthest-side, #0000 calc(99% - var(--thickness)), #000 calc(100% - var(--thickness)))',
			},
			'&:after': {
				'@apply bg-current': {},
				'inset': 'calc(50% - var(--thickness) / 2)',
				'transform': 'rotate(calc(var(--value) * 3.6deg - 90deg)) translate(calc(var(--size) / 2 - 50%))',
			},
		},
	});
};
