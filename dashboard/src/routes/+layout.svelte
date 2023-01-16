<script>
	import { page } from '$app/stores';
	import '../app.css';
	import brand from '$lib/images/brand.png';
	import { faLanguage } from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<!-- svelte-ignore a11y-label-has-associated-control -->
<!-- svelte-ignore a11y-missing-attribute -->
<div class="w-full h-full flex flex-col">
	<div class="navbar bg-base-300 py-0 pl-0 pr-2 flex-0">
		<div class="flex-none w-56 flex flex-row text-center gap-3">
			<img width="48" height="48" src={brand} class="pl-2" />
			<div class="flex flex-col items-start w-full">
				<div class="text-3xl font-bold">MirrorX</div>
			</div>
		</div>
		<div class="flex-1" />
		<div class="flex-none">
			<ul class="menu menu-horizontal px-1 gap-2">
				<li tabindex="0">
					<a>
						<Fa icon={faLanguage} />
						<svg
							class="fill-current"
							xmlns="http://www.w3.org/2000/svg"
							width="20"
							height="20"
							viewBox="0 0 24 24"
							><path d="M7.41,8.58L12,13.17L16.59,8.58L18,10L12,16L6,10L7.41,8.58Z" /></svg
						>
					</a>
					<ul class="p-2 bg-base-100 shadow">
						<li><a>English</a></li>
						<li><a>中文</a></li>
					</ul>
				</li>
			</ul>
		</div>
	</div>

	<div class="flex-1 w-full h-full flex flex-row overflow-auto">
		<nav class="w-56 h-full flex-0">
			<ul class="menu bg-base-200 w-full p-2 h-full">
				<li class="mb-2">
					<a href="/overview" class={$page.url.pathname == '/overview' ? 'active' : undefined}
						>Overview</a
					>
				</li>
			</ul>
		</nav>

		<div class="flex-1 h-full p-4 overflow-auto"><slot /></div>
	</div>
</div>
