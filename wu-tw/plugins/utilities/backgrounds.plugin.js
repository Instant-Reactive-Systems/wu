/*
Provides with a set of custom backgrounds.

# Usage

```html
<div class="w-[200px] h-[100px] bg-blue-700">
  <div class="w-full h-full bg-stripes-blue-500"/>
</div>
```
*/
export default ({ addUtilities, theme }) => {
	const backgroundSize = '7.07px 7.07px';
	const backgroundImage = (color) => `linear-gradient(135deg, ${color} 10%, transparent 10%, transparent 50%, ${color} 50%, ${color} 60%, transparent 60%, transparent 100%)`;
	const colors = Object.entries(theme('backgroundColor')).filter(
		([, value]) => typeof value === 'object' && value[400] && value[500]
	);

	addUtilities(
		Object.fromEntries(
			colors.map(([name, colors]) => {
				let backgroundColor = colors[400] + '1a'; // 10% opacity
				let stripeColor = colors[500] + '80'; // 50% opacity

				return [
					`.bg-stripes-${name}`,
					{
						backgroundColor,
						backgroundImage: backgroundImage(stripeColor),
						backgroundSize,
					},
				];
			})
		)
	);

	addUtilities({
		'.bg-stripes-white': {
			backgroundImage: backgroundImage('rgba(255 255 255 / 0.75)'),
			backgroundSize,
		},
	});
};
