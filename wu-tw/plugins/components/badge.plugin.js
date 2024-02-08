/*
Styling for small non-interactive elements that indicate some information.

# Usage

```html
<span class="badge">{tag}</span>
<span class="badge-icon">{icon}</span>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.badge': {
			'@apply inline-flex justify-center items-center whitespace-nowrap': {},
			'@apply font-semibold text-xs': {},
			'@apply px-2 py-1': {},
			'@apply rounded-xl': {},
		},
		'.badge-icon': {
			'@apply w-5 h-5 flex justify-center items-center rounded-full': {},
			'@apply font-semibold text-xs': {},
		},
	});
};
