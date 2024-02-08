/*
Used to display keyboard shortcuts.

# Usage

```html
<kbd class="kbd">Ctrl + L</kbd>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.kbd': {
			'@apply inline-flex justify-center items-center': {},
			'@apply border border-b-4 min-h-[2.2em] min-w-[2.2em] px-2 rounded-lg': {},
			'@apply bg-surface-100 dark:bg-surface-800 border-surface-200 dark:border-surface-900': {},
		},
	});
};
