/*
Used to remove a scrollbar from a container.

# Usage

```html
<div class="hide-scrollbar"/>
```
*/
export default ({ addUtilities }) => {
	addUtilities({
		'.hide-scrollbar': {
			'-ms-overflow-style': 'none',
			'scrollbar-width': 'none',
			'&::-webkit-scrollbar': {
				'@apply hidden': {},
			},
		},
	});
};
