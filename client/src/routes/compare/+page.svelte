<script lang='ts'>
  import Error from "../error.svelte";
  import Summary from "./summary.svelte";
  import Inputfield from "./inputfield.svelte";
  let comparison_result: {filename: string, summaries: {name: string, mean: number, p: number}[]};
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
    comparison_result = await response.json();
  };

</script>

<div class="inputfield">
  <p class="text">Input a query to request a plot</p>
	{#each inputs as input}
    <Inputfield bind:text={input} onEnter={sendPlotQuery} /> 
		<br/>
	{/each}
	<button class="request" on:click={_ => inputs = [...inputs, ""]}>More</button>
	<button class="request" on:click={_ => inputs = inputs.slice(0, inputs.length - 1)}>Less</button>
  <br/>
  <button on:click={ sendPlotQuery } class="request">Send Query</button>
  <br/>
</div>

{#if comparison_result}
<div class="results">
  <table class="summary-table">
    <thead>
      <tr class="header">
        <th colspan=2>Statistical Summary</th>
      </tr>
      <tr class="column-names">
        <th>Group</th>
        <th>Mean</th>
        <th>p-value</th>
      </tr>
    </thead>
    <tbody>
        {#each comparison_result.summaries as summary}
          <Summary summary={summary} />
        {/each}
    </tbody>
  </table>
</div>
{/if}

<div class="image">
  {#if comparison_result && status < 300}
   <img src={"/plot/" + comparison_result.filename} alt="cool plot" />
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
  .header {
    font-weight: bold;
  }

  .column-names {
    font-weight: normal;
  }

  .results {
    align-content: center;
  }
  
  .summary-table {
    color: #D1AC00;
    margin-left: auto;
    margin-right: auto;
    text-align: left;
  }
  
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
