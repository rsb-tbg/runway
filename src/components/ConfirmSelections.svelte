<script lang="ts">
	import { Button } from "flowbite-svelte";
	import { ArrowLeftOutline, PlaySolid } from "flowbite-svelte-icons";
	import { invoke } from "@tauri-apps/api/tauri";
	import { next_view, prev_view } from "../utils";
    import type { Param, Threshold } from "../utils";
	import { onMount } from "svelte";

    onMount(()=>{set_parse_names()})

    type Selections = {
        models: {
            category: string,
            models: string[]
        },
        params: {
            model_name: string,
            params: Param[]
        },
        selected_path: string,
        path_type: string,
        result_thresholds: Threshold[],
    };

    let selections = get_selections();
    async function get_selections():Promise<Selections> {
		let models:{category: string, models: string[]} = await invoke('get_models');
		let params:{model_name: string, params: Param[]} = await invoke('get_params');
		let selected_path:{path: string} = await invoke('get_path');
        let path_type:{path_type: string} = await invoke('get_path_type');
        let result_thresholds:{result_thresholds: Threshold[]} = await invoke('get_result_thresholds');
        let selections:Selections = {models: models, params: params, selected_path: selected_path.path, path_type: path_type.path_type, result_thresholds: result_thresholds.result_thresholds};
        return selections
    }

    function transform_params(params:Param[]):string {
        let params_display:string = "";
        params.forEach((param,index,array)  => {
            if (index + 1 == array.length) {
                params_display = params_display + param.name + " [" + (Math.round((param.value + Number.EPSILON) * 100) / 100).toString() + "]"
            } else {
                params_display = params_display + param.name + " [" + (Math.round((param.value + Number.EPSILON) * 100) / 100).toString() + "], "
            }
        });
        return params_display
    }

    function transform_thresholds(thresholds:Threshold[]):string {
        let thresholds_display:string = "";
        thresholds.forEach((threshold,index,array) => {
            if (index + 1 == array.length) {
                thresholds_display = thresholds_display + threshold.name + " [" + (Math.round((threshold.value + Number.EPSILON) * 100) / 100).toString() + "]"
            } else {
                thresholds_display = thresholds_display + threshold.name + " [" + (Math.round((threshold.value + Number.EPSILON) * 100) / 100).toString() + "], "
            }
        });
        return thresholds_display
    }

    let checked = true;
    function set_parse_names() {
        if (checked) {
            checked = false;
        } else {
            checked = true;
        }
        invoke('set_parse_names', {parse_names: checked})
    }

</script>

<section>
    <br/>
    <h1 style="text-decoration:underline">Confirm selections:</h1>
    <br/>

    {#await selections}
        <h1>awaiting....</h1>
    {:then result}
        <table>
            <tr>
                <td>ML Category</td>
                <td class="value">{result.models.category}</td>
            </tr>
            <tr>
                <td>Model</td>
                <td class="value">{result.params.model_name}</td>
            </tr>
            <tr>
                <td>Parameters</td>
                <td class="value">{transform_params(result.params.params)}</td>
            </tr>
            <tr>
                <td>Selected Data</td>
                <td class="value">{result.selected_path}</td>
            </tr>
            {#if result.path_type != "Directory"}
                <tr>
                    <td>Result Thresholds</td>
                    <td class="value">{transform_thresholds(result.result_thresholds)}</td>
                </tr>
            {/if}
        </table>
    {:catch error}
        <h1>{error}</h1>
    {/await}

    <br/>
    <label class="cursor-pointer relative inline-flex items-center">
        <input on:click={()=>set_parse_names()} checked={checked} type="checkbox" value="" class="sr-only peer">
        <div class="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-4 peer-focus:ring-teal-600 dark:peer-focus:ring-teal-600 rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full rtl:peer-checked:after:-translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:start-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-teal-600"></div>
        <span class="ms-3 text-sm font-medium text-gray-900 dark:text-white">Parse Names?</span>
    </label>

    <div id="prev" class="dark:bg-gray-800 dark:text-white">
        <Button on:click={()=>prev_view()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
            <ArrowLeftOutline class="w-3.5 h-3.5 mr-2" />Previous
        </Button>
    </div>

    <div id="next" class="dark:bg-gray-800 dark:text-white">
        <Button on:click={()=>next_view()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
            Execute<PlaySolid class="w-3.5 h-3.5 ml-2" />
        </Button>
    </div>
</section>

  <style>
    tr, td {
        padding: 20px;
    }

    .value {
        text-decoration: underline;
    }
  </style>