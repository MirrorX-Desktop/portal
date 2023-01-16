<script lang="ts">
	import { fetchStatDetails, fetchStatMetrics } from '$lib/api';
	import type { MetricsInfo, StatDetails } from '$lib/models';
	import { faSpinner, faExclamationCircle } from '@fortawesome/free-solid-svg-icons';
	import { onDestroy, onMount } from 'svelte';
	import DetailsPanel from './details_panel.svelte';
	import Fa from 'svelte-fa';
	import { formatDeviceID, formatTransferSpeed, joinClasses } from '$lib/utils';
	import moment from 'moment';
	import type { EChartsOption } from 'echarts';
	import { useECharts } from '$lib/echarts';
	import { to_number } from 'svelte/internal';

	let statDetails: StatDetails | null = null;
	let metricsInfo: MetricsInfo | null = null;
	let metricsTimer: NodeJS.Timer | null = null;
	$: cpu_usage = to_number(metricsInfo?.cpu_usage ?? 0);
	$: memory_usage = to_number(metricsInfo?.memory_usage ?? 0);

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
		// statDetails = await fetchStatDetails();
		metricsTimer = setInterval(async () => {
			metricsInfo = await fetchStatMetrics();
		}, 1000);
	});

	onDestroy(() => {
		if (metricsTimer) {
			clearInterval(metricsTimer);
		}
	});

	const metricsColor = (value: number): Array<string> => {
		if (value >= 80) {
			return ['bg-error', 'border-error'];
		} else if (value >= 40) {
			return ['bg-warning', 'border-warning'];
		} else {
			return ['bg-success', 'border-success'];
		}
	};
</script>

<svelte:head>
	<title>Overview | MirrorX</title>
	<meta name="description" content="MirrorX" />
</svelte:head>

<div class="w-full h-full flex flex-col gap-4">
	<div class="flex-0">
		<DetailsPanel
			domain={statDetails?.domain ?? ''}
			allocated={statDetails?.allocated ?? 0}
			bytesTransferred={statDetails?.bytes_transferred ?? 0}
		/>
	</div>

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
		<div class="shadow py-4 px-10 rounded-2xl flex flex-col gap-4">
			<div class="flex flex-row items-center justify-center gap-8">
				<div class="flex-1 flex flex-col gap-8 items-center justify-center">
					<div
						class={joinClasses(
							metricsColor(cpu_usage),
							'radial-progress',
							'text-primary-content',
							'border-4',
							'shadow'
						)}
						style="--value:{cpu_usage}; --size:8rem;"
					>
						{cpu_usage}%
					</div>
					<div class="font-bold">CPU</div>
				</div>

				<div class="flex-1 flex flex-col gap-8 items-center justify-center">
					<div
						class={joinClasses(
							metricsColor(memory_usage),
							'radial-progress',
							'text-primary-content',
							'border-4',
							'shadow'
						)}
						style="--value:{memory_usage}; --size:8rem;"
					>
						{memory_usage}%
					</div>
					<div class="font-bold">Memory</div>
				</div>
			</div>
			<hr />
			<div class="flex-1 flex flex-col items-center justify-evenly">
				<div class="flex flex-row items-center justify-between w-full">
					<div class="font-bold">In Bytes</div>
					<div>{formatTransferSpeed(metricsInfo?.network_in_bytes ?? 0)}</div>
				</div>
				<div class="flex flex-row items-center justify-between w-full">
					<div class="font-bold">Out Bytes</div>
					<div>{formatTransferSpeed(metricsInfo?.network_out_bytes ?? 0)}</div>
				</div>
				<div class="flex flex-row items-center justify-between w-full">
					<div class="font-bold">In Packets</div>
					<div>{metricsInfo?.network_in_packets ?? 0} pkt/s</div>
				</div>
				<div class="flex flex-row items-center justify-between w-full">
					<div class="font-bold">Out Packets</div>
					<div>{metricsInfo?.network_out_packets ?? 0} pkt/s</div>
				</div>
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
