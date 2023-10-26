<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';

	import type { StorageBucket, StorageProfile } from '../types';
	import Toolbar from '../components/toolbar.svelte';
	import Loading from '../components/loading.svelte';

	let error: any;
	let buckets: StorageBucket[] = [];
	let selectedBuckets: StorageBucket[] = [];

	const profile: StorageProfile = {
		name: 'scaleway',
		region: 'fr-par',
		endpoint: 'https://s3.fr-par.scw.cloud',
		accessKeyId: '',
		secretAccessKey: '',
		forcePathStyle: false,
		current: true
	};

	const listBuckets = async () => {
		buckets = await invoke('list_buckets', { profile });
	};

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

<!-- 
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
</div> -->

<div class="uk-card uk-margin-large-top">
	{#await listBuckets()}
		<Loading />
	{:then}
		<Toolbar location="buckets" updateSet={listBuckets} />
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
							<a class="uk-text-lowercase" href="/buckets/{bucket.name}">{bucket.name}</a>
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
	{:catch error}
		<p>I got an error: {error}</p>
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
