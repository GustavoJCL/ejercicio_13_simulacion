<script lang="ts">
	import numberOutlined from '@iconify/icons-ant-design/number-outlined';
	import characterDecimal from '@iconify/icons-carbon/character-decimal';
	import igorPro from '@iconify/icons-file-icons/igor-pro';
	import checkCorrect from '@iconify/icons-icon-park-outline/check-correct';
	import Icon from '@iconify/svelte';
	import { invoke } from '@tauri-apps/api/tauri';
	import { Table, tableMapperValues, type TableSource } from '@skeletonlabs/skeleton';
	import type { ResponseData as ResponseDataArr } from './Interfaces/interfacesPruebaArribaAbajo';
	import type { GraphDistributionValues } from './Interfaces/interfacesGraphDistribution';
	import type { SourceDataItem } from './Interfaces/interfacesPruebaArribaAbajo';
	import NormalDistGraph from './ChartJs/NormalDistGraph.svelte';
	import { afterUpdate } from 'svelte';

	export let csv_string: string;
	let nivel = '';
	let validInputs = false;
	let showResults = false;
	let response_data: ResponseDataArr;
	let estadistico_tabla_graph: [number, number];
	let sourceData: SourceDataItem[] = [];
	// let normalDistGraph = NormalDistGraph;
	// afterUpdate(()=>{
	// 	normalDistGraph = estadistico_tabla_graph;
	// });

	async function prueba_arriba_abajo(nivel_confianza: number) {
		return (await invoke('prueba_arriba_abajo', {
			data_string: csv_string,
			nivel_confianza: nivel_confianza
		})) as Object;
	}
	let validar_datos = () => {
		let nivel_value = parseFloat(nivel);
		if (!isNaN(nivel_value) && nivel_value >= 0 && nivel_value < 1 && csv_string !== undefined) {
			validInputs = true;
		} else {
			validInputs = false;
		}
	};
	async function btn_prueba_click() {
		if (validInputs) {
			response_data = (await prueba_arriba_abajo(parseFloat(nivel))) as ResponseDataArr;
			showResults = true;

			sourceData = [];
			sourceData.push({
				mu: response_data.mu,
				sigma: response_data.sigma,
				estadistico: response_data.estadistico,
				estadistico_tabla: response_data.estadistico_tabla,
				corridas: response_data.corridas
			});

			estadistico_tabla_graph = [parseFloat(nivel), response_data.estadistico_tabla];
			// let alpha = 1 - parseFloat(nivel);
			// console.log('praise the omnissiah');
			// console.log(response_data);
			// console.log(sourceData);
			// console.log(estadistico_tabla_graph);
			// console.log(response_data.normal_inv_value);
		}
	}
	let getTableData: TableSource;
	$: {
		if (response_data && response_data.estadistico_tabla !== undefined) {
			let result_dis_norm =
				sourceData[0].estadistico <= sourceData[0].estadistico_tabla
					? 'Se rechaza que los numeros son independientes'
					: 'No se rechaza que los numeros son independientes';
			getTableData = {
				head: [
					'mu (μ)',
					'Sigma (σ²)',
					'Corridas',
					'Estadistico (Z)',
					'Estadistico de Tabla (Z a/2)'
				],
				body: tableMapperValues(sourceData, [
					'mu',
					'sigma',
					'corridas',
					'estadistico',
					'estadistico_tabla'
				]),
				meta: tableMapperValues(sourceData, ['position', 'name', 'symbol', 'weight']),
				foot: ['Conclusion: ', '', result_dis_norm.toString()]
			};
		}
	}
</script>

<div class="flex relative flex-col gap-2 items-center max-w-xl">
	<label class="flex flex-col order-1 w-full label">
		<span class="">Nivel de confianza</span>
		<input
			bind:value={nivel}
			class="flex-grow w-full h-9 appearance-none input"
			type="text"
			on:change={validar_datos}
			placeholder="Nivel de confianza"
		/>
		<div class="flex flex-row items-center text-error-400">
			<!-- svelte-ignore empty-block -->
			{#if nivel === ''}{:else if isNaN(parseFloat(nivel))}
				<Icon icon={numberOutlined} />
				<p>Ingrese un valor numerico</p>
			{:else if parseFloat(nivel) < 0 || parseFloat(nivel) >= 1}
				<Icon icon={characterDecimal} />
				<p>El valor debe estar entre 0 y 1</p>
			{:else}
				<div class="items-center text-secondary-300-600-token">
					<Icon icon={checkCorrect} />
					<p>Correcto</p>
				</div>
			{/if}
		</div>
	</label>
	<button
		disabled={!validInputs}
		on:click={btn_prueba_click}
		class="order-3 items-center m-3 btn variant-filled-surface"
	>
		<Icon class="text-2xl" icon={igorPro} />
		<span class="text-1xl">Realizar Prueba</span>
	</button>
</div>
{#if showResults}
	<div class="order-4 w-fit">
		<p class="text-2xl font-bold">Resultado</p>
		<Table interactive={true} source={getTableData} />
		<br />
		<div class="p-3 max-w-full rounded border-gray-300 bg-primary-100">
			{#key response_data}
				<NormalDistGraph
					dis_norm_values_i={response_data.normal_inv_value}
					estadistico_tabla_i={estadistico_tabla_graph}
				/>
			{/key}
		</div>
	</div>
{/if}
