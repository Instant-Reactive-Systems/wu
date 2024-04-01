/*
Utilities for quickly centering the contents of a container.

# Usage

## Center both vertically and horizontally
```html
<div class="w-16 h-16 center">
  <div class="w-8 h-8"/>
</div>
```

## Center vertically
```html
<div class="w-16 h-16 vcenter">
  <div class="w-8 h-8"/>
</div>
```

## Center horizontally 
```html
<div class="w-16 h-16 hcenter">
  <div class="w-8 h-8"/>
</div>
```
*/
export default ({ addUtilities }) => {
	addUtilities({
		'.center': {
			'@apply justify-center items-center': {},
		},
		'.vcenter': {
			'@apply items-center': {},
		},
		'.hcenter': {
			'@apply justify-center': {},
		},
		'.center-none': {
			'@apply justify-normal items-start': {},
		},
		'.vcenter-none': {
			'@apply items-start': {},
		},
		'.hcenter-none': {
			'@apply justify-normal': {},
		},
	});
};
