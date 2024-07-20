/*
Styling for elements that provide the user to take an action.

# Usage

```html
<div class="btns">
	<button class="btn">{text}</button>
	<button class="btn">{text}</button>
	<button class="btn">{text}</button>
</btns>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.hbtns': {
			'@apply horizontal': {},
			'& > input[type="radio"]': {
				'@apply absolute invisible': {}, // make it so the radio itself has no affect on DOM
			},
			'& > input[type="checkbox"]': {
				'@apply absolute invisible': {}, // make it so the radio itself has no affect on DOM
			},
			'& > label': {
				'content': '',
				'@apply flex center cursor-pointer': {},
			},
		},
		'.vbtns': {
			'@apply vertical': {},
			'& > input[type="radio"]': {
				'@apply absolute invisible': {}, // make it so the radio itself has no affect on DOM
			},
			'& > input[type="checkbox"]': {
				'@apply absolute invisible': {}, // make it so the radio itself has no affect on DOM
			},
			'& > label': {
				'content': '',
				'@apply flex center cursor-pointer': {},
			},
		},
	});
};
