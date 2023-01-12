<script lang="ts">
	import { fetchStatDetails } from '$lib/api';
	import type { StatDetails } from '$lib/models';
	import { faSpinner, faExclamationCircle } from '@fortawesome/free-solid-svg-icons';
	import { onMount } from 'svelte';
	import DetailsPanel from './details_panel.svelte';
	import Fa from 'svelte-fa';
	import { formatDeviceID } from '$lib/utils';
	import moment from 'moment';
	import type { EChartsOption } from 'echarts';
	import { useECharts } from '$lib/echarts';

	let statDetails: StatDetails | null = null;

	var networkTrafficOption: EChartsOption = {
		title: { text: 'Network Traffic' },
		tooltip: {
			trigger: 'axis'
		},
		grid: {
			left: '12%',
			right: '3%',
			bottom: '10%'
		},
		xAxis: {
			type: 'category',
			data: ['22-12-21', '22-12-22', '22-12-23', '22-12-24', '22-12-25', '22-12-26', '22-12-27']
		},
		yAxis: {
			type: 'value',
			axisLabel: {
				formatter: '{value} MB'
			}
		},
		series: [
			{
				data: [1253.36, 2789.36, 440.66, 934.78, 6523.12, 4588.14, 963.4],
				type: 'line',
				smooth: true
			}
		]
	};

	var sessionsOption: EChartsOption = {
		title: { text: 'Sessions' },
		tooltip: {
			trigger: 'axis'
		},
		grid: {
			left: '6%',
			right: '3%',
			bottom: '10%'
		},
		xAxis: {
			type: 'category',
			data: ['22-12-21', '22-12-22', '22-12-23', '22-12-24', '22-12-25', '22-12-26', '22-12-27']
		},
		yAxis: {
			type: 'value'
		},
		series: [
			{
				data: [12, 10, 14, 13, 15, 8, 6],
				type: 'line',
				smooth: true
			}
		]
	};

	var onlineDevicesOption: EChartsOption = {
		title: { text: 'Online Devices' },
		tooltip: {
			trigger: 'axis'
		},
		grid: {
			left: '6%',
			right: '3%',
			bottom: '10%'
		},
		xAxis: {
			type: 'category',
			data: ['22-12-21', '22-12-22', '22-12-23', '22-12-24', '22-12-25', '22-12-26', '22-12-27']
		},
		yAxis: {
			type: 'value'
		},
		series: [
			{
				data: [24, 32, 41, 33, 66, 14, 20],
				type: 'line',
				smooth: true
			}
		]
	};

	onMount(async () => {
		statDetails = await fetchStatDetails();
	});
</script>

<svelte:head>
	<title>Overview</title>
	<meta name="description" content="Svelte demo app" />
</svelte:head>

<div class="w-full h-full flex flex-col gap-4">
	<DetailsPanel
		domain={statDetails?.domain ?? ''}
		allocated={statDetails?.allocated ?? 0}
		bytesTransferred={statDetails?.bytes_transferred ?? 0}
	/>

	<!-- <div class="overflow-x-auto">
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
	</div> -->

	<div class="flex flex-row w-full gap-4">
		<div class="flex-1 flex flex-row shadow px-4 pt-4 rounded-2xl">
			<div class="flex-1 flex flex-col gap-8 items-center justify-center">
				<div
					class="radial-progress bg-primary text-primary-content border-4 border-primary shadow"
					style="--value:8; --size:8rem;"
				>
					8%
				</div>
				<div>CPU</div>
			</div>

			<div class="flex-1 flex flex-col gap-8 items-center justify-center">
				<div
					class="radial-progress bg-primary text-primary-content border-4 border-primary shadow"
					style="--value:12; --size:8rem;"
				>
					12%
				</div>
				<div>Memory</div>
			</div>

			<!-- <div class="flex-1 flex flex-col gap-8 items-center justify-center">
				<div
					class="radial-progress bg-primary text-primary-content border-4 border-primary shadow"
					style="--value:2; --size:8rem;"
				>
					2%
				</div>
				<div>Network Pressure</div>
			</div> -->

			<div class="flex-1 flex flex-col gap-8 items-center justify-center">
				<div
					class="radial-progress bg-error text-primary-content border-4 border-error shadow"
					style="--value:85; --size:8rem;"
				>
					85%
				</div>
				<div>Storage Usage</div>
			</div>
		</div>

		<div class="flex-1 flex flex-col rounded-2xl shadow px-4 pt-4">
			<div id="network-charts" class="w-full h-72" use:useECharts={onlineDevicesOption} />
		</div>

		
	</div>

	<div class="flex flex-row w-full gap-4">

		<div class="flex-1 flex flex-col rounded-2xl shadow px-4 pt-4">
			<div id="network-charts" class="w-full h-72" use:useECharts={networkTrafficOption} />
		</div>

		<div class="flex-1 flex flex-col rounded-2xl shadow px-4 pt-4">
			<div id="network-charts" class="w-full h-72" use:useECharts={sessionsOption} />
		</div>

		
	</div>
</div>
