/*
Used to place an element on the corner of another element.

# Usage

```html
<div class="indicator">
  <span class="w-4 h-4 indicator-item indicator-tr"/>
  <p>{content}</p>
</div>
```
*/
export default ({ addUtilities }) => {
	addUtilities({
		'.indicator': {
			'@apply overlay-container': {},
			'& :where(.indicator-item)': {
				'@apply grid overlay': {},
			},
			'& :where(.indicator-item.indicator-tl)': {
				'@apply justify-self-start self-start -translate-x-1/2 -translate-y-1/2': {},
			},
			'& :where(.indicator-item.indicator-tr)': {
				'@apply justify-self-end self-start translate-x-1/2 -translate-y-1/2': {},
			},
			'& :where(.indicator-item.indicator-bl)': {
				'@apply justify-self-start self-end -translate-x-1/2 translate-y-1/2': {},
			},
			'& :where(.indicator-item.indicator-br)': {
				'@apply justify-self-end self-end translate-x-1/2 translate-y-1/2': {},
			},
		},
	});
};
