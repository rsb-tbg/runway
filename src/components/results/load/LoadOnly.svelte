<script lang="ts">
	import { Button } from 'flowbite-svelte';
	import { prev_view } from '../../../utils';
	import { ArrowLeftOutline, CaretRightSolid } from 'flowbite-svelte-icons';
	import { appWindow } from '@tauri-apps/api/window';
	import { invoke } from '@tauri-apps/api/tauri';

    
	let disabled:boolean = true;
	let results = load_only();
	async function load_only() {
		await invoke('load_only');
		disabled = false;
	}

	let processed_images_ct:number;
	appWindow.listen("PROCESSED_IMAGES_CT", ({payload}) => {processed_images_ct = payload as number});

	let total_face_ct:number;
	appWindow.listen("TOTAL_FACES_DETECTED", ({payload}) => {total_face_ct = payload as number});

	let show_ct_results:boolean = false;
	$: if (processed_images_ct == undefined && total_face_ct == undefined) {
		set_dots_interval();
	} else {
		clearInterval(dots_interval);
		show_ct_results = true;
		document.getElementById("spinner")?.remove();
	}

	let dots_interval:NodeJS.Timeout;
	function set_dots_interval() {
		if (!dots_interval) {
			dots_interval = setInterval(dot_dot_dot, 500)
		}
	}

	let dots:string = ". ";
	function dot_dot_dot():string {
		if (dots.length == 10) {
			dots = ". "
		} else {
			dots = dots + ". "
		}
		return dots
	}

</script>

<section style="height: 100%;">
	<div style="height: 110px">
		<h1 style="text-decoration: underline; padding-top: 5px;">Summary:</h1>
		<table>
			<tr>
				<td><CaretRightSolid size="xs" style="float: left; margin-right: 10px;"/></td>
				<td>Number of processed images:</td>
				{#if show_ct_results && processed_images_ct != null}
					<td style="padding-left: 15px;">{processed_images_ct.toLocaleString()}</td>
				{:else}
					<td style="padding-left: 15px;">{dots}</td>
				{/if}
			</tr>
			<tr>
				<td><CaretRightSolid size="xs" style="float: left; margin-right: 10px;"/></td>
				<td>Total number of faces detected:</td>
				{#if show_ct_results && total_face_ct != null}
					<td style="padding-left: 15px">{total_face_ct.toLocaleString()}</td>
				{:else}
					<td style="padding-left: 15px;">{dots}</td>
				{/if}
			</tr>
		</table>
	</div>

	<div id="prev" class="dark:bg-gray-800 dark:text-white">
		<Button {disabled} on:click={()=>prev_view()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
			<ArrowLeftOutline class="w-3.5 h-3.5 mr-2" />Previous
		</Button>
	</div>
</section>