<script lang='ts'>
  import Error from "../error.svelte";
  let image: {filename: string};
  let value: string = "";
  let status: number;
  let doRegression: boolean = false;
  let dateFrom: string;
  let dateUntil: string;

  async function sendPlotQuery() {
    let requestBody = {
      fieldInput: (value ? value : "") + generateAppendable(),
      withRegression: doRegression,
    };
    let response = await fetch("api/plot", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(requestBody),
    });
    status = response.status;
    image = await response.json();
  };

  function generateAppendable(): string {
    let text = "";
    if(dateFrom) {
      text += " *:DATE:FROM:" + dateFrom;
    }
    if(dateUntil) {
      text += " *:DATE:UNTIL:" + dateUntil;
    }
    return text;
  }
</script>

<div class="inputfield">
  <p class="text">Input a query to request a plot</p>
  <input type="text" class="form" bind:value on:keydown={e => { if(e.key == "Enter") {sendPlotQuery()} } }>
  <br/>
  <input type="checkbox" id="regression" name="regression" class="regression" bind:checked={doRegression} />
  <label for="regression">With linear regression?</label>
  <div class="formtext">
    <span class="text">From: 
    <input type="date" class=dateform bind:value={dateFrom} /></span>
  </div>
  <div class="formtext">
    <span class="text">Until: 
    <input type="date" class=dateform bind:value={dateUntil} /></span>
  </div>
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
  .regression {
    margin-top: 10px;
    margin-bottom: 5px;
  }

  .dateform {
    background-color: #0C1618;
    border: 2px solid #D1AC00;
    text-align: center;
    color: #FAF4D3;
    font-weight: bold;
  }

  div {
    text-align: center;
  }

  .inputfield {
    background: linear-gradient(180deg, #285a58 0%, #004643 50%);
    border: 2px solid #D1AC00;
    padding: 20px;  }

  .formtext {
    color: #D1AC00;
    font-weight: bold;
    margin-top: 5px;
    margin-bottom: 5px;
  }

  .text {
    color: #D1AC00;
    font-weight: bold;
    margin-right: 5px;
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
