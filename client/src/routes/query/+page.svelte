<script lang='ts'>
  let datapoints: {timestamp: string, data: string, tags: string[]}[];
  let value: string;

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
    datapoints = await response.json();
  };

</script>

<div>
  <p>Input a query to retrieve your information</p>
  <input type="text" class="center" bind:value on:keydown={e => { if(e.key == "Enter") {sendQuery()} } }>
  <br/>
  <button on:click={ sendQuery } class="center">Send Query</button>
</div>

<div>
  {#if datapoints}
    {#each datapoints as datapoint}
      <div>
        <span class="timestamp">{datapoint.timestamp}</span>
        <span class="data">{datapoint.data}</span>
        {#each datapoint.tags as tag}
          <span class="tag">+{tag}&nbsp</span>
        {/each}
      </div>
    {/each}
  {/if}
</div>

<style>
  div {
    padding-top: 1em;
    text-align: center;
  }

  input {
    width: 25%;
  }

  .data {
    font-weight: bold;
  }

  .tag {
    font-style: italic;
  }
</style>