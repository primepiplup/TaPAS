<script lang='ts'>
	export let datapoint: {timestamp: string, data: string, tags: string[]}; 
</script>

<div class="result">
  <span class="timestamp">{datapoint.timestamp}</span>
  <span class="data">{datapoint.data}</span>
	<span class="tags">
  {#each datapoint.tags as tag}
    <span class="tag">+{tag}&nbsp</span>
  {/each}
	</span>
</div>

<style>
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
