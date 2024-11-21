import flattenColorPalette from 'tailwindcss/lib/util/flattenColorPalette';

/*
Used to show a list of steps in a process.

# Usage

## Horizontal steps
```html
<ul class="hsteps">
  <li class="step step-primary">Register</li>
  <li class="step step-primary">Choose plan</li>
  <li class="step">Purchase</li>
  <li class="step">Receive Product</li>
</ul>
```

## Vertical steps
```html
<ul class="vsteps">
  <li class="step step-primary">Register</li>
  <li class="step step-primary">Choose plan</li>
  <li class="step">Purchase</li>
  <li class="step">Receive Product</li>
</ul>
```
*/
export default ({ addComponents, matchComponents, theme }) => {
	addComponents({
		'.hsteps': {
			'@apply inline-grid grid-flow-col overflow-hidden overflow-x-auto': {},
			'grid-auto-columns': '1fr',
			'counter-reset': 'step',
			'.step': {
				'@apply grid grid-cols-1 grid-rows-2 place-items-center text-center': {},
				'grid-template-rows': '40px 1fr',
				'grid-template-columns': 'auto',
				'min-width': '4rem',
				'&:before': {
					'@apply h-2 w-full top-0 col-start-1 row-start-1 transform translate-x-0 translate-y-0 rtl:translate-x-0': {},
					'margin-inline-start': '-100%',
				},
			},
		},
		'.vsteps': {
			'@apply inline-grid grid-flow-row overflow-y-auto': {},
			'grid-auto-rows': '1fr',
			'counter-reset': 'step',
			'.step': {
				'@apply grid grid-cols-2 grid-rows-1': {},
				'gap': '0.5rem',
				'grid-template-columns': '40px 1fr',
				'grid-template-rows': 'auto',
				'min-height': '4rem',
				'justify-items': 'start',
				'&:before': {
					'@apply w-2 h-full top-0 col-start-1 row-start-1 transform -translate-x-1/2 -translate-y-1/2 rtl:translate-x-1/2': {},
					'margin-inline-start': '50%',
				},
			},
		},
		'.step': {
			'&:before': {
				'content': '""',
			},
			'&:after': {
				'content': 'counter(step)',
				'counter-increment': 'step',
				'z-index': '1',
				'@apply relative col-start-1 row-start-1 grid h-8 w-8 place-items-center place-self-center rounded-full': {},
			},
			'&:first-child:before': {
				'content': 'none',
			},
			'&[data-content]:after': {
				'content': 'attr(data-content)',
			},
		},
	});
	matchComponents(
		{
			'step': (value, options) => ({
				'&:before': {
					backgroundColor: typeof value === 'function' ? value(options) : value,
				},
				'&:after': {
					backgroundColor: typeof value === 'function' ? value(options) : value,
				},
			}),
		},
		{ values: flattenColorPalette(theme('colors')), type: ['color'] },
	);
};
