/*
Styling for surfaces.

# Usage

```html
<div class="surface-1">{...}</div>
```
*/
export default ({ addUtilities }) => {
	addUtilities({
		'.surface-bg-1': {
			'@apply bg-light-1 dark:bg-dark-1': {},
		},
		'.surface-bg-2': {
			'@apply bg-light-2 dark:bg-dark-2': {},
		},
		'.surface-bg-3': {
			'@apply bg-light-3 dark:bg-dark-3': {},
		},
		'.surface-border-1': {
			'@apply border-light-1 dark:border-dark-1 divide-light-1 dark:divide-dark-1': {},
		},
		'.surface-border-2': {
			'@apply border-light-2 dark:border-dark-2 divide-light-2 dark:divide-dark-2': {},
		},
		'.surface-border-3': {
			'@apply border-light-3 dark:border-dark-3 divide-light-3 dark:divide-dark-3': {},
		},
		'.surface-1': {
			'@apply bg-light-1 dark:bg-dark-1 border-light-2 dark:border-dark-2 divide-light-2 dark:divide-dark-2': {},
		},
		'.surface-2': {
			'@apply bg-light-2 dark:bg-dark-2 border-light-3 dark:border-dark-3 divide-light-3 dark:divide-dark-3': {},
		},
		'.surface-3': {
			'@apply bg-light-3 dark:bg-dark-3 border-light-3 dark:border-dark-3 divide-light-3 dark:divide-dark-3': {},
		},
	});
};
