/*
Used to place an element on the corner of another element. (Only useful on box elements)

# Usage

```html
<div class="indicator">
  <span class="w-4 h-4 indicator-item"/>
  <p>{content}</p>
</div>
```
*/
export default ({ addUtilities }) => {
	addUtilities({
		'.indicator': {
			'@apply relative': {},
			'& :where(.indicator-item)': {
				'@apply absolute right-0 left-auto top-0 bottom-auto translate-x-1/2 -translate-y-1/2': {},
			},
			'& :where(.indicator-item.indicator-left)': {
				'@apply right-auto left-0 -translate-x-1/2': {},
			},
			'& :where(.indicator-item.indicator-xcenter)': {
				'@apply right-1/2 left-1/2 -translate-x-1/2': {},
			},
			'& :where(.indicator-item.indicator-right)': {
				'@apply right-0 left-auto translate-x-1/2': {},
			},
			'& :where(.indicator-item.indicator-bottom)': {
				'@apply top-auto bottom-0 translate-y-1/2': {},
			},
			'& :where(.indicator-item.indicator-ycenter)': {
				'@apply top-1/2 bottom-1/2 -translate-y-1/2': {},
			},
			'& :where(.indicator-item.indicator-top)': {
				'@apply top-0 bottom-auto -translate-y-1/2': {},
			},
		},
	});
};
