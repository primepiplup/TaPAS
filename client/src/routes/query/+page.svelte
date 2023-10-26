<script lang='ts'>
  import Result from "./result.svelte";
  import Error from "../error.svelte";
  let datapoints: {timestamp: string, data: string, tags: string[], key: number}[];
  let value: string = "";
  let status: number;

  async function sendQuery() {
    let requestBody = {
      fieldInput: value ? value : ""
    };
    let response = await fetch("api/query", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(requestBody),
    });
    status = response.status;
    datapoints = await response.json();
  };

</script>

<div class="container">
  <div class="inputfield">
    <p class="text">Input a query to retrieve your information</p>
    <input type="text" class="form" bind:value on:keydown={e => { if(e.key == "Enter") {sendQuery()} } }>
    <br/>
    <button on:click={ sendQuery } class="request">Send Query</button>
  </div>

  <div>
    {#if datapoints}
      {#each datapoints as datapoint}
        <Result datapoint={datapoint} />
      {/each}
    {:else}
      <p class="field">No queries performed.</p>
    {/if}
  </div>
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
  .container {
    display: flex;
    justify-content: space-evenly;
    padding-top: 1em;
  }

  div {
    text-align: center;
  }

  input {
    width: 100%;
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

  .form {
    background-color: #0C1618;
    border: 2px solid #D1AC00;
    text-align: center;
    color: #FAF4D3;
    font-weight: bold;
  }
  .inputfield {
    background: linear-gradient(180deg, #285a58 0%, #004643 50%);
    border: 2px solid #D1AC00;
    padding: 20px;
    width: 40%;
    flex: 0 0 50%;
  }

  .field {
    background: linear-gradient(180deg, #285a58 0%, #004643 50%);
    border: 2px solid #D1AC00;
    padding: 20px;
    width: 40%;
    flex: 0 0 1;
    color: #D1AC00;
    font-weight: bold;
  }
  

  .text {
    color: #D1AC00;
    font-weight: bold;
  }
</style>
