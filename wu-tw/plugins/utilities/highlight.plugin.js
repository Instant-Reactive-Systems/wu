import { withAlphaVariable, withAlphaValue } from 'tailwindcss/lib/util/withAlphaVariable';
import flattenColorPalette from 'tailwindcss/lib/util/flattenColorPalette';

/*
A button style that highlights the surrounding area on hover.

# Usage

```html
<button class="highlight"/>
```
*/
export default ({ addUtilities }) => {
	addUtilities({
		'.highlight': {
			'@apply hover:bg-light-content/5 focus-within:bg-light-content/5 hover:dark:bg-dark-content/5 focus-within:dark:bg-dark-content/5 transition-colors motion-safe:transition-none': {},
		},
	});
};
