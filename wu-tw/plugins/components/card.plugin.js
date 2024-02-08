/*
Used to group and display content in a way that is easily readable.

# Usage

```html
<div class="card w-96 shadow-xl">
  <figure><img .../></figure>
  <div class="card-body">
    <h2 class="card-title">{title}</h2>
    <p>{desc}</p>
    <div class="card-actions justify-end">
      <button>{text}</button>
    </div>
  </div>
</div>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.card': {
			'@apply relative flex flex-col gap-2': {},
			'@apply bg-surface-100 dark:bg-surface-800': {},
			'@apply border border-surface-200 dark:border-surface-700': {},
			'@apply rounded-lg': {},
			'&:focus': {
				'@apply outline-none': {},
			},
			'&-body': {
				'@apply flex flex-auto flex-col gap-2 p-4 pt-0': {},
				':where(p)': {
					'@apply flex-grow': {},
				},
			},
			'&-title': {
				'@apply hcenter gap-2 text-xl font-semibold': {},
			},
			'&-actions': {
				'@apply flex flex-wrap items-start gap-2': {},
			},
			'& figure': {
				'@apply center': {},
			},
			'&.image-full': {
				'@apply grid': {},
				'&:before': {
					'@apply relative content-[""]': {},
					'@apply z-1 bg-surface-100 dark:bg-surface-800 opacity-75 rounded-lg': {},
				},
				'&:before, & > *': {
					'@apply col-start-1 row-start-1': {},
				},
				'& > figure img': {
					'@apply h-full object-cover': {},
				},
				':where(figure)': {
					'@apply overflow-hidden': {},
					'border-radius': 'inherit',
				},
			},
			'&.image-full > &-body': {
				'@apply relative z-2 center': {},
			},

			':where(figure:first-child)': {
				'@apply overflow-hidden': {},
				'border-start-start-radius': 'inherit',
				'border-start-end-radius': 'inherit',
				'border-end-start-radius': 'unset',
				'border-end-end-radius': 'unset',
			},
			':where(figure:last-child)': {
				'@apply overflow-hidden': {},
				'border-start-start-radius': 'unset',
				'border-start-end-radius': 'unset',
				'border-end-start-radius': 'inherit',
				'border-end-end-radius': 'inherit',
			},
		},
	});
};
