<script lang="ts">
	import '../app.postcss';
	import '../app.css'
	import { DarkMode } from 'flowbite-svelte';
	import { Navbar, NavBrand } from 'flowbite-svelte';
	import { appWindow } from "@tauri-apps/api/window";
	import { invoke } from '@tauri-apps/api/tauri';
	import { ComponentType, onMount, type SvelteComponent } from 'svelte';
	import Sidebar from '../components/Sidebar.svelte';
	import SelectCategory from '../components/SelectCategory.svelte';
	import SelectModel from '../components/SelectModel.svelte';
	import SetParameters from '../components/SetParameters.svelte';
	import SelectPath from '../components/SelectPath.svelte';
	import ConfirmSelections from '../components/ConfirmSelections.svelte';
	import Results from '../components/results/Results.svelte';
	import DBcount from '../components/DBcount.svelte';
	import SetResultThresholds from '../components/SetResultThresholds.svelte';
	import { goto } from '$app/navigation';

	onMount(()=>{
		invoke('reset_app');
	})

	let current_view_map: Map<string, ComponentType> = new Map([
		["SelectCategory", SelectCategory],
		["SelectModel", SelectModel],
		["SetParameters", SetParameters],
		["SelectPath", SelectPath],
		["SetResultThresholds", SetResultThresholds],
		["ConfirmSelections", ConfirmSelections],
		["Results", Results]
	]);
	let current_view: ComponentType  = SelectCategory;

	appWindow.listen("CURRENT_VIEW", ({ payload }) => { current_view = current_view_map.get(payload as string) as typeof SvelteComponent});
</script>

<!-- svelte-ignore a11y-no-static-element-interactions -->
<!-- <div id="app" on:contextmenu|preventDefault> -->
<div id="app">
	<div id="top">
		<Navbar fluid class="bg-slate-100 dark:bg-gray-900">
			<NavBrand>
				<button on:click={()=>{
					invoke('reset_app');
					current_view = SelectCategory;
				}}>
					<svg class="mr-2 h-6 sm:h-9 fill-teal-600" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0,0,256,256" width="50px" height="50px"><g fill-rule="nonzero" stroke="none" stroke-width="1" stroke-linecap="butt" stroke-linejoin="miter" stroke-miterlimit="10" stroke-dasharray="" stroke-dashoffset="0" font-family="none" font-weight="none" font-size="none" text-anchor="none" style="mix-blend-mode: normal"><g transform="scale(5.12,5.12)"><path d="M43.937,19.352c1.818,-4.849 0.685,-8.053 -0.587,-9.888c-1.996,-2.879 -5.682,-4.464 -10.38,-4.464h-17.97c-0.404,0 -0.77,0.243 -0.924,0.617c-0.155,0.374 -0.069,0.804 0.217,1.09l3.588,3.588l-7.762,28.138l-5.76,4.799c-0.323,0.27 -0.443,0.713 -0.3,1.109c0.144,0.395 0.52,0.659 0.941,0.659h16c0.332,0 0.642,-0.165 0.828,-0.439c0.186,-0.274 0.224,-0.624 0.101,-0.932l-1.89,-4.725l1.78,-8.904h4.412l3.803,14.258c0.116,0.437 0.513,0.742 0.966,0.742h10c0.404,0 0.77,-0.243 0.924,-0.617c0.155,-0.374 0.069,-0.804 -0.217,-1.09l-3.813,-3.813l-2.692,-9.873c5.782,-2.538 8.611,-9.927 8.735,-10.255zM26.778,11.915c1.521,-0.107 5.486,0.002 7.316,3.908c1.43,3.052 -0.81,6.935 -1.621,8.177h-9.147z"></path></g></g></svg>
				</button>
				<span class="self-center whitespace-nowrap text-xl font-semibold text-teal-600">Runway</span>
			</NavBrand>
			<DarkMode></DarkMode>
		</Navbar>
	</div>

	<div id="middle">
		<Sidebar/>
		{#if current_view != SelectCategory}
			<DBcount/>
		{/if}
		<main id="main" class="dark:bg-gray-800 dark:text-white">
			<svelte:component this={current_view}/>
		</main>
	</div>

	<div id="bottom" class="cursor-default w-full bg-slate-100 p-2.5 text-center font-semibold text-teal-600 dark:bg-gray-900"></div>
</div>

<style>

</style>