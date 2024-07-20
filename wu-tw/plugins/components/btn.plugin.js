/*
Styling for elements that provide the user to take an action.

# Usage

```html
<button class="btn">{text}</button>
<button class="btn-wide">{text}</button>
<button class="btn-circle">{text}</button>
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
			'@apply w-full': {},
			'@apply px-5 py-2': {},
			'@apply whitespace-nowrap': {},
			'@apply inline-flex justify-center items-center': {},
			'@apply hover:brightness-[1.15]': {},
			'@apply transition-all': {},
			'&:disabled': {
				'@apply opacity-50 cursor-not-allowed hover:brightness-100': {},
			},
		},
		'.btn-circle': {
			'@apply rounded-full': {},
			'@apply whitespace-nowrap': {},
			'@apply inline-flex justify-center items-center': {},
			'@apply hover:brightness-[1.15]': {},
			'@apply transition-all': {},
			'&:disabled': {
				'@apply opacity-50 cursor-not-allowed hover:brightness-100': {},
			},
		},
		'.btn-square': {
			'@apply whitespace-nowrap': {},
			'@apply inline-flex justify-center items-center': {},
			'@apply hover:brightness-[1.15]': {},
			'@apply transition-all': {},
			'&:disabled': {
				'@apply opacity-50 cursor-not-allowed hover:brightness-100': {},
			},
		},
		'.btn-primary': {
			'@apply bg-primary-500 border-primary-600': {},
		},
		'.btn-secondary': {
			'@apply ghost-primary-500': {},
		},
		'.btn-tertiary': {
			// nothing currently, make it a blank slate
		},
	});
};
