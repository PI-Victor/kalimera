<script lang="ts">
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	import type { StorageProfile } from '../../../types';
	import { profiles, addProfile, deleteProfile, loadProfiles } from '../../../store';

	var storedProfiles: StorageProfile[] = [];

	function handleDeleteProfile(profileName: string) {
        deleteProfile(profileName);
    }

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


	profiles.subscribe((value) => {
		console.log(value);
	});

	onMount(async () => {
		try {
			credentials = await invoke('read_aws_config');
			setCurrentProfile({ target: { value: Object.keys(credentials)[0] } });
			loadProfiles();
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
		addProfile(currentProfile);
	};
</script>

<div class="uk-card uk-card-body uk-overflow-auto uk-margin-large-top">
	<h6 class="uk-text-medium uk-text-capitalize">Profiles</h6>
	<hr class="uk-divider-icon" />

	<form class="uk-width-2-3 uk-align-center uk-form-horizontal" on:submit|preventDefault={createNewProfile}>
		<fieldset class="uk-fieldset">
			<div class="uk-margin">
				<label class="uk-form-label" for="form-horizontal-text" >AWS Credentials Profile</label>
				<div class="uk-form-controls">
				<select
					class="uk-select small-input"
					id="uk-form-horizontal-select"
					placeholder="Profile name"
					on:change={setCurrentProfile}
					bind:value={currentProfile.name}
				>
					{#each Object.keys(credentials) as credential}
						<option>{credential}</option>
					{/each}
				</select>
			</div>
			<div class="uk-margin">
				<label class="uk-form-label" for="form-horizontal-text">Profile Name</label>
				<div class="uk-form-controls">
				<input
					class="uk-input small-input"
					type="text"
					placeholder="Region"
					bind:value={currentProfile.region}
				/>
			</div>
			<div class="uk-margin">
				<label class="uk-form-label" for="form-horizontal-text">API Endpoint</label>
				<div class="uk-form-controls">
				<input
					class="uk-input small-input"
					type="text"
					placeholder="Endpoint"
					bind:value={currentProfile.endpoint}
				/>
			</div>
			<div class="uk-margin">
				<label class="uk-form-label" for="form-horizontal-text">Force path style</label>
				<div class="uk-form-controls">
				<input
					class="uk-checkbox"
					type="checkbox"
					placeholder="Force Path Style"
					bind:checked={currentProfile.forcePathStyle}
				/>
			<button class="uk-button uk-button-primary uk-align-center" type="submit">Add</button>
		</fieldset>
	</form>
	{#if $profiles.length > 0}
	<div class="uk-width-1-1 uk-align-center uk-card uk-card-body">
		<table
			class="uk-table uk-table-striped uk-table-hover
		uk-table-divider uk-table-small"
		>
			<thead>
				<tr>
					<th class="uk-table-shrink">Name</th>
					<th class="uk-table-expand">Region</th>
					<th class="uk-table-expand">Endpoint</th>
					<th class="uk-table-shrink">Force Path Style</th>
				</tr>
			</thead>
			<tbody>
				{#each $profiles as profile}
					<tr>
						<td>{profile.name}</td>
						<td>{profile.region}</td>
						<td>{profile.endpoint}</td>
						<td>
							<input class="uk-checkbox" type="checkbox" checked={profile.forcePathStyle} />
						</td>
						<td>
							<a href="#" uk-icon="icon: trash" class="uk-text-danger" uk-tooltip="Delete" on:click={() => handleDeleteProfile(profile.name)} />
						</td>
						<td>
							<a href="#" uk-icon="icon: pencil" class="uk-text-primary" uk-tooltip="Edit" />
						</td>
					</tr>
				{/each}
			</tbody>
		</table>
	</div>
	{/if}
</div>

<style>
	.small-input {
		height: 30px; /* Adjust this value as needed */
		font-size: small;
	}
</style>
