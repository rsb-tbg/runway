<script lang="ts">
	import { Button, Dropdown, DropdownItem } from 'flowbite-svelte';
	import { ArrowLeftOutline, ArrowRightOutline, ChevronDownSolid } from 'flowbite-svelte-icons';
	import { page } from '$app/stores';
	$: activeUrl = $page.url.pathname;
	import { next_view, prev_view } from '../utils';
	import { invoke } from '@tauri-apps/api/tauri';

	let button_text:string;
	$: open = false;
	let disabled:boolean = true;

	let models = get_models();
	async function get_models():Promise<{category: string, models: string[]}> {
		let models:{category: string, models: string[]} = await invoke('get_models');
		button_text = models.category + " models"
		return models
	}

	function select_model(selection:string) {
		button_text = selection
		open = false
		disabled = false
		invoke('select_model', { selected_model: selection });
	}

</script>

<section>
	<br/>
	{#await models}
		<h1 style="text-decoration: underline;">Select a...</h1>
	{:then result}
		<h1 style="text-decoration: underline;">Select a {result.category} model:</h1>
	{:catch}
		<h1 style="text-decoration: underline;">Error loading category...</h1>
	{/await}
	<br/>
	<Button id="dropdown_btn" on:click={() => open = true} class="text-black bg-teal-300 dark:text-white dark:bg-gray-700 font-medium">{button_text}<ChevronDownSolid class="w-3 h-3 ml-2" /></Button>
	{#if open}
		<Dropdown {open} {activeUrl} class="bg-teal-300 dark:bg-gray-700 rounded-lg">
			{#await models}
				<DropdownItem id="dropdown_item" class="hover:bg-teal-300 hover:font-semibold">Waiting for models to load...</DropdownItem>
			{:then result}
				{#each result.models as _, i}
					<DropdownItem on:click={()=>select_model(result.models[i])} id="dropdown_item" class="hover:bg-teal-300 hover:font-semibold">{result.models[i]}</DropdownItem>
				{/each}
			{:catch error}
				<DropdownItem id="dropdown_item" class="hover:bg-teal-300 hover:font-semibold">Error loading models...</DropdownItem>
			{/await}
		</Dropdown>
	{/if}

	<div id="prev" class="dark:bg-gray-800 dark:text-white">
		<Button on:click={()=>prev_view()}  class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
			<ArrowLeftOutline class="w-3.5 h-3.5 mr-2" />Previous
		</Button>
	</div>

	<div id="next" class="dark:bg-gray-800 dark:text-white">
		<Button {disabled} on:click={()=>next_view()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
			Next<ArrowRightOutline class="w-3.5 h-3.5 ml-2" />
		</Button>
	</div>
</section>