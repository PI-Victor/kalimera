<script lang="ts">
	export let location: string;
</script>

<div class="uk-flex uk-flex-between">
	<h6 class="uk-text-medium uk-text-capitalize">{location}</h6>
	<div>
		<form class="uk-search uk-search-default">
			<span class="uk-search-icon-flip" uk-search-icon />
			<input
				class="uk-search-input uk-height-small small-input"
				type="search"
				placeholder="Search"
				aria-label=""
			/>
		</form>
	</div>
</div>
<hr />
<div class="uk-flex uk-flex-between">
	<div class="uk-text-primary">
		{#if location == 'objects'}
			<a
				href="/"
				uk-icon="icon: arrow-left"
				class="uk-text-primary uk-margin-medium-right"
				uk-tooltip="Back"
			/>
		{/if}
		{#if location == 'buckets'}
			<a href="#" uk-icon="icon: plus" class="uk-margin-small-right" uk-tooltip="New Bucket" />
		{/if}
		{#if location == 'objects'}
			<a href="#" uk-icon="icon: upload" class="uk-margin-small-right" uk-tooltip="Upload" />
			<a href="#" uk-icon="icon: download" class="uk-margin-small-right" uk-tooltip="Download" />
		{/if}
		<a href="#" uk-icon="icon: refresh" class="uk-margin-small-right" uk-tooltip="Refresh" />
	</div>
	<div>
		<a href="#" uk-icon="icon: trash" class="uk-text-danger" uk-tooltip="Delete" />
	</div>
</div>
<hr class="uk-divider-icon uk-margin-remove" />

<style>
	.small-input {
		height: 30px; /* Adjust this value as needed */
		font-size: small;
	}
</style>
