/*
Used to style a scrollbar to be thin.

# Usage

```html
<div class="thin-scrollbar"/>
```
*/
export default ({ addUtilities }) => {
	addUtilities({
		'.thin-scrollbar': {
			'scrollbar-width': 'thin',
			'&::-webkit-scrollbar': {
				'height': '0.5rem',
				'width': '0.5rem',
			},
		},
	});
};
