/*
Styling of small interactive elements for actions, selection, or filtering.

# Usage

```html
<span class="chip">{tag}</span>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.chip': {
			'@apply px-3 py-1 whitespace-nowrap cursor-pointer': {},
			'@apply text-xs text-center': {},
			'@apply rounded-md': {},
			'@apply inline-flex justify-center items-center gap-2': {},
			'@apply hover:brightness-[1.15]': {},
			'@apply transition-all': {},
			'&:disabled': {
				'@apply opacity-50 cursor-not-allowed': {},
			},
		},
	});
};
