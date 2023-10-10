<script lang='ts'>
  let information: string;
  let value: string;

  async function sendInput() {
    let requestBody = {
      fieldInput: value
    };
    let response = await fetch("api/input", {
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
  <input type="text" class="center" bind:value on:keydown={e => { if(e.key == "Enter") {sendInput()} } }>
  <br/>
  <button on:click={ sendInput } class="center">Send Input</button>
</div>

{#if information}
  <p>{information}</p>
{/if}

<style>
  div {
    padding-top: 1em;
    text-align: center;
  }
</style>