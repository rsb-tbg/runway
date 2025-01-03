<script lang="ts">
    import { invoke } from '@tauri-apps/api/tauri'
	import { appWindow } from '@tauri-apps/api/window';


    $: db_count_message = get_db_count();
    appWindow.listen("DB_COUNT", ({ payload }) => { db_count_message = payload as Promise<{ db_type: string, db_count: number; }>});
	async function get_db_count() {
		const res:{db_type: string, db_count: number} = await invoke('get_db_count');
		if (res != null) {
			return res;
		} else {
			throw new Error("error");
		}
    }
</script>

<div id="db_count" class="text-sm cursor-default text-black duration-75 dark:text-white" style="position: absolute; bottom: 68px; left: 20px;">
    {#await db_count_message}
        <h1>db Face count: waiting...</h1>
    {:then result} 
        <h1>{result.db_type} in DB: <span style="text-decoration:underline">{result.db_count.toLocaleString()}</span></h1>
    {/await}
</div>