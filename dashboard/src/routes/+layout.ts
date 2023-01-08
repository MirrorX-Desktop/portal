export const prerender = true;
export const ssr = false;

export const _switchTheme = () => {
	const currentTheme = (document.getElementsByTagName('html').item(0)?.getAttribute('data-theme') ??
		'light') as 'light' | 'dark';

	document
		.getElementsByTagName('html')
		.item(0)
		?.setAttribute('data-theme', currentTheme == 'light' ? 'dark' : 'light');
};
