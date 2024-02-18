/*
Avatar component with optional slots on its circumference at 8 points.

# Usage

```html
<img src="my-avatar.png" class="avatar [&>*]:avatar-slot">
	<span class="avatar-slot-1"/>
	<span class="avatar-slot-3"/>
	<span class="avatar-slot-5"/>
	<span class="avatar-slot-7"/>
</img>
```
*/
export default ({ addComponents, matchUtilities, theme }) => {
	addComponents({
		'.avatar': {
			'height': 'var(--wu-avatar-size)',
			'width': 'var(--wu-avatar-size)',
			'@apply rounded-full overlay-container': {},

			'& .avatar-slot-1, & .avatar-slot-2, & .avatar-slot-3, & .avatar-slot-4, & .avatar-slot-5, & .avatar-slot-6, & .avatar-slot-7, & .avatar-slot-8': {
				'@apply overlay w-fit h-fit justify-self-center self-center': {},
				'transform': 'translate(calc(var(--wu-avatar-size) * 0.5 * var(--wu-avatar-cos)), calc(var(--wu-avatar-size) * 0.5 * var(--wu-avatar-sin)))',
			},
		},
		'.avatar-slot-1': {
			'--wu-avatar-cos': '0',
			'--wu-avatar-sin': '-1',
		},
		'.avatar-slot-2': {
			'--wu-avatar-cos': '0.70710678118',
			'--wu-avatar-sin': '-0.70710678118',
		},
		'.avatar-slot-3': {
			'--wu-avatar-cos': '1',
			'--wu-avatar-sin': '0',
		},
		'.avatar-slot-4': {
			'--wu-avatar-cos': '0.70710678118',
			'--wu-avatar-sin': '0.70710678118',
		},
		'.avatar-slot-5': {
			'--wu-avatar-cos': '0',
			'--wu-avatar-sin': '1',
		},
		'.avatar-slot-6': {
			'--wu-avatar-cos': '-0.70710678118',
			'--wu-avatar-sin': '0.70710678118',
		},
		'.avatar-slot-7': {
			'--wu-avatar-cos': '-1',
			'--wu-avatar-sin': '0',
		},
		'.avatar-slot-8': {
			'--wu-avatar-cos': '-0.70710678118',
			'--wu-avatar-sin': '-0.70710678118',
		},
	});

	matchUtilities(
		{
			'avatar-size': (value) => ({
				'--wu-avatar-size': value,
			}),
		},
		{ values: theme('width'), type: 'length' }
	)
};
