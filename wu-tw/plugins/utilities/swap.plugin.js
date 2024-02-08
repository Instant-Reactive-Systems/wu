/*
Allows toggling the visibility of two elements using a checkbox or a class name.

# Usage

```html
<label class="swap <anim-of-your-choosing>">
  <input type="checkbox" />
  <div class="swap-on">ON</div>
  <div class="swap-off">OFF</div>
</label>
```
*/
export default ({ addUtilities }) => {
	addUtilities({
		'.swap': {
			'@apply relative inline-grid select-none place-content-center cursor-pointer': {},
			'& > *': {
				'@apply col-start-1 row-start-1': {},
				'@apply duration-300 ease-in-out': {},
				'transition-property': 'transform, opacity',
			},
			'& input': {
				'@apply appearance-none hidden': {},
			},
			'& .swap-on, & .swap-indeterminate, & input:indeterminate ~ .swap-on': {
				'@apply opacity-0': {},
			},
			'& input:checked ~ .swap-off, & .swap-active .swap-off, & input:indeterminate ~ .swap-off': {
				'@apply opacity-0': {},
			},
			'& input:checked ~ .swap-on, & .swap-active .swap-on, & input:indeterminate ~ .swap-indeterminate': {
				'@apply opacity-100': {},
			},
		},
		'.swap-rotate': {
			'& .swap-on, & .swap-indeterminate, & input:indeterminate ~ .swap-on': {
				'@apply rotate-45': {},
			},
			'& input:checked ~ .swap-off, & .swap-active .swap-off, & input:indeterminate ~ .swap-off': {
				'@apply -rotate-45': {},
			},
			'& input:checked ~ .swap-on, & .swap-active .swap-on, & input:indeterminate ~ .swap-indeterminate': {
				'@apply rotate-0': {},
			},
		},
		'.swap-flip': {
			'transform-style': 'preserve-3d',
			'perspective': '16em',
			'& .swap-on, & .swap-indeterminate, & input:indeterminate ~ .swap-on': {
				'transform': 'rotateY(180deg)',
				'backface-visibility': 'hidden',
				'@apply opacity-100': {},
			},
			'& input:checked ~ .swap-off, & .swap-active .swap-off, & input:indeterminate ~ .swap-off': {
				'transform': 'rotateY(-180deg)',
				'backface-visibility': 'hidden',
				'@apply opacity-100': {},
			},
			'& input:checked ~ .swap-on, & .swap-active .swap-on, & input:indeterminate ~ .swap-indeterminate': {
				'transform': 'rotateY(0deg)',
			},
		},
	});
};
