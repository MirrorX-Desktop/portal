<script lang="ts">
	import { fetchStatDetails } from '$lib/api';
	import type { StatDetails } from '$lib/models';
	import { faSpinner } from '@fortawesome/free-solid-svg-icons';
	import { onMount } from 'svelte';
	import DetailsPanel from './details_panel.svelte';
	import Fa from 'svelte-fa';
	import { formatDeviceID } from '$lib/utils';
	import moment from 'moment';

	let statDetails: StatDetails | null = null;

	onMount(async () => {
		statDetails = await fetchStatDetails();
	});
</script>

<svelte:head>
	<title>Stat</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="w-full h-full">
	<DetailsPanel
		domain={statDetails?.domain ?? ''}
		allocated={statDetails?.allocated ?? 0}
		bytesTransferred={statDetails?.bytes_transferred ?? 0}
	/>
	<div class="overflow-x-auto">
		{#if statDetails}
			<table class="table w-full">
				<thead>
					<tr>
						<th />
						<th>Active EndPoint</th>
						<th>Active Addr</th>
						<th>Passive EndPoint</th>
						<th>Passive Addr</th>
						<th>Establish Time (UTC)</th>
					</tr>
				</thead>
				<tbody>
					{#each statDetails.client_snapshot as client, index}
						<tr>
							<th>{index + 1}</th>
							<td>{formatDeviceID(client.active_device_id)}</td>
							<td>{client.active_addr}</td>
							<td>{formatDeviceID(client.passive_device_id)}</td>
							<td>{client.passive_addr}</td>
							<td>{moment.unix(client.timestamp).utc().format('YYYY-MM-DD hh:mm:ss')}</td>
						</tr>
					{/each}
				</tbody>
			</table>
		{:else}
			<Fa icon={faSpinner} spin />
		{/if}
	</div>
</div>
