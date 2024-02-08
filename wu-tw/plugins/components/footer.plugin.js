/*
Contains logo, copyright notice, and links to other pages.

# Usage

## Normal footer
```html
<footer class="footer-table">
  <aside>
    {logo}
    <p>{summary}</p>
  </aside>
  <nav>
    <h6 class="footer-table-title">{title}</h6>
    <ul>
      <li><a href="#">{link}</a></li>
      <li><a href="#">{link}</a></li>
      <li><a href="#">{link}</a></li>
    </ul>
  </nav>
</footer>
```

## Centered footer
```html
<footer class="footer-table footer-table-center">
  <nav>
    <h6 class="footer-table-title">{title}</h6>
    <ul>
      <li><a href="#">{link}</a></li>
      <li><a href="#">{link}</a></li>
      <li><a href="#">{link}</a></li>
    </ul>
  </nav>
</footer>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.footer-table': {
			'@apply grid w-full place-items-start grid-flow-row tablet:grid-flow-col': {},
			'@apply gap-y-10 gap-x-4 text-sm': {},

			'& > *': {
				'@apply grid place-items-start': {},
				'@apply gap-2': {},
			},
		},
		'.footer-table-center': {
			'@apply place-items-center text-center tablet:grid-flow-row-dense': {},
			'& > *': {
				'@apply place-items-center': {},
			},
		},
		'.footer-table-title': {
			'@apply mb-2 font-bold uppercase opacity-50': {},
		},
	});
};
