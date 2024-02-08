/*
Styling for elements that provide the user to take an action.

# Usage

```html
<button class="btn">{text}</button>
<button class="btn-wide">{text}</button>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.btn': {
			'@apply px-5 py-2': {},
			'@apply whitespace-nowrap': {},
			'@apply inline-flex justify-center items-center': {},
			'@apply hover:brightness-[1.15]': {},
			'@apply transition-all': {},
			'&:disabled': {
				'@apply opacity-50 cursor-not-allowed hover:brightness-100': {},
			},
		},
		'.btn-wide': {
			'@apply center w-full': {},
		},
	});
};
