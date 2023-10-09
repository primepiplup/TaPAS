<script lang='ts'>
  let information: string;

  async function handleClick() {
    let response = await fetch("api/test");
    information = await response.text();
  };
</script>

<button on:click={ handleClick }>Click me!</button>

{#if information}
  <p>{information}</p>
{/if}

<style>
  button {
    align-self: center;
  }
</style>