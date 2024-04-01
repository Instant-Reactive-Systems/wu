/*
Utility for vertically aligning elements using flex.

Basically just `flex flex-col`.

# Usage

## Vertically align a list
```html
<ul class="vertical gap-2">
	<li>...</li>
	<li>...</li>
	<li>...</li>
</ul>
```
*/
export default ({ addUtilities }) => {
	addUtilities({
		'.vertical': {
			'@apply flex flex-col': {},
		},
	});
};
