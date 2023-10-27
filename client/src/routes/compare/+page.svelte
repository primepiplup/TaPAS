<script lang='ts'>
  import Error from "../error.svelte";
  let image: {filename: string};
  let status: number;
	let inputs: string[] = [""];

  async function sendPlotQuery() {
    let requestBody = {
      fieldInputs: inputs ? inputs : [""],
    };
    let response = await fetch("api/comparison", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(requestBody),
    });
    status = response.status;
    image = await response.json();
  };

</script>

<div class="inputfield">
  <p class="text">Input a query to request a plot</p>
	{#each inputs as input}
	  <input type="text" class="form" bind:value={input} on:keydown={e => { if(e.key == "Enter") {sendPlotQuery()} } }>
		<br/>
	{/each}
	<button class="request" on:click={_ => inputs = [...inputs, ""]}>More</button>
	<button class="request" on:click={_ => inputs = inputs.slice(0, inputs.length - 1)}>Less</button>
  <br/>
  <button on:click={ sendPlotQuery } class="request">Send Query</button>
  <br/>
</div>

<div class="image">
  {#if image && status < 300}
   <img src={"/plot/" + image.filename} alt="cool plot" />
  {/if}
</div>

{#if status == undefined}
  <br/>
{:else if status == 200}
  <p class="text">Request handled succesfully.</p>
{:else if status >= 400}
  <Error errorText="Incorrect input was given."/>
{:else if status >= 500}
  <Error errorText="The server experienced an error." />
{:else}
  <Error errorText="Unknown error occurred." />
{/if}


<style>
  div {
    padding-top: 1em;
    text-align: center;
  }

  .inputfield {
    background: linear-gradient(180deg, #285a58 0%, #004643 50%);
    border: 2px solid #D1AC00;
    padding: 20px;  }

  .text {
    color: #D1AC00;
    font-weight: bold;
  }

  label {
    color: #D1AC00;
    font-style: italic;
  }

  .form {
    background-color: #0C1618;
    border: 2px solid #D1AC00;
    text-align: center;
    color: #FAF4D3;
    width: 25%;
    font-weight: bold;
  }

  .request {
    color: #D1AC00;
    background-color: #004643;
    border-top: 0px solid #D1AC00;
    border: 2px solid #D1AC00;
    font-weight: bold;
    padding: 5px;
  }

  .request:hover {
    color: #FAF4D3;
  }

</style>
