<script lang="ts">
	import { AppRail, AppRailTile, AppRailAnchor } from '@skeletonlabs/skeleton';

	let selectedOption = '';

	const handleOptionSelect = (event: Event) => {
		const select = event.target as HTMLSelectElement;
		selectedOption = select.value;
		// Handle the selected option here
		console.log(selectedOption);
	};
	let currentTitle = 0;
</script>

<div>
	<AppRail>
		<svelte:fragment slot="lead">
			<AppRailAnchor href="/">(icon)</AppRailAnchor>
		</svelte:fragment>
		<AppRailTile bind:group={currentTitle} name="title-1" value="0" title="title-2">
			<AppRailAnchor href="#" on:click|preventDefault={handleOptionSelect}>
				Select an option
			</AppRailAnchor>
		</AppRailTile>
		<AppRailTile>
			<AppRailAnchor
				href="/prueba_huecos"
				on:click|preventDefault={handleOptionSelect}
				value="Prueba de Huecos"
			>
				Prueba de Huecos
			</AppRailAnchor>
		</AppRailTile>
		<AppRailTile>
			<AppRailAnchor href="#" on:click|preventDefault={handleOptionSelect} value="Prueba de Pocker">
				Prueba de Pocker
			</AppRailAnchor>
		</AppRailTile>
		<AppRailTile>
			<AppRailAnchor
				href="#"
				on:click|preventDefault={handleOptionSelect}
				value="Prueba arriba abajo"
			>
				Prueba arriba abajo
			</AppRailAnchor>
		</AppRailTile>
		<AppRailTile>
			<AppRailAnchor
				href="#"
				on:click|preventDefault={handleOptionSelect}
				value="Prueba arriba abajo de medias"
			>
				Prueba arriba abajo de medias
			</AppRailAnchor>
		</AppRailTile>
		<AppRailTile>
			<AppRailAnchor href="#" on:click|preventDefault={handleOptionSelect} value="Prueba de Series">
				Prueba de Series
			</AppRailAnchor>
		</AppRailTile>
	</AppRail>
</div>
