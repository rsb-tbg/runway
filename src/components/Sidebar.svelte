<script lang="ts">
    import { Sidebar, SidebarGroup, SidebarItem, SidebarWrapper } from 'flowbite-svelte';
    import { ArrowRightSolid } from 'flowbite-svelte-icons';
    import { invoke } from '@tauri-apps/api/tauri'
	import { appWindow } from '@tauri-apps/api/window';

    type sidebar_item = {
        name: string,
        assoc_view: any,
        disabled: boolean,
    }

    $: sidebar_items = app_load_sidebar_items();
    appWindow.listen("SIDEBAR_ITEMS", ({ payload }) => { sidebar_items = payload as Promise<{ items: sidebar_item[]; }>});
	async function app_load_sidebar_items() {
		const res:{items: sidebar_item[]} = await invoke('get_sidebar_items');
		if (res != null) {
			return res;
		} else {
			throw new Error("error");
		}
	}

</script>

<div id="sidebar" class="dark:bg-gray-800" style="width:220px; height:100%;">
    <Sidebar>
        <SidebarWrapper class="bg-transparent rounded-none" style="height:100%;">
            <SidebarGroup style="height:100%;">
                {#await sidebar_items}
                    <SidebarItem class="cursor-default text-black dark:text-white hover:bg-transparent dark:hover:bg-transparent" label="Loading steps...">
                        <svelte:fragment slot="icon">
                            <ArrowRightSolid class="w-5 h-5 text-gray-500 duration-75 dark:text-gray-400" />
                        </svelte:fragment>
                    </SidebarItem>
                {:then result}
                    {#each result.items as item}
                        {#if !item.disabled}
                            <SidebarItem class="text-black dark:text-white hover:bg-transparent dark:hover:bg-transparent" label={item.name}>
                                <svelte:fragment slot="icon">
                                    <ArrowRightSolid class="w-5 h-5 text-black duration-75 dark:text-gray-400" />
                                </svelte:fragment>
                            </SidebarItem>
                        {:else}
                            <SidebarItem class="cursor-default text-gray-400 dark:text-gray-500 hover:bg-transparent dark:hover:bg-transparent" label={item.name}>
                                <svelte:fragment slot="icon">
                                    <ArrowRightSolid class="w-5 h-5 text-gray-400 duration-75 dark:text-gray-400" />
                                </svelte:fragment>
                            </SidebarItem>
                        {/if}
                    {/each}
                {:catch error}
                    <SidebarItem class="cursor-default text-black dark:text-white hover:bg-transparent dark:hover:bg-transparent" label="Error while loading steps...">
                        <svelte:fragment slot="icon">
                            <ArrowRightSolid class="w-5 h-5 text-gray-500 duration-75 dark:text-gray-400" />
                        </svelte:fragment>
                    </SidebarItem>
                {/await}
            </SidebarGroup>
        </SidebarWrapper>
    </Sidebar>
</div>