import type { StatDetails } from './models';

export const fetchStatDetails = async (): Promise<StatDetails> => {
	const response = await fetch('/api/stat/details');
	return response.json();
};
