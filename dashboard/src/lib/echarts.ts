import * as echarts from 'echarts';

export function useECharts(node: HTMLElement, option: echarts.EChartsOption) {
	const chart = echarts.init(node);
	chart.setOption(option);
}
