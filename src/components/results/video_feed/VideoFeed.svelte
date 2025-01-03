<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import { appWindow } from "@tauri-apps/api/window";
	import { Button } from "flowbite-svelte";
	import { onDestroy, onMount } from "svelte";
	import { prev_view } from "../../../utils";
	import { ArrowLeftOutline } from "flowbite-svelte-icons";

  let v_f_err:string | void;
  onMount(async () => {
    v_f_err = await invoke('get_video_feed')
      .then((value) => {console.log(value)})
      .catch((error) => {return error as string})
  })

  onDestroy(() => {
    appWindow.emit("STOP_FRAMES", true );
  })

  let next_frame:string;
  appWindow.listen("NEXT_FRAME", ({payload}) => { next_frame = payload as string});

  document.getElementById("spinner")?.remove();

</script>

<section style="height: 100%; width: 100%;">
  <br/>
  {#if v_f_err != null}
    <h1 style="position: fixed; left: 40%;">{v_f_err}</h1>
  {/if}
  <br/>
  {#if next_frame != null}
    <!-- svelte-ignore a11y-missing-attribute -->
    <img style="display: block; margin-left: auto; margin-right: auto; width: 90%; height: calc(100% - 160px);" class="rounded-lg" src={next_frame}>
  {/if}
  <div id="prev" class="dark:bg-gray-800 dark:text-white">
    <Button id="prev_button" on:click={()=>prev_view()} class="w-28 bg-teal-300 text-black dark:bg-gray-700 dark:text-white">
    <ArrowLeftOutline class="w-3.5 h-3.5 mr-2" />Previous
    </Button>
  </div>
</section>