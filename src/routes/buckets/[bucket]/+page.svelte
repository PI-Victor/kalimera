<script lang="ts">
	import { page } from '$app/stores';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	import type { StorageObject, StorageProfile } from '../../../types';
	import Toolbar from '../../../components/toolbar.svelte';
	import Loading from '../../../components/loading.svelte';

	let error: any;

	let objects: StorageObject[] = [];
	let selectedObjects: StorageObject[] = [];

	const { bucket } = $page.params;
	const profile: StorageProfile = {};

	const listObjects = async () => {
		try {
			objects = await invoke('list_objects', { profile, bucket });
		} catch (e) {
			error = e;
		}
	};

	onMount(async () => {
		await listObjects();
	});

	const loadObjects = listObjects();

	const convertToLocaleString = (timestamp: number) => {
		const date = new Date(timestamp * 1000);
		return date.toLocaleString();
	};
	const convertToMB = (size: number) => {
		if (size === 0) return 0;
		const i = Math.floor(Math.log(size) / Math.log(1024));
		const sizes = ['bytes', 'KB', 'MB', 'GB'];
		size = (size / Math.pow(1024, i)).toFixed(2) * 1;
		return `${size} ${sizes[i]}`;
	};
	const updateSelected = (obj: StorageObject) => {
		return (event: any) => {
			if (event.target.checked) {
				selectedObjects.push(obj);
			} else {
				selectedObjects = selectedObjects.filter((b) => b.name !== obj.name);
			}
		};
	};
</script>

<div class="uk-card uk-margin-large-top">
	{#await loadObjects}
		<Loading />
	{:then}
		<Toolbar location="objects" updateSet={listObjects} />
		<ul class="uk-breadcrumb">
			<li>
				<a href="/buckets/{bucket}" class="uk-text-primary">/{bucket}/</a>
			</li>
		</ul>
		<table class="uk-table uk-table-striped uk-table-hover uk-table-divider">
			<thead>
				<tr>
					<th />
					<th class="uk-text-capitalize uk-text-small">Name</th>
					<th
						class="uk-text-capitalize uk-text-small
				uk-text-center">Last Modfied</th
					>
					<th class="uk-text-capitalize uk-text-small uk-text-center"> Size </th>
					<th
						class="uk-text-capitalize uk-text-small
				uk-text-center"
					>
						Properties
					</th>
					<th
						class="uk-text-capitalize uk-text-small
				uk-text-center">Link</th
					>
				</tr>
			</thead>
			<tbody>
				{#each objects as obj}
					<tr>
						<td class="uk-text-center">
							<input class="uk-checkbox" on:change={updateSelected(obj)} type="checkbox" />
						</td>
						<td class="uk-table-expand uk-text-small"> {obj.name} </td>
						<td class="uk-table-expand uk-text-center uk-text-small">
							{convertToLocaleString(obj.last_modified)}
						</td>
						<td class="uk-table-expand uk-text-center uk-text-small">
							{convertToMB(obj.size)}
						</td>
						<td class="uk-text-small uk-text-center">
							<a href="/buckets/{bucket}/object/properties">
								<span uk-icon="icon: settings" />
							</a>
						</td>
						<td class="uk-text-center">
							<a href="/buckets/{bucket}/object/properties">
								<span uk-icon="icon: link" />
							</a></td
						>
					</tr>
				{/each}
			</tbody>
		</table>
	{:catch error}
		{#if error != null}
			<div class="uk-card">
				{error}
			</div>
		{/if}
	{/await}
</div>
