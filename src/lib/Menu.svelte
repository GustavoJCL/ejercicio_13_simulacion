<script lang="ts">
	import { TabGroup, Tab } from '@skeletonlabs/skeleton';
	import '@fortawesome/fontawesome-free/js/all.js';
	import '@fortawesome/fontawesome-free/css/all.css';
	import PruebaHuecos from './PruebaHuecos.svelte';
	import PruebaArribaAbajo from './PruebaArribaAbajo.svelte';
	import PruebaArribaAbajoMedia from './PruebaArribaAbajoMedia.svelte';
	import PruebaSeries from './PruebaSeries.svelte';

	export let csv_string: string;

	let tabSet: Number = 0;
</script>

<TabGroup
	justify="justify-center"
	active="variant-filled-primary"
	hover="hover:variant-soft-primary"
	flex="flex-1 lg:flex-none"
	rounded=""
	border=""
	class="w-full bg-surface-100-800-token"
>
	<Tab bind:group={tabSet} name="prueba_huecos" value={0}>
		<!-- <svelte:fragment slot="lead">fas fa-home</svelte:fragment> -->
		<i slot="lead" class="text-base fas fa-flask" />
		<span>Prueba de Huecos</span>
	</Tab>
	<Tab bind:group={tabSet} name="prueba_poker" value={1}>
		<i slot="lead" class="text-base fas fa-play-card" />
		<span>Prueba de Poker</span>
	</Tab>
	<Tab bind:group={tabSet} name="prueba_arriba_abajo" value={2}>
		<i slot="lead" class="text-base fas fa-arrows-alt-v" />
		<span>Prueba de Arriba y Abajo</span>
	</Tab>
	<Tab bind:group={tabSet} name="prueba_arriba_abajo_media" value={3}>
		<i slot="lead" class="text-base fas fa-chart-line" />
		<span>Prueba de Arriba y Abajo Media</span>
	</Tab>
	<Tab bind:group={tabSet} name="prueba_series" value={4}>
		<i slot="lead" class="text-base fas fa-list-ol" />
		<span>Prueba de Series</span>
	</Tab>
	<!-- Tab Panels --->
	<svelte:fragment slot="panel">
		{#key tabSet}
			{#if tabSet === 0}
				<PruebaHuecos {csv_string} />
			{:else if tabSet === 1}
				(tab panel 2 contents)
			{:else if tabSet === 2}
				<PruebaArribaAbajo {csv_string} />
			{:else if tabSet === 3}
				<PruebaArribaAbajoMedia {csv_string} />
			{:else if tabSet === 4}
				<PruebaSeries {csv_string} />
			{/if}
		{/key}
	</svelte:fragment>
</TabGroup>
