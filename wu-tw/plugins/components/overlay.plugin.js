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
			// '@apply grid grid-cols-1 grid-rows-1': {},
			'display': 'grid',
			'grid-template-columns': '1fr',
			'pointer-events': 'none',
			'& > *': {
				'pointer-events': 'auto',
			},
		},
		'.overlay': {
			// '@apply col-span-1 row-span-1': {},
			'grid-row-start': '1',
			'grid-column-start': '1',
			'pointer-events': 'none',
			'& > *': {
				'pointer-events': 'auto',
			},
		},
	});
};
