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

export interface MetricsInfo {
	cpu_usage: string;
	memory_usage: string;
	network_in_bytes: number;
	network_out_bytes: number;
	network_in_packets: number;
	network_out_packets: number;
}
