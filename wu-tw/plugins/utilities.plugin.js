import centerPlugin from './utilities/center.plugin.js';
import backgroundsPlugin from './utilities/backgrounds.plugin.js';
import dividerPlugin from './utilities/divider.plugin.js';
import ghostPlugin from './utilities/ghost.plugin.js';
import hidescrollbarPlugin from './utilities/hide-scrollbar.plugin.js';
import indicatorPlugin from './utilities/indicator.plugin.js';
import linkPlugin from './utilities/link.plugin.js';
import maskPlugin from './utilities/mask.plugin.js';
import stackPlugin from './utilities/stack.plugin.js';
import swapPlugin from './utilities/swap.plugin.js';
import textoutlinePlugin from './utilities/text-outline.plugin.js';
import thinscrollbarPlugin from './utilities/thin-scrollbar.plugin.js';
import scrollbarcolorPlugin from './utilities/scroll-color.plugin.js';
import xPlugin from './utilities/x.plugin.js';
import verticalPlugin from './utilities/vertical.plugin.js';
import horizontalPlugin from './utilities/horizontal.plugin.js';

export default (params) => {
	centerPlugin(params);
	backgroundsPlugin(params);
	dividerPlugin(params);
	ghostPlugin(params);
	hidescrollbarPlugin(params);
	indicatorPlugin(params);
	linkPlugin(params);
	maskPlugin(params);
	stackPlugin(params);
	swapPlugin(params);
	textoutlinePlugin(params);
	thinscrollbarPlugin(params);
	scrollbarcolorPlugin(params);
	xPlugin(params);
	verticalPlugin(params);
	horizontalPlugin(params);
};
