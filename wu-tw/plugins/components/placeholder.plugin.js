/*
Used as placeholders while actual content is loading in the background.

# Usage

```html
<section class="card w-full">
	<div class="p-4 space-y-4">
		<div class="placeholder-circle" />
		<div class="grid grid-cols-3 gap-8">
			<div class="placeholder" />
			<div class="placeholder" />
			<div class="placeholder" />
		</div>
		<div class="grid grid-cols-4 gap-4">
			<div class="placeholder" />
			<div class="placeholder" />
			<div class="placeholder" />
			<div class="placeholder" />
		</div>
	</div>
</section>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.placeholder': {
			'@apply bg-surface-300 dark:bg-surface-600 h-5 rounded-full': {},
		},
		'.placeholder-circle': {
			'@apply bg-surface-300 dark:bg-surface-600 aspect-square rounded-full': {},
		},
	});
};
