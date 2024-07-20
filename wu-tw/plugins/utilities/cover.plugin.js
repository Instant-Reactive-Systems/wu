/*
Utilities for covering the full area of the parent.                                                                                                                                                                                                                                                                                                                                             ontents of a container.

# Usage
```html
<div class="w-32 h-32">
  <div class="cover"/>
</div>
```
*/
export default ({ addUtilities }) => {
	addUtilities({
		'.cover': {
			'@apply w-full h-full': {},
		},
	});
};
