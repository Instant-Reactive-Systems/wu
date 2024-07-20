/*
Used to show a block of code in a box that looks like a code editor.

# Usage

## Code with no line numbers
```html
<div class="code">
  <pre>{code}</pre>
  <pre>{code}</pre>
  <pre data-prefix=">">{code}</pre>
</div>
```

## Code with line numbers
```html
<div class="code code-numbered">
  <pre>{code}</pre>
  <pre>{code}</pre>
  <pre>{code}</pre>
</div>
```
*/
export default ({ addComponents, matchComponents }) => {
	addComponents({
		'.drawer-hook': {
			'@apply overlay-container transition-transform motion-safe:transition-none bg-black opacity-25 backdrop-blur-sm': {},
			'@starting-style': {
				'&:popover-open': {
					'@apply opacity-0': {},
				},
			},
		},
		'.drawer-left': {
			'@apply overlay justify-self-start h-full max-h-svh w-[300px] desktop:w-[400px] transition-transform motion-safe:transition-none -translate-x-[300px] desktop:-translate-x-[400px]': {},
			'@starting-style': {
				'&:popover-open': {
					'@apply translate-x-0': {},
				},
			},
		},
		'.drawer-right': {
			'@apply overlay justify-self-end h-full max-h-svh w-[300px] desktop:w-[400px] translate-x-[300px] transition-transform motion-safe:transition-none desktop:translate-x-[400px]': {},
			'@starting-style': {
				'&:popover-open': {
					'@apply translate-x-0': {},
				},
			},
		},
		'.drawer-top': {
			'@apply overlay self-start w-full max-w-svw h-[200px] desktop:h-[300px] transition-transform motion-safe:transition-none -translate-y-[200px] desktop:-translate-y-[300px]': {},
			'@starting-style': {
				'&:popover-open': {
					'@apply translate-y-0': {},
				},
			},
		},
		'.drawer-bottom': {
			'@apply overlay self-end w-full max-w-svw h-[200px] desktop:h-[300px] transition-transform motion-safe:transition-none translate-y-[200px] desktop:translate-y-[300px]': {},
			'@starting-style': {
				'&:popover-open': {
					'@apply translate-y-0': {},
				},
			},
		},
	});

	matchComponents(
		{
			'code-numbered': (value) => ({
				'--code-start': value,
				'counter-reset': 'line calc(var(--code-start) - 1)',

				'& pre': {
					'&:before': {
						'content': 'counter(line)',
						'counter-increment': 'line',
						'@apply w-8 opacity-50 mr-4': {},
					},
				},
			}),
		},
	);
};
