<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	import type { StorageBucket } from '../types';
	import Toolbar from '../components/toolbar.svelte';
	import Loading from '../components/loading.svelte';

	let error = null;
	let buckets: StorageBucket[] = [];
	let selectedBuckets: StorageBucket[] = [];
	let profiles: any[] = [
		{
			name: 'Profile 1'
		},
		{
			name: 'Profile 2'
		},
		{
			name: 'Profile 3'
		}
	];

	const listBuckets = async () => {
		try {
			buckets = await invoke('list_buckets');
		} catch (e) {
			error = e;
			console.log(error);
		}
	};

	let loadBuckets = listBuckets();

	onMount(async () => {
		await listBuckets();
	});

	const convertToLocaleString = (timestamp: number) => {
		const date = new Date(timestamp * 1000);
		return date.toLocaleString();
	};

	const updateSelected = (bucket: StorageBucket) => {
		return (event: any) => {
			if (event.target.checked) {
				selectedBuckets.push(bucket);
			} else {
				selectedBuckets = selectedBuckets.filter((b) => b.name !== bucket.name);
			}
		};
	};
</script>

<div class="uk-position-top-right uk-margin-small-right uk-margin-small-top">
	<div class="uk-inline">
		<button
			class="uk-button uk-button-small uk-border-rounded
			 profile-button"
			type="button"
			uk-icon="icon: user"
		/>
		<div uk-dropdown="mode: click">
			<ul class="uk-nav uk-dropdown-nav">
				{#each profiles as profile}
					<li>
						<button class="uk-button uk-button-link uk-text-lowercase" type="button">
							{profile.name}
						</button>
					</li>
				{/each}
			</ul>
		</div>
	</div>
</div>

<div class="uk-card uk-overflow-auto uk-margin-large-top">
	{#await loadBuckets}
		<Loading />
	{:then}
		<Toolbar location="buckets" />
		<table class="uk-table uk-table-striped uk-table-hover uk-table-divider">
			<thead>
				<tr>
					<th />
					<th class="uk-text-capitalize uk-text-small">Name</th>
					<th class="uk-text-capitalize uk-text-small uk-text-center">Creation Date</th>
					<th class="uk-text-capitalize uk-text-small uk-text-center">Properties</th>
				</tr>
			</thead>
			<tbody>
				<!-- repeat the following block for each bucket -->
				{#each buckets as bucket}
					<tr>
						<td class="uk-text-center">
							<input class="uk-checkbox" on:change={updateSelected(bucket)} type="checkbox" />
						</td>
						<td class="uk-table-expand uk-text-small">
							<a id="bucket-table-link" href="/buckets/{bucket.name}">{bucket.name}</a>
						</td>
						<td class="uk-table-expand uk-text-center uk-text-small">
							{convertToLocaleString(bucket.creation_date)}
						</td>
						<td class="uk-text-small uk-text-center">
							<a href="/buckets/{bucket.name}/properties">
								<span uk-icon="icon: settings" />
							</a>
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	{/await}
</div>

<style>
	#bucket-table-link {
		text-transform: lowercase;
	}
	.profile-button {
		border-radius: 50%;
		width: 47px;
		height: 47px;
	}
</style>
