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
			'counter-reset': 'step',
			'grid-auto-columns': '1fr',
		},
		'.vsteps': {
			'counter-reset': 'step',
			'.step': {
				'gap': '.5rem',
				'grid-template-columns': '40px 1fr',
				'grid-template-rows': 'auto',
				'min-height': '4rem',
				'justify-items': 'start',
				'&:before': {
					'@apply w-2 h-full top-0 transform -translate-y-1/2 -translate-x-1/2': {},
					'margin-left': '50%',
				},
				'&:after': {
					'content': 'counter(step)',
					'counter-increment': 'step',
				},
			},
		},
		'.step': {
			'@apply grid grid-cols-1 grid-rows-2 place-items-center text-center': {},
			'@apply min-w-[4rem]': {},
			'grid-template-rows': '40px 1fr',
			'grid-template-columns': 'auto',
			'&:before': {
				'@apply top-0 col-start-1 row-start-1 h-2 w-full transform': {},
				'content': '""',
				'margin-left': '-100%',
			},
			'&:after': {
				'content': 'counter(step)',
				'counter-increment': 'step',
				'@apply relative col-start-1 row-start-1 grid h-8 w-8 place-items-center place-self-center rounded-full z-1': {},
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
			'step': (value) => ({
				'&:before': {
					backgroundColor: value,
				},
				'&:after': {
					backgroundColor: value,
				},
			}),
		},
		{ values: flattenColorPalette(theme('colors')) },
	);
};
