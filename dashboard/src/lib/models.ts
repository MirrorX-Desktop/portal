export interface StatDetails {
	domain: string;
	allocated: number;
	bytes_transferred: number;
	client_snapshot: Array<{
		active_device_id: number;
		active_addr: string;
		passive_device_id: number;
		passive_addr: string;
		timestamp: number;
	}>;
}
