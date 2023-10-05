<script lang="ts">
	import Menu from '$lib/Menu.svelte';
	import Greet from '../lib/Greet.svelte';
	import { FileDropzone } from '@skeletonlabs/skeleton';
	let files: File[] = [];
	function handleDrop(event: any) {
		files = event.detail.files;
	}
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>

<h1>Welcome to SvelteKit</h1>
<Greet />

<div>
	<FileDropzone name="file_upload" on:drop={handleDrop} />
	<Menu />
</div>
