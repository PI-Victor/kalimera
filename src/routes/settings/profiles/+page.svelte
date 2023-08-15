<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	import type { StorageProfile } from '../../../types';
	import { profiles } from '../../../store';

	let error;
	type Credentials = {
		[key: string]: {
			aws_access_key_id: string;
			aws_secret_access_key: string;
		};
	};
	let credentials: Credentials = {};
	let accessKeyId: string;
	let secretAccessKey: string;
	let currentProfile: StorageProfile = {};

	profiles.set(currentProfile);

	profiles.subscribe((value) => {
		console.log(value);
	});

	onMount(async () => {
		try {
			credentials = await invoke('read_aws_config');
			setCurrentProfile({ target: { value: Object.keys(credentials)[0] } });
		} catch (e) {
			error = e;
		}
	});

	const setCurrentProfile = (event: any) => {
		accessKeyId = credentials[event.target.value].aws_access_key_id;
		secretAccessKey = credentials[event.target.value].aws_secret_access_key;

		currentProfile = {
			...currentProfile,
			name: event.target.value,
			accessKeyId,
			secretAccessKey
		};
	};
	const createNewProfile = async () => {
		console.log(currentProfile);
	};
</script>

<div class="uk-card uk-card-body uk-overflow-auto uk-margin-large-top">
	<h6 class="uk-text-medium uk-text-capitalize">Profiles</h6>
	<hr class="uk-divider-icon" />

	<form class="uk-width-3-5 uk-align-center" on:submit|preventDefault={createNewProfile}>
		<fieldset class="uk-fieldset">
			<div class="uk-margin">
				<select
					class="uk-select small-input"
					placeholder="Profile name"
					on:change={setCurrentProfile}
					bind:value={currentProfile.name}
				>
					{#each Object.keys(credentials) as profile}
						<option>{profile}</option>
					{/each}
				</select>
			</div>
			<div class="uk-margin">
				<input
					class="uk-input small-input"
					type="text"
					placeholder="Region"
					bind:value={currentProfile.region}
				/>
			</div>
			<div class="uk-margin">
				<input
					class="uk-input small-input"
					type="text"
					placeholder="Endpoint"
					bind:value={currentProfile.endpoint}
				/>
			</div>
			<button class="uk-button uk-button-primary uk-align-center" type="submit">Add</button>
		</fieldset>
	</form>
	<div class="uk-card uk-card-body">
		<table
			class="uk-table uk-table-striped uk-table-hover
		uk-table-divider"
		>
			<thead>
				<tr>
					<th class="uk-table-expand uk-text-lowercase">Name</th>
					<th class="uk-table-shrink uk-text-lowercase">Region</th>
					<th class="uk-table-expand uk-text-lowercase">Endpoint</th>
				</tr>
			</thead>
			<tbody>
				{#each Object.keys(credentials) as profile}
					<tr>
						<td>{profile}</td>
						<td>{credentials[profile].region}</td>
						<td>{credentials[profile].endpoint}</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
</div>

<style>
	.small-input {
		height: 30px; /* Adjust this value as needed */
		font-size: small;
	}
</style>
