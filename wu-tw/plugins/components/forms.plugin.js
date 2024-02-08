/*
Styling of form elements.

# Usage

```html
<input type="text" class=move || ok.then_some("input-success").unwrap_or("input-error")/>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.input-success': {
			'@apply bg-success-200 border-success-500 text-success-700 focus-within:ring-success-500 focus:ring': {},
			'&::placeholder': {
				'@apply text-success-700': {},
			}
		},
		'.input-warning': {
			'@apply bg-warning-200 border-warning-500 text-warning-700 focus-within:ring-warning-700 focus:ring': {},
			'&::placeholder': {
				'@apply text-warning-700': {},
			}
		},
		'.input-error': {
			'@apply bg-error-200 border-error-500 text-error-700 focus-within:ring-error-700 focus:ring': {},
			'&::placeholder': {
				'@apply text-error-700': {},
			}
		},
	});
};
