/*
Styling of a list that displays the current navigation hierarchy.

# Usage

```html
<div class="breadcrumbs">
  <ul>
    <li><a>{home}</a></li>
    <li><a>{docs}</a></li>
    <li>{std::variant}</li>
  </ul>
</div>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.breadcrumbs': {
			'@apply inline-vcenter gap-2 overflow-x-auto py-2': {},
			'& > *': {
				'@apply inline-vcenter': {},
				'& + *:before': {
					'content': '""',
					'border-top': '1px solid',
					'border-right': '1px solid',
					'@apply bg-transparent mr-3 inline-flex h-1.5 w-1.5 rotate-45 transform opacity-40': {},
				},
			},
		},
	});
};
