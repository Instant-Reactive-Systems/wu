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
  <div class="overlay-glued overlay-container">
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
			'pointer-events': 'none',
			'& > *': {
				'pointer-events': 'auto',
			},
		},
		'.overlay': {
			'grid-row-start': '1',
			'grid-column-start': '1',
			'pointer-events': 'none',
			'& > *': {
				'pointer-events': 'auto',
			},
		},
		'.overlay-glued': {
			'@apply fixed top-0 bottom-0 left-0 right-0': {},
			'grid-row-start': '1',
			'grid-column-start': '1',
			'pointer-events': 'none',
			'& > *': {
				'pointer-events': 'auto',
			},
		},
	});
};
