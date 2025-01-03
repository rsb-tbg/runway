<script lang="ts">
    import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";
    import { Button } from 'flowbite-svelte';
	import { ArrowLeftOutline } from "flowbite-svelte-icons";
	import { prev_view } from "../../../utils";

    let img_cont_max_height:string = "calc(100% - 158px";
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

    let detected_faces:Promise<{detected_faces: string[]}> = get_detected_faces();
    async function get_detected_faces():Promise<{detected_faces: string[]}> {
        return await invoke('get_detected_faces') as {detected_faces: string[]};
    }

    $: if (detected_faces != null) {
        document.getElementById("spinner")?.remove();
        disabled = false;
    }

    function select_face(selected_face:string) {
        invoke('select_face', {selected_face: selected_face})
    }
 
</script>

<svelte:window on:resize={()=>adjust_columns()}/>
<section style="height: 100%;">
    <div style="height: 90px">
        <br/>
        <h1 style="text-decoration: underline;">Which face would you like to submit for comparison?</h1>
    </div>
    <div style="max-height: {img_cont_max_height}; overflow-y: auto;" id="img_container" class="grid gap-4 dark:[color-scheme:dark]">
        {#await detected_faces}
            <h1>waiting for detected faces</h1>
        {:then result}
                {#each result.detected_faces as face}
                    <div class="rounded-lg" style="position:relative; height: 192px; width: 128px; font-size: 12px; text-align: center;">
                        <!-- svelte-ignore a11y-click-events-have-key-events -->
                        <!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
                        <img id="hover_img" on:click={()=>select_face(face)} class="rounded-lg" style="height: 184px; width: 100%; cursor: pointer;" src={face} alt="" loading="lazy">
                    </div> 
                {/each}
        {:catch error}
            <h1>{error}</h1>
        {/await}
    </div>
    <div id="prev" class="dark:bg-gray-800 dark:text-white">
		<Button {disabled} id="prev_button" on:click={()=>prev_view()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
			<ArrowLeftOutline class="w-3.5 h-3.5 mr-2" />Previous
		</Button>
	</div>
</section>

<style>
    #hover_img:hover {
        border: solid transparent;
    }
</style>