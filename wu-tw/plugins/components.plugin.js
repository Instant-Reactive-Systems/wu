import alertPlugin from './components/alert.plugin.js';
import badgePlugin from './components/badge.plugin.js';
import breadcrumbsPlugin from './components/breadcrumbs.plugin.js';
import btnPlugin from './components/btn.plugin.js';
import cardPlugin from './components/card.plugin.js';
import carouselPlugin from './components/carousel.plugin.js';
import chipPlugin from './components/chip.plugin.js';
import codePlugin from './components/code.plugin.js';
import footerPlugin from './components/footer.plugin.js';
import formsPlugin from './components/forms.plugin.js';
import kbdPlugin from './components/kbd.plugin.js';
import loadingPlugin from './components/loading.plugin.js';
import overlayPlugin from './components/overlay.plugin.js';
import placeholderPlugin from './components/placeholder.plugin.js';
import progressPlugin from './components/progress.plugin.js';
import stepsPlugin from './components/steps.plugin.js';
import tabsPlugin from './components/tabs.plugin.js';
import windowPlugin from './components/window.plugin.js';
import avatarPlugin from './components/avatar.plugin.js';
import btnsPlugin from './components/btns.plugin.js';
import iconsPlugin from './components/icons.plugin.js';

export default (params) => {
	alertPlugin(params);
	badgePlugin(params);
	breadcrumbsPlugin(params);
	btnPlugin(params);
	cardPlugin(params);
	carouselPlugin(params);
	chipPlugin(params);
	codePlugin(params);
	footerPlugin(params);
	formsPlugin(params);
	kbdPlugin(params);
	loadingPlugin(params);
	overlayPlugin(params);
	placeholderPlugin(params);
	progressPlugin(params);
	stepsPlugin(params);
	tabsPlugin(params);
	windowPlugin(params);
	avatarPlugin(params);
	btnsPlugin(params);
	iconsPlugin(params);
};
