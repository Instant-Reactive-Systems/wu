/*
Utility for horizontally aligning elements using flex.

Basically just `flex flex-row`.

# Usage

## Horizontally align a list
```html
<ul class="horizontal gap-2">
	<li>...</li>
	<li>...</li>
	<li>...</li>
</ul>
```
*/
export default ({ addUtilities }) => {
	addUtilities({
		'.horizontal': {
			'@apply flex flex-row': {},
		},
	});
};
