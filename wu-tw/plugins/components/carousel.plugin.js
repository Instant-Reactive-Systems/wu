/*
Shows images or content in a scrollable area.

# Usage

## Horizontal carousel
```html
<div class="carousel rounded-lg">
  <div class="carousel-item">
    <img .../>
  </div>
  <div class="carousel-item">
    <img .../>
  </div>
  <div class="carousel-item">
    <img .../>
  </div>
</div>
```

## Vertical carousel
```html
<div class="vcarousel rounded-lg">
  <div class="carousel-item">
    <img .../>
  </div>
  <div class="carousel-item">
    <img .../>
  </div>
  <div class="carousel-item">
    <img .../>
  </div>
</div>
```

## Carousel with snap-to-center
```html
<div class="carousel carousel-center rounded-lg">
  <div class="carousel-item">
    <img .../>
  </div>
  <div class="carousel-item">
    <img .../>
  </div>
  <div class="carousel-item">
    <img .../>
  </div>
</div>
```

## Carousel with snap-to-end
```html
<div class="carousel carousel-end rounded-lg">
  <div class="carousel-item">
    <img .../>
  </div>
  <div class="carousel-item">
    <img .../>
  </div>
  <div class="carousel-item">
    <img .../>
  </div>
</div>
```
*/
export default ({ addComponents }) => {
	addComponents({
		'.carousel': {
			'@apply inline-flex overflow-x-scroll': {},
			'@apply snap-x snap-mandatory scroll-smooth': {},
			'@apply hide-scrollbar': {},
		},
		'.vcarousel': {
			'@apply inline-flex flex-col overflow-y-scroll': {},
			'@apply snap-y snap-mandatory scroll-smooth': {},
			'@apply hide-scrollbar': {},
		},
		'.carousel-item': {
			'@apply box-content flex flex-none snap-start': {},
		},
		'.carousel-center': {
			'& .carousel-item': {
				'@apply snap-center': {},
			},
		},
		'.carousel-end': {
			'& .carousel-item': {
				'@apply snap-end': {},
			},
		},
	});
};
