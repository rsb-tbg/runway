<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri'
	import { Button, Dropdown, DropdownItem, Select } from 'flowbite-svelte';
	import { ArrowRightOutline, ChevronDownSolid } from 'flowbite-svelte-icons';
	import { page } from '$app/stores';
	$: activeUrl = $page.url.pathname;
	import { next_view } from '../utils';
	
	let button_text:string = "ML Categories";
	$: open = false;
	let disabled:boolean = true;

	let categories = get_categories();
	async function get_categories():Promise<{categories: string[]}> {
		let categories:{categories: string[]} = await invoke('get_categories');
		return categories
	};

	function select_category(selection:string) {
		button_text = selection
		open = false
		disabled = false
		invoke('select_category', { selected_category: selection });
	};

</script>

<section>	
	<br/>
	<h1 style="text-decoration: underline;">Select a Machine Learning category:</h1>
	<br/>
	<Button id="dropdown_btn" on:click={() => open = true} class="text-black bg-teal-300 dark:text-white dark:bg-gray-700 font-medium">{button_text}<ChevronDownSolid class="w-3 h-3 ml-2" /></Button>
	{#if open}
		<Dropdown {open} {activeUrl} class="bg-teal-300 dark:bg-gray-700 rounded-lg">
			{#await categories}
				<DropdownItem id="dropdown_item" class="hover:bg-teal-300 hover:font-semibold">Waiting for categories to load...</DropdownItem>
			{:then result}
				{#each result.categories as _, i}
					<DropdownItem on:click={()=>select_category(result.categories[i])} id="dropdown_item" class="hover:bg-teal-300 hover:font-semibold">{result.categories[i]}</DropdownItem>
				{/each}
			{:catch error}
				<DropdownItem id="dropdown_item" class="hover:bg-teal-300 hover:font-semibold">Error loading categories...</DropdownItem>
			{/await}
		</Dropdown>
	{/if}

	<div id="next" class="dark:bg-gray-800 dark:text-white">
		<Button {disabled} on:click={()=>next_view()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
			Next<ArrowRightOutline class="w-3.5 h-3.5 ml-2" />
		</Button>
	</div>
</section>