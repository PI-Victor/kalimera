<script lang="ts">
	import 'uikit/dist/css/uikit.min.css';
	import UIkit from 'uikit';
	import Icons from 'uikit/dist/js/uikit-icons';

	import { page } from '$app/stores';
	import { afterNavigate } from '$app/navigation';

	UIkit.use(Icons);

	let { pathname } = $page.url;

	afterNavigate(() => {
		pathname = $page.url.pathname;
	});
</script>

<div class="uk-height-viewport" uk-grid>
	<aside id="menu" class="uk-position-fixed uk-height-viewport">
		<ul class="uk-nav uk-nav-default uk-nav-parent-icon">
			<li
				class="uk-padding-small"
				class:uk-active={pathname == '/' || pathname.includes('buckets')}
			>
				<a href="/" uk-icon="icon: album" uk-tooltip="Buckets" />
			</li>
			<li class="uk-padding-small" class:uk-active={pathname == '/settings/profiles'}>
				<a href="/settings/profiles" uk-icon="icon: users" uk-tooltip="Profiles" />
			</li>
			<li class="uk-padding-small" class:uk-active={pathname == '/settings/general'}>
				<a href="/settings/general" uk-icon="icon: cog" uk-tooltip="General" />
			</li>
			<li class="uk-padding-small" class:uk-active={pathname == '/settings/appearance'}>
				<a href="/settings/appearance" uk-icon="icon: paint-bucket" uk-tooltip="Appearance" />
			</li>
			<li class="uk-padding-small" class:uk-active={pathname == '/about'}>
				<a href="/about" uk-icon="icon: info" uk-tooltip="About" />
			</li>
		</ul>
	</aside>

	<div id="main" class="uk-width-expand uk-height-viewport">
		<div class="uk-container uk-container-expand uk-height-viewport">
			<slot />
		</div>
	</div>
</div>

<style>
	#menu {
		width: 100px;
	}
	#main {
		margin-left: 120px;
	}
</style>
