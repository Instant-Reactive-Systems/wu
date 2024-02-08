/*
Used to show a list of links in a tabbed format.

# Usage

```html
<div role="tablist" class="tabs">
  <a role="tab" class="tab">Tab 1</a>
  <a role="tab" class="tab tab-active">Tab 2</a>
  <a role="tab" class="tab">Tab 3</a>
</div>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.tabs': {
			'@apply flex flex-wrap items-end': {},
			'& > .tab': {
				'@apply border-surface-400 dark:border-surface-500': {},
			},
		},
		'.tab': {
			'@apply text-surface-content text-opacity-50 px-4 py-2 cursor-pointer': {},
			'&.tab-active:not(.tab-disabled, [disabled])': {
				'@apply border-opacity-100 text-opacity-100': {},
			},
			'&:not(.tab-active, .tab-disabled, [disabled])': {
				'@apply [@media(hover:hover)]:hover:text-opacity-75': {},
			},
			'&:focus': {
				'@apply outline-none': {},
			},
			'&:focus-visible': {
				'@apply outline outline-2 outline-current outline-offset-[-3px]': {},
			},
			'&[disabled]': {
				'@apply text-opacity-25 cursor-not-allowed': {},
			},
			'@media (hover: hover)': {
				'&[disabled], &[disabled]:hover': {
					'@apply text-opacity-25 cursor-not-allowed': {},
				},
			},
		},
		'.tab-disabled': {
			'@apply text-opacity-25 cursor-not-allowed': {},
		},
		'.tabs-bordered > .tab': {
			'@apply border-b-2 border-opacity-20': {},
		},
		'.tabs-lifted > .tab': {
			'@apply border-b-2': {},
			'&.tab-active:not(.tab-disabled, [disabled])': {
				'@apply border-2 border-b-0 rounded-t-lg': {},
			},
		},
		'.tabs-boxed': {
			'@apply bg-surface-400/20 dark:bg-surface-500/20': {},
			'&.tab-active:not(.tab-disabled, [disabled])': {
				'@apply bg-primary-500': {},
			},
		},
	});
};
