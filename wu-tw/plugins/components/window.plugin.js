/*
Shows a box that looks like an operating system window.

# Usage

```html
<div class="window border border-base-300">
  <div class="flex justify-center px-4 py-16 border-t border-base-300">Hello!</div>
</div>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.window': {
			'@apply relative overflow-hidden overflow-x-auto': {},
			'@apply pt-5 rounded-md': {},
			'&:before': {
				'@apply content-[""] mb-4 block h-3 w-3 rounded-full opacity-100': {},
				'box-shadow': '1.4em 0 rgba(225, 0, 11, 0.8), 2.8em 0 rgba(222, 172, 10, 0.8), 4.2em 0 rgba(71, 172, 7, 0.8)',
			},
		},
		'.window-grayscale': {
			'@apply relative overflow-hidden overflow-x-auto': {},
			'@apply pt-5 rounded-md': {},
			'&:before': {
				'@apply content-[""] mb-4 block h-3 w-3 rounded-full opacity-20': {},
				'box-shadow': '1.4em 0, 2.8em 0, 4.2em 0',
			},
		},
	});
};
