/*
Visually puts elements on top of each other.

# Usage

```html
<div class="stack">
  <div/> 
  <div/> 
  <div/> 
</div>
```
*/
export default ({ addUtilities }) => {
	addUtilities({
		'.stack': {
			'@apply inline-grid place-items-center items-end': {},
			'& > *': {
				'@apply col-start-1 row-start-1 w-full z-1 opacity-60': {},
				'transform': 'translateY(10%) scale(0.9)',
			},
			'& > *:nth-child(2)': {
				'@apply z-2 opacity-80': {},
				'transform': 'translateY(5%) scale(0.95)',
			},
			'& > *:nth-child(1)': {
				'@apply z-3 opacity-100': {},
				'transform': 'translateY(0) scale(1)',
			},
		},
	});
};
