<script lang="ts">
	export let text: string;
	export let onEnter: () => Promise<void>;

	let extraOptions: boolean = false;
	let input: string;
	let dateFrom: string;
	let dateUntil: string;

	$: extra = parseDates(dateFrom, dateUntil);
	$: text = input + extra;

	function switchOptionState() {
		extraOptions = !extraOptions;
	}

	function parseDates(dateFrom: string, dateUntil: string): string {
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

<input type="text" class="form" bind:value={input} on:keydown={e => { if(e.key == "Enter") {onEnter} } }>
<button on:click={switchOptionState}>+</button>
{#if extraOptions}
	<div>
    <div class="formtext">
      <span class="text">From: 
      <input type="date" class=dateform bind:value={dateFrom} /></span>
    </div>
    <div class="formtext">
      <span class="text">Until: 
      <input type="date" class=dateform bind:value={dateUntil} /></span>
    </div>	
	</div>
{/if}

<style>
	.form {
    background-color: #0C1618;
    border: 2px solid #D1AC00;
    text-align: center;
    color: #FAF4D3;
    width: 25%;
    font-weight: bold;
  }	

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

	.dateform {
    background-color: #0C1618;
    border: 2px solid #D1AC00;
    text-align: center;
    color: #FAF4D3;
    font-weight: bold;
  }

	button {
    color: #D1AC00;
    background-color: #004643;
    border-top: 0px solid #D1AC00;
    border: 2px solid #D1AC00;
    font-weight: bold;
  }

  button:hover {
    color: #FAF4D3;
  }
</style>
