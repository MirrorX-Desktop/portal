import type { MetricsInfo, StatDetails } from './models';

export const fetchStatDetails = async (): Promise<StatDetails> => {
	const response = await fetch('/api/stat/details');
	return response.json();
};

export const fetchStatMetrics = async (): Promise<MetricsInfo> => {
	const response = await fetch('/api/stat/metrics');
	return response.json();
};
