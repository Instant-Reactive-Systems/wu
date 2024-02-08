import { withAlphaVariable, withAlphaValue } from 'tailwindcss/lib/util/withAlphaVariable';
import flattenColorPalette from 'tailwindcss/lib/util/flattenColorPalette';

/*
Styling for components with a strong border and weak body.

# Usage

```html
<span class="w-8 h-8 bg-ghost-blue-500"/>
```
*/
export default ({ matchUtilities, theme }) => {
	matchUtilities(
		{
			ghost: (value) => {
				return ({
					borderColor: withAlphaValue(value, 1),
					backgroundColor: withAlphaValue(value, 0.20),
				})
			},
		},
		{ values: flattenColorPalette(theme('colors')), type: ['color', 'any'] }
	);
};
