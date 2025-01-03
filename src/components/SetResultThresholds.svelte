<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	import { Label, Button } from 'flowbite-svelte';
	import { ArrowLeftOutline, ArrowRightOutline } from 'flowbite-svelte-icons';
	import { next_view, prev_view } from '../utils';
	import type { Threshold } from '../utils';

	$: result_thresholds = get_result_thresholds();
	async function get_result_thresholds():Promise<{result_thresholds: Threshold[]}> {
		let result_thresholds:{result_thresholds: Threshold[]} = await invoke('get_result_thresholds');
		return result_thresholds
	}

	async function set_result_thresholds() {
		let selected_result_thresholds = await result_thresholds.then((result)=>result.result_thresholds);
		invoke('set_result_thresholds', {selected_result_thresholds: selected_result_thresholds});
	}

    async function reset_result_thresholds() {
        await invoke('reset_result_thresholds');
        result_thresholds = get_result_thresholds();
    }

</script>

<section>
	<br/>
	    <h1 style="text-decoration: underline;">Set thresholds for results:</h1>
	<br/>
	<br/>
	{#await result_thresholds}
		<h1 style="text-decoration: underline;">Waiting for result thresholds to load...</h1>
	{:then result}
		{#each result.result_thresholds as threshold}
			<Label>{threshold.name}</Label>
			<div id="range">
				<div class="slidecontainer">
					<input type="range" min={threshold.min} max={threshold.max} step={threshold.increment} bind:value={threshold.value} class="slider bg-gray-200 dark:bg-gray-700 appearance-none" id="myRange">
				</div>
			</div>
            <Label>Include results &nbsp {threshold.include_results} &nbsp {Math.round((threshold.value + Number.EPSILON) * 100) / 100}</Label>
			<br/>
		{/each}
	{:catch}
		<h1 style="text-decoration: underline;">Error loading result thresholds...</h1>
	{/await}

    <br/>
    <Button on:click={()=>reset_result_thresholds()} style="position: absolute; font-size: small; height: 20px; width: 64px; left: calc(50% - 32px);" class="bg-teal-300 text-black dark:bg-gray-700 dark:text-white">Reset</Button>

		<div id="prev" class="dark:bg-gray-800 dark:text-white">
			<Button on:click={()=>prev_view()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
				<ArrowLeftOutline class="w-3.5 h-3.5 mr-2" />Previous
			</Button>
		</div>

		<div id="next" class="dark:bg-gray-800 dark:text-white">
			<Button on:click={()=>next_view()} on:click={()=>set_result_thresholds()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
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