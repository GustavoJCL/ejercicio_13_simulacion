<script lang="ts">
	import {
		Chart as ChartJS,
		Title,
		Tooltip,
		Legend,
		LineElement,
		CategoryScale,
		LinearScale,
		PointElement
	} from 'chart.js';
	import { Scatter } from 'svelte-chartjs';
	import type { SourceDataItem } from '../Interfaces/interfacesHuecos.ts';

	export let sourceData: SourceDataItem[];

	ChartJS.register(Title, Tooltip, Legend, LineElement, CategoryScale, LinearScale, PointElement);
	let data = {
		labels: ['Scatter'],
		datasets: [
			{
				label: 'Estadistico',
				data: sourceData.map((item) => ({ x: item.indice, y: item.estadistico })),
				backgroundColor: 'rgba(75, 192, 192, 0.2)',
				borderColor: 'rgba(75, 192, 192, 1)',
				borderWidth: 1
			},
			{
				label: 'Frecuencia Esperada',
				data: sourceData.map((item) => ({ x: item.indice, y: item.frecuencia_esperada })),
				backgroundColor: 'rgba(255, 99, 132, 0.2)',
				borderColor: 'rgba(255, 99, 132, 1)',
				borderWidth: 1
			},
			{
				label: 'Frecuencia Observada',
				data: sourceData.map((item) => ({ x: item.indice, y: item.frecuencia_observada })),
				backgroundColor: 'rgba(54, 162, 235, 0.2)',
				borderColor: 'rgba(54, 162, 235, 1)',
				borderWidth: 1
			}
		]
	};

	let options = {
		responsive: true,
		scales: {
			x: {
				beginAtZero: true,
				title: {
					display: true,
					text: 'Index'
				}
			},
			y: {
				beginAtZero: true,
				title: {
					display: true,
					text: 'Value'
				}
			}
		}
	};
</script>

<Scatter {data} {options} />
