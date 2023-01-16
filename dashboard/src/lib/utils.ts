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

export function joinClasses(classVariables: string[], ...otherClasses: string[]): string {
	otherClasses.push(...classVariables);
	return otherClasses.join(' ');
}

export function formatTransferSpeed(size: number): string {
	const num = 1024.0; //byte

	if (size < num) return size + ' b/s';
	if (size < Math.pow(num, 2)) return (size / num).toFixed(2) + ' Kb/s';
	if (size < Math.pow(num, 3)) return (size / Math.pow(num, 2)).toFixed(2) + ' Mb/s';
	if (size < Math.pow(num, 4)) return (size / Math.pow(num, 3)).toFixed(2) + ' Gb/s';
	return (size / Math.pow(num, 4)).toFixed(2) + ' Tb/s';
}
