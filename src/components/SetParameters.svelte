<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	import { Label, Button } from 'flowbite-svelte';
	import { ArrowLeftOutline, ArrowRightOutline } from 'flowbite-svelte-icons';
	import { next_view, prev_view } from '../utils';
	import type { Param } from '../utils';

	let params = get_params();
	async function get_params():Promise<{model_name: string, params: Param[]}> {
		let params:{model_name: string, params: Param[]} = await invoke('get_params');
		return params
	}

	async function set_params() {
		let selected_params = await params.then((result)=>result.params);
		invoke('set_params', {selected_params: selected_params});
	}

	async function reset_params() {
        await invoke('reset_params');
        params = get_params();
    }

</script>

<section>
	<br/>
	{#await params}
		<h1 style="text-decoration: underline;">Set parameters for...</h1>
	{:then result}
		<h1 style="text-decoration: underline;">Set parameters for {result.model_name} model:</h1>
	{:catch}
		<h1 style="text-decoration: underline;">Error loading parameters...</h1>
	{/await}
	<br/>
	<br/>
	{#await params}
		<h1 style="text-decoration: underline;">Waiting for parameters to load...</h1>
	{:then result}
		{#each result.params as param}
			<Label>{param.name}</Label>
			<div id="range">
				<div class="slidecontainer">
					<input type="range" min={param.min} max={param.max} step={param.increment} bind:value={param.value} class="slider bg-gray-200 dark:bg-gray-700 appearance-none" id="myRange">
				</div>
			</div>
			<Label>Value: {Math.round((param.value + Number.EPSILON) * 100) / 100}</Label>
			<br/>
		{/each}
	{:catch}
		<h1 style="text-decoration: underline;">Error loading parameters...</h1>
	{/await}

	<br/>
    <Button on:click={()=>reset_params()} style="position: absolute; font-size: small; height: 20px; width: 64px; left: calc(50% - 32px);" class="bg-teal-300 text-black dark:bg-gray-700 dark:text-white">Reset</Button>

		<div id="prev" class="dark:bg-gray-800 dark:text-white">
			<Button on:click={()=>prev_view()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
				<ArrowLeftOutline class="w-3.5 h-3.5 mr-2" />Previous
			</Button>
		</div>

		<div id="next" class="dark:bg-gray-800 dark:text-white">
			<Button on:click={()=>next_view()} on:click={()=>set_params()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
				Next<ArrowRightOutline class="w-3.5 h-3.5 ml-2" />
			</Button>
		</div>
</section>

<style>
.slidecontainer {
  width: 50%; /* Width of the outside container */
}

/* The slider itself */
.slider {
  -webkit-appearance: none;  /* Override default CSS styles */
  appearance: none;
  width: 100%; /* Full-width */
  height: 8px; /* Specified height */
  border-radius: 25px;
  outline: none; /* Remove outline */
  opacity: 0.7; /* Set transparency (for mouse-over effects on hover) */
  -webkit-transition: .2s; /* 0.2 seconds transition on hover */
  transition: opacity .2s;
}

/* Mouse-over effects */
.slider:hover {
  opacity: 1; /* Fully shown on mouse-over */
}

/* The slider handle (use -webkit- (Chrome, Opera, Safari, Edge) and -moz- (Firefox) to override default look) */
.slider::-webkit-slider-thumb {
  -webkit-appearance: none; /* Override default look */
  appearance: none;
  width: 20px; /* Set a specific slider handle width */
  height: 20px; /* Slider handle height */
  background: var(--primary_color); /* Green background */
  cursor: pointer; /* Cursor on hover */
}

.slider::-moz-range-thumb {
  width: 20px; /* Set a specific slider handle width */
  height: 20px; /* Slider handle height */
  background: var(--primary_color); /* Green background */
  cursor: pointer; /* Cursor on hover */
}
</style>