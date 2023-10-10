<script lang='ts'>
  let information: string;
  let value: string;

  async function sendQuery() {
    let requestBody = {
      fieldInput: value
    };
    let response = await fetch("api/query", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(requestBody),
    });
    information = await response.text();
  };

</script>

<div>
  <p>Input a query to retrieve your information</p>
  <input type="text" class="center" bind:value on:keydown={e => { if(e.key == "Enter") {sendQuery()} } }>
  <br/>
  <button on:click={ sendQuery } class="center">Send Query</button>
</div>

{#if information}
  <p>{information}</p>
{/if}

<style>
  div {
    padding-top: 1em;
    text-align: center;
  }

  input {
    width: 25%;
  }
</style>