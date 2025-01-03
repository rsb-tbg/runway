<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
	import type { SvelteComponent } from 'svelte';
	import NoFace from './NoFace.svelte';
	import OneFace from './OneFace.svelte';
	import MultipleFaces from './MultipleFaces.svelte';
	import { appWindow } from '@tauri-apps/api/window';
	import { error } from '@sveltejs/kit';

	let change_query_view:boolean =  false;
	appWindow.listen("CHANGE_QUERY_VIEW", ({payload}) => {change_query_view = payload as boolean});

	let query_view = get_query_view();
	$: if (change_query_view) {
		query_view = get_query_view();
	}
	
	async function get_query_view():Promise<typeof SvelteComponent> {
		let qv:typeof SvelteComponent;
		let face_count:{face_count: number} = await invoke('get_face_count');
		if ( face_count.face_count == 0 ) {
			qv = NoFace as typeof SvelteComponent;
		} else if ( face_count.face_count == 1) {
			qv = OneFace as typeof SvelteComponent;
		} else {
			qv = MultipleFaces as typeof SvelteComponent;
		}
		return qv
	}

</script>

<section style="height: 100%;">
	{#await query_view}
		<h1>awaiting</h1>
	{:then result}
		<svelte:component this={result}/>
	{:catch error}
		<h1>{error}</h1>
	{/await}
</section>