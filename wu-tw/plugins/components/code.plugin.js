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
		'.code': {
			'@apply relative overflow-hidden overflow-x-auto': {},
			'@apply w-full py-5 rounded-md': {},
			'pre': {
				'@apply pr-5': {},
				'&:before': {
					'@apply mr-[2ch] inline-block text-right': {},
				},
			},
			'pre[data-prefix]': {
				'content': 'attr(data-prefix)',
				'@apply w-8 opacity-50': {},
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
