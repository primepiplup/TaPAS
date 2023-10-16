<script lang='ts'>
  let image: {filename: string};
  let value: string;

  async function sendPlotQuery() {
    let requestBody = {
      fieldInput: value ? value : ""
    };
    let response = await fetch("api/plot", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(requestBody),
    });
    image = await response.json();
  };

</script>

<div>
  <p>Input a query to request a plot</p>
  <input type="text" class="center" bind:value on:keydown={e => { if(e.key == "Enter") {sendPlotQuery()} } }>
  <br/>
  <button on:click={ sendPlotQuery } class="center">Send Query</button>
</div>

<div>
  {#if image}
   <img src={"/plot/" + image.filename} alt="cool plot" />
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