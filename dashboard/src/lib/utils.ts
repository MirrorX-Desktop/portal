export const formatNumberWithCommas = (num: string): string => {
	return num.replace(/\B(?=(\d{3})+(?!\d))/g, ',');
};

export function formatDeviceID(deviceID: number): string {
	const deviceIDStr = String(deviceID).padStart(10, '0');
	return `${deviceIDStr.substring(0, 2)}-${deviceIDStr.substring(2, 6)}-${deviceIDStr.substring(
		6,
		10
	)}`;
}