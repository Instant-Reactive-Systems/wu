/*
Used to overlay a new plane over an existing one.

# Usage

```html
<div class="overlay-container">
  <div class="overlay overlay-container">
    <div class="overlay">
      <p>{some_content}</p>
    </div>
  </div>
  <div class="overlay overlay-container">
    <div class="overlay">
      <p>{some_content}</p>
    </div>
  </div>
</div>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.overlay-container': {
			'display': 'grid',
			'grid-template-columns': '1fr',
			'pointer-events': 'none !important',
			'& > *': {
				'pointer-events': 'auto',
			},
		},
		'.overlay': {
			'grid-row-start': '1',
			'grid-column-start': '1',
			'pointer-events': 'none !important',
			'& > *': {
				'pointer-events': 'auto',
			},
		},
		'.overlay-viewport-container': {
			'display': 'grid',
			'grid-template-columns': '1fr',
			'pointer-events': 'none !important',
			'position': 'fixed',
			'top': '0',
			'bottom': '0',
			'right': '0',
			'left': '0',
			'& > *': {
				'pointer-events': 'auto',
			},
		},
	});
};
