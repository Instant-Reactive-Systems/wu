/*
Enables variant grouping.

# Usage

```html
<div class="[&_*]:x-[btn btn-primary shadow-md]"/>
```
*/
export default ({ matchUtilities }) => {
	matchUtilities({
		'x': (value) => ({
			[`@apply ${value.replaceAll(',', ' ')}`]: {},
		}),
	})
};
