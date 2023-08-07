<script lang="ts">
	import { page } from '$app/stores';
</script>

<div class="uk-card">
	{#if $page.error != null}
		{$page.error.message}
	{/if}
</div>
