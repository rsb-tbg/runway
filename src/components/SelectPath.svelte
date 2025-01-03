<script lang="ts">
	import { Button, Radio } from 'flowbite-svelte';
	import { ArrowLeftOutline, ArrowRightOutline } from 'flowbite-svelte-icons';
	import { invoke } from '@tauri-apps/api/tauri'
	import { next_view, prev_view } from '../utils';

	let disabled:boolean = true;

	let selected_path = get_path();
	async function get_path():Promise<{ path: string; }> {
		let selected_path:{path: string} = await invoke('get_path');
		if (selected_path.path != "") {
			disabled = false;
		} else {
			disabled = true;
		}
		return selected_path
	}

	async function select_file() {
		await invoke('select_file');
		selected_path = get_path();
		invoke('get_file_vec')
	}

	async function select_directory() {
		await invoke('select_directory');
		selected_path = get_path();
		invoke('get_file_vec')
	}

	async function set_video_feed() {
		await invoke('set_video_feed')
		selected_path = get_path();
	}

</script>

<section>
	<br/>
	    <h1 style="text-decoration: underline;">Select data for processing:</h1>
	<br/>

	<div class="grid gap-6 w-fit md:grid-cols-1 min-w-max">
		<Radio on:click={()=>select_file()} name="custom" custom>
		  <div class="inline-flex justify-between items-center p-5 w-full text-gray-500 hover:text-gray-700 bg-transparent rounded-lg border border-gray-200 cursor-pointer dark:hover:text-gray-300 dark:border-gray-700 dark:peer-checked:text-primary-500 peer-checked:border-primary-600 peer-checked:text-primary-600 hover:border-black hover:bg-teal-100 dark:text-gray-400 dark:bg-gray-800 dark:hover:bg-gray-700">
			<div>
			  <div class="w-full text-lg font-semibold">Find Face Matches</div>
			  <div class="w-full">Selected face will be compared and stored in database</div>
			</div>
			<ArrowRightOutline class="ml-5 w-6 h-6" />
		  </div>
		</Radio>
		<Radio on:click={()=>select_directory()} name="custom" custom>
			<div class="inline-flex justify-between items-center p-5 w-full text-gray-500 hover:text-gray-700 bg-transparent rounded-lg border border-gray-200 cursor-pointer dark:hover:text-gray-300 dark:border-gray-700 dark:peer-checked:text-primary-500 peer-checked:border-primary-600 peer-checked:text-primary-600 hover:border-black hover:bg-teal-100 dark:text-gray-400 dark:bg-gray-800 dark:hover:bg-gray-700">
			<div class="block">
			  <div class="w-full text-lg font-semibold">Load Faces to Database</div>
			  <div class="w-full">Faces from selected images will be stored in database</div>
			</div>
			<ArrowRightOutline class="ml-5 w-6 h-6" />
		  </div>
		</Radio>
		<Radio on:click={()=>set_video_feed()} name="custom" custom>
			<div class="inline-flex justify-between items-center p-5 w-full text-gray-500 hover:text-gray-700 bg-transparent rounded-lg border border-gray-200 cursor-pointer dark:hover:text-gray-300 dark:border-gray-700 dark:peer-checked:text-primary-500 peer-checked:border-primary-600 peer-checked:text-primary-600 hover:border-black hover:bg-teal-100 dark:text-gray-400 dark:bg-gray-800 dark:hover:bg-gray-700">
			<div class="block">
			  <div class="w-full text-lg font-semibold">Monitor Video Feed</div>
			  <div class="w-full">Faces from selected video feed will be stored in database</div>
			</div>
			<ArrowRightOutline class="ml-5 w-6 h-6" />
		  </div>
		</Radio>
	  </div>
	  <br/>

	{#if !disabled}
	  {#await selected_path}
	  	<h1>Selection: <u style="text-decoration:underline">...</u></h1>
	  {:then result} 
	  	<h1>Selection: <u style="text-decoration:underline">{result.path}</u></h1>
	  {:catch error}
	  	<h1>Selection: <u style="text-decoration:underline">{error}</u></h1>
	  {/await}
	{/if}

	<div id="prev" class="dark:bg-gray-800 dark:text-white">
		<Button on:click={()=>prev_view()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
			<ArrowLeftOutline class="w-3.5 h-3.5 mr-2" />Previous
		</Button>
	</div>

	<div id="next" class="dark:bg-gray-800 dark:text-white">
		<Button {disabled} on:click={()=>next_view()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
			Next<ArrowRightOutline class="w-3.5 h-3.5 ml-2" />
		</Button>
	</div>
</section>