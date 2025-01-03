<script lang="ts">
    import { convertFileSrc, invoke } from '@tauri-apps/api/tauri'
	import { Button } from 'flowbite-svelte';
	import { prev_view } from '../../../utils';
	import { ArrowLeftOutline, CaretRightSolid } from 'flowbite-svelte-icons';
	import { appWindow } from '@tauri-apps/api/window';
	import { onMount } from 'svelte';

    let img_cont_max_height:string = "calc(100% - 174px)";
    onMount(()=>{adjust_columns()});
    function adjust_columns() {
        const img_container:HTMLElement|null = document.getElementById("img_container");
        let col_ct:string = (Math.round(window.innerWidth / 360)).toString();
        if (img_container != null) {
            img_container.style.gridTemplateColumns = "repeat(" + col_ct + ", minmax(0, 1fr))";
            img_container.style.maxHeight = img_cont_max_height;
        };
    }
    
	let disabled:boolean = true;

    let results = get_results();
	async function get_results() {
		await invoke('compare_face');
		disabled = false;
	}

	type matched_face = {
		file_path: string,
		person_name: string,
		euc_dist: number,
		cos_sim: number,
		e_c_f: string,
	}

	let matched_names:number = 0;
	appWindow.listen("MATCHED_NAME", (() => matched_names += 1));

	let matched_faces:matched_face[] = [];
	$: if (matched_faces.length != 0) {
		document.getElementById("spinner")?.remove();
	}
	appWindow.listen("MATCHED_FACE", ({payload}) => {
		matched_faces.push(payload as matched_face);
		matched_faces = matched_faces;
	});

</script>

<svelte:window on:resize={()=>adjust_columns()}/>
<section style="height: 100%;">
	<div style="height: 100px">
		<h1 style="text-decoration: underline; padding-top: 5px;">Results:</h1>
		<table>
			<tr>
				<td><CaretRightSolid size="xs" style="float: left; margin-right: 10px;"/></td>
				<td>Number of matched names:</td>
				<td style="padding-left: 15px">{matched_names.toLocaleString()}</td>
			</tr>
			<tr>
				<td><CaretRightSolid size="xs" style="float: left; margin-right: 10px;"/></td>
				<td>Number of matched faces:</td>
				<td style="padding-left: 15px">{matched_faces.length.toLocaleString()}</td>
			</tr>
		</table>
	</div>
	<div style="max-height: {img_cont_max_height}; overflow-y: auto;" id="img_container" class="grid gap-4 dark:[color-scheme:dark]">
		{#each matched_faces as face}
			<div class="rounded-lg" style="position:relative; height: auto; width: auto; font-size: 12px; text-align: center;">
				<p style="height: auto; width: 100%;">Name: {face.person_name}</p>
				<img class="rounded-lg" style="height: 184px; width: 128px; display: block; margin: auto;" src={face.e_c_f} alt="" loading="lazy">
				<p style="height: auto; width: 100%;">L2 Norm: {Math.round((face.euc_dist + Number.EPSILON) * 1000) / 1000}</p>
				<p style="height: auto; width: 100%">Cosine Sim.: {Math.round((face.cos_sim + Number.EPSILON) * 1000) / 1000}</p>
			</div>
		{/each}
	</div>
	<div id="prev" class="dark:bg-gray-800 dark:text-white">
		<Button {disabled} id="prev_button" on:click={()=>prev_view()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
			<ArrowLeftOutline class="w-3.5 h-3.5 mr-2" />Previous
		</Button>
	</div>

</section>