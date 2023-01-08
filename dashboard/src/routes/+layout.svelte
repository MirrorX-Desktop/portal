<script>
	import { page } from '$app/stores';
	import '../app.css';
	import brand from '$lib/images/brand.png';
</script>

<!-- svelte-ignore a11y-no-noninteractive-tabindex -->
<!-- svelte-ignore a11y-label-has-associated-control -->
<!-- svelte-ignore a11y-missing-attribute -->
<div class="w-full h-full flex flex-col">
	<div class="navbar bg-base-300 pl-0">
		<div class="flex-none w-56 flex flex-row text-center gap-3">
			<img width="48" height="48" src={brand} class="pl-2" />
			<span class="text-3xl font-bold">Dashboard</span>
		</div>
		<div class="flex-1" />
		<!-- <div class="dropdown dropdown-end">
			<label tabindex="0" class="btn btn-ghost btn-circle avatar">
				<div class="w-10 rounded-full">
					<img src="https://placeimg.com/80/80/people" />
				</div>
			</label>

			<ul
				tabindex="0"
				class="menu menu-compact dropdown-content mt-3 p-2 shadow bg-base-100 rounded-box w-52"
			>
				<li>
					<a class="justify-between">
						Profile
						<span class="badge">New</span>
					</a>
				</li>

				<li><a>Settings</a></li>

				<li><a>Logout</a></li>
			</ul>
		</div> -->
	</div>

	<div class="flex-1 w-full h-full flex flex-row">
		<nav class="w-56 h-full flex-0">
			<ul class="menu bg-base-200 w-full p-2 h-full">
				<li class="mb-2">
					<a href="/stat" class={$page.url.pathname == '/stat' ? 'active' : undefined}>Stat</a>
				</li>
				<li>
					<a href="/about" class={$page.url.pathname == '/about' ? 'active' : undefined}>About</a>
				</li>
			</ul>
		</nav>

		<div class="flex-1 h-full p-4 overflow-auto"><slot /></div>
	</div>
</div>
