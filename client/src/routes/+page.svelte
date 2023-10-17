<script lang='ts'>
  import { onMount } from "svelte";
  import Tag from "./tag.svelte";
  import { tagList } from "./stores.js";
  import Error from "./error.svelte";

  let status: number;
  let value: string;
  let tags: { tag: string }[];
  let tagsToApply: string[];

  tagList.subscribe((tags) => {
    tagsToApply = tags;
  });

  async function sendInput() {
    let appendable = generateAppendable();
    
    let requestBody = {
      fieldInput: value + appendable,
    };
    let response = await fetch("api/input", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(requestBody),
    });
    status = response.status;
    value = "";
    getTags();
  };

  async function getTags() {
    let response = await fetch("api/tags");
    tags = await response.json();
  }

  function generateAppendable() {
    let text = "";
    for(let i = 0; i < tagsToApply.length; i++) {
      text += "+" + tagsToApply[i];
    }
    return text;
  }

  onMount(getTags);

</script>

<div class="inputfield">
  <p class="text">Please provide some input</p>
  <input type="text" class="form" bind:value on:keydown={e => { if(e.key == "Enter") {sendInput()} } }> 
  {#if tagsToApply}
    {#each tagsToApply as tagToApply}
      <span class="tag">+{tagToApply} </span>
    {/each}
  {/if}
  <br/>
  <button on:click={ sendInput } class="request">Send Input</button>
  <div class="tagcontainer">
    {#if tags}
      {#each tags as tag}
        <Tag tag_text={tag.tag}/>
      {/each}
    {/if}
  </div>
  </div>



{#if status == 200}
  <p class="text">Request handled succesfully.</p>
{/if}

{#if status >= 400}
  <Error errorText="Incorrect input was given."/>
{/if}

<style>
  div {
    padding-top: 1em;
    text-align: center;
  }

  input {
    width: 50%;
  }

  .tagcontainer {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .form {
    background-color: #0C1618;
    border: 2px solid #D1AC00;
    text-align: center;
    color: #FAF4D3;
    font-weight: bold;
  }

  .request {
    color: #D1AC00;
    background-color: #004643;
    border-top: 0px solid #D1AC00;
    border: 2px solid #D1AC00;
    font-weight: bold;
  }

  .request:hover {
    color: #FAF4D3;
  }

  .inputfield {
    background: linear-gradient(180deg, #285a58 0%, #004643 50%);
    border: 2px solid #D1AC00;
    padding: 20px;
  }

  .text {
    color: #D1AC00;
  }

  .tag {
    color: #D1AC00;
    font-style: italic;
  }

  .inputfield .text {
    font-weight: bold;
  }
     
</style>