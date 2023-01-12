<script lang="ts">
	import type { StatDetails } from '$lib/models';
	import { formatNumberWithCommas } from '$lib/utils';
	import {
		faHardDrive,
		faHouseSignal,
		faArrowRightArrowLeft,
		faDownload
	} from '@fortawesome/free-solid-svg-icons';
	import Fa from 'svelte-fa';

	export let domain: string;
	export let allocated: number;
	export let bytesTransferred: number;

	const getFileSize = (size: number): string => {
		var num = 1024.0; //byte

		if (size < num) return `${formatNumberWithCommas(size.toString())} B`;
		if (size < Math.pow(num, 2))
			return `${formatNumberWithCommas((size / num).toFixed(2).toString())} KB`;
		if (size < Math.pow(num, 3))
			return `${formatNumberWithCommas((size / Math.pow(num, 2)).toFixed(2).toString())} MB`;
		if (size < Math.pow(num, 4))
			return `${formatNumberWithCommas((size / Math.pow(num, 3)).toFixed(2).toString())} GB`;

		return `${formatNumberWithCommas((size / Math.pow(num, 4)).toFixed(2).toString())} TB`;
	};
</script>

<div class="stats shadow flex flex-row">
	<div class="stat flex-1">
		<div class="stat-figure text-primary">
			<Fa icon={faHouseSignal} size="1.5x" class="stroke-current" />
		</div>
		<div class="stat-title">Domain</div>
		<div class="stat-value">
			{domain}
		</div>
		<div class="stat-desc" />
	</div>

	<div class="stat flex-1">
		<div class="stat-figure text-primary">
			<Fa icon={faHardDrive} size="1.5x" class="stroke-current" />
		</div>
		<div class="stat-title">Device ID</div>
		<div class="stat-value">
			{allocated}
		</div>
		<div class="stat-desc">In total</div>
	</div>

	<div class="stat flex-1">
		<div class="stat-figure text-primary">
			<Fa icon={faArrowRightArrowLeft} size="1.5x" class="stroke-current" />
		</div>
		<div class="stat-title">Data Transferred</div>
		<div class="stat-value">
			{getFileSize(bytesTransferred)}
		</div>
		<div class="stat-desc">Today</div>
	</div>
</div>
