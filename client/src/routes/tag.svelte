<script lang='ts'>
	import { tagList } from "./stores.js";
	import { onMount } from "svelte";
	export let tag_text: string;
	let set: boolean = false;

	function addTag() {
		tagList.update((tags) => {
			if(!tags.includes(tag_text)) {
	      tags.push(tag_text);
	    }
			return tags;
		});
  };

  function removeTag() {
    tagList.update((tags) => {
			let tagIndex = tags.indexOf(tag_text);
	    if(tagIndex > -1) {
	      tags.splice(tagIndex, 1);
	    }
			return tags;
		});
  }

	onMount(() => {
		set = $tagList.includes(tag_text);
	});
</script>

{#if set}
<button class="set" on:click={ _ => { set = false; removeTag(); } }>{tag_text}</button>
{:else}
<button class="unset" on:click={ _ => { set = true; addTag(); } }>{tag_text}</button>
{/if}

<style>
	button {
		border: 2px solid #D1AC00;
		border-radius: 30px;
		padding: 10px;
		margin: 5px;
		color: #FAF4D3;
		font-weight: bold;
	}

	button.set {
		background-color: #0C1618;
		color: #C1292E;
	}

	button.unset {
		background-color: #004643;
	}
</style>
