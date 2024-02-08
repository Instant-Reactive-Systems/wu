/*
Applies equal height and width to an element.

# Usage

```html
<div class="square-16"/>
```
*/
export default ({ matchUtilities, theme }) => {
	matchUtilities(
		{
			'square': (value) => ({
				width: value,
				height: value,
			}), 
		},
		{ values: theme('width'), type: 'length' },
	);
};
