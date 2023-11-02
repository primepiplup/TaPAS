<script lang='ts'>
    import Error from "../error.svelte";

	export let datapoint: {timestamp: string, data: string, tags: string[], key: number};
	let deletionResult: {datastoreDeleted: boolean, databaseDeleted: boolean};
	let editing = false;
	let time = datapoint.timestamp.split(" ")[4];
	let date = getDateFromString(datapoint.timestamp);
	let data = datapoint.data + " " + datapoint.tags.map(tag => "+" + tag).join(" ");
	let key = datapoint.key;
	let status: number;

	function getDatapointValues() {
		time = datapoint.timestamp.split(" ")[4];
		date = getDateFromString(datapoint.timestamp);
		data = datapoint.data + " " + datapoint.tags.map(tag => "+" + tag).join(" ");
		key = datapoint.key;
	}

	function getDateFromString(timestamp: string): string {
		let day = timestamp.split(" ")[1];
		let month_word = timestamp.split(" ")[2];
		let year = timestamp.split(" ")[3];
		let month = "";
		switch(month_word) {
			case "Jan":
				month = "01"
				break;
			case "Feb":
				month = "02"
				break;
			case "Mar":
				month = "03"
				break;
			case "Apr":
				month = "04"
				break;
			case "May":
				month = "05"
				break;
			case "Jan":
				month = "06"
				break;
			case "Jul":
				month = "07"
				break;
			case "Aug":
				month = "08"
				break;
			case "Sep":
				month = "09"
				break;
			case "Oct":
				month = "10"
				break;
			case "Nov":
				month = "11"
				break;
			case "Dec":
				month = "12"
				break;
			default:
				month = "what"
		}
		if(day.length == 1) {
			day = "0" + day;
		}
		let date = year + "-" + month + "-" + day;
		return date;
	}	

	function switchMode() {
		editing = !editing;
		getDatapointValues();
	} 

	async function updateDatapoint() {
		let updated: string = data + toDateTag(date) + toTimeTag(time);
    let requestBody = {
      fieldInput: updated,
			key,
    };
    let response = await fetch("api/update", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(requestBody),
    });
    status = response.status;
		datapoint = await response.json();
		switchMode();
		getDatapointValues();
	}

	async function deleteDatapoint() {
    let requestBody = {
			value: key,
    };
    let response = await fetch("api/delete", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(requestBody),
    });
    status = response.status;
		deletionResult = await response.json();
		switchMode();
	}

	function toDateTag(date: string): string {
		return " +DATE:"+date;
	}

	function toTimeTag(time: string): string {
		return " +TIME:"+time.replaceAll(":", "-");
	}
</script>

{#if deletionResult}
	<div class="editbox">
		{#if deletionResult.databaseDeleted}
			<span>Successfully removed from database.</span>
		{:else}
			<span>Removal from database unsuccessful.</span>
		{/if}
		{#if deletionResult.datastoreDeleted}
			<span>Successfully removed from datastore.</span>
		{:else}
			<span>Removal from datastore unsuccessful.</span>
		{/if}
	</div>
{:else if editing}
	<div class="editbox">
		<input type="text" bind:value={data} class="dataentry"/>
		<br/>
		<input type="date" bind:value={date} />
		<input type="time" bind:value={time} />
		<br/>
		<button class="deletebutton" on:click={deleteDatapoint}>Delete</button>
		<button class="editbutton" on:click={updateDatapoint}>Submit</button>
		<button class="editbutton" on:click={switchMode}>Cancel</button>
	</div>
{:else}
	<div class="result">
	  <span class="timestamp">{datapoint.timestamp}</span>
	  <span class="data">{datapoint.data}</span>
		<span class="tags">
	  {#each datapoint.tags as tag}
	    <span class="tag">+{tag}&nbsp</span>
	  {/each}
		</span>
		<button class="editbutton" on:click={switchMode}>Edit</button>
	</div>
{/if}


<style>
	input {
	  background-color: #0C1618;
    border: 2px solid #D1AC00;
    text-align: center;
    color: #FAF4D3;
    font-weight: bold;
	}

	.dataentry {
		width: 80%;
	}

	.deletebutton {
		margin-left: 10px;
		color: #D1AC00;
    background-color: #C1292E;
    border-top: 0px solid #D1AC00;
    border: 2px solid #D1AC00;
    font-weight: bold;
	}
	
	.editbutton {
		margin-left: 10px;
		color: #D1AC00;
    background-color: #004643;
    border-top: 0px solid #D1AC00;
    border: 2px solid #D1AC00;
    font-weight: bold;
	}

	.editbutton:hover {
		background-color: #002624;
	}

	.editbox {
		background-color: #004643;
    border: 2px solid #D1AC00;
    padding: 20px;
		margin-bottom: 10px;
		color: #D1AC00;
		width: 100%;
	}
	
	.result {
    background-color: #004643;
    border: 2px solid #D1AC00;
    padding: 20px;
		margin-bottom: 10px;
		color: #D1AC00;
		width: 100%;
		display: flex;
		justify-content:left;
		flex: 0 0 1;
	}

	.timestamp {
		justify-self: left;
		font-style: italic;
		margin-right: 30px;
	}

	.data {
		justify-self: left;
    color: #D1AC00;
		font-weight: bold;
	}

	.tags {
		margin-left: auto;
		justify-self: right;
	}

	.tag {
		font-style: italic;
	}
</style>
