/*
Styling of UI elements that inform users about important events.

# Usage

```html
<div class="alert">
  <Icon/>
  <p>{message}</p>
</div>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.alert': {
			'@apply flex flex-col tablet:flex-row tablet:items-center p-4 gap-4': {},
			'@apply rounded-md': {},
		},
	});
};
