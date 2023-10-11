<script lang="ts">
	import type { GraphDistributionValues } from '../Interfaces/interfacesGraphDistribution';
	import { Scatter } from 'svelte-chartjs';
	import {
		Chart as ChartJS,
		Title,
		Tooltip,
		Legend,
		LineElement,
		LinearScale,
		PointElement,
		CategoryScale
	} from 'chart.js';

	export let dis_norm_values_i: GraphDistributionValues;
	export let dis_norm_values_s: GraphDistributionValues = [];
	export let estadistico_tabla_i: [number, number];
	export let estadistico_tabla_s: [number, number] = [-1, -1];

	console.log('heil esquizo');
	$: {
		console.log(estadistico_tabla_i);
		console.log(estadistico_tabla_s);
	}

	ChartJS.register(Title, Tooltip, Legend, LineElement, LinearScale, PointElement, CategoryScale);

	const data_estadistico_tabla_i: [number, number][] = Object.entries(dis_norm_values_i).map(
		([_, value]) => {
			return [value[0], value[1]];
			// console.log(`Key: ${key}, Value: ${value}`);
		}
	);
	const data_estadistico_tabla_s: [number, number][] = Object.entries(dis_norm_values_s).map(
		([_, value]) => {
			return [value[0], value[1]];
			// console.log(`Key: ${key}, Value: ${value}`);
		}
	);
	let data = {
		labels: data_estadistico_tabla_i.map((item) => item[0]),
		datasets: [
			{
				label: 'Valores de Distribucion Normal Z',
				data: data_estadistico_tabla_i.map((item) => item[1]),
				showLine: true,
				backgroundColor: 'rgba(75, 192, 230, 0.2)',
				pointRadius: 0.5
				// cubicInterpolationMode: 'monotone'
			},
			{
				label: 'Estadistico de Tabla Inferior',
				data: [{ x: estadistico_tabla_i[0], y: estadistico_tabla_i[1] }],
				borderColor: 'rgba(0, 0, 0, 0)',
				borderWidth: 2,
				pointBackgroundColor: 'rgb(255, 0, 0)',
				pointRadius: 5,
				pointHoverRadius: 5,
				borderDash: [5, 5]
			}
		]
	};
	if (estadistico_tabla_s[0] !== -1 && estadistico_tabla_s[1] !== -1) {
		data.datasets.push({
			label: 'Estadistico de Tabla Superior',
			data: [{ x: estadistico_tabla_s[0], y: estadistico_tabla_s[1] }],
			borderColor: 'rgba(0, 0, 0, 0)',
			borderWidth: 2,
			pointBackgroundColor: 'rgb(255, 0, 0)',
			pointRadius: 5,
			pointHoverRadius: 5,
			borderDash: [5, 5]
		});
	} else {
		data.datasets[1].label = 'Estadistico de Tabla';
	}
	if (data_estadistico_tabla_s.length !== 0) {
		data.datasets.push({
			label: 'Valores de Distribucion Normal Z superior',
			data: data_estadistico_tabla_s.map((item) => item[1]),
			showLine: true,
			backgroundColor: 'rgba(75, 192, 230, 0.2)',
			pointRadius: 0.5
		});
		data.datasets[0].label = 'Valores de Distribucion Normal Z inferior';
	}
	// console.log(data);

	let options = {
		responsive: true
	};
</script>

<Scatter {data} {options} />
