<script>
	import SigninForm from '$lib/components/ui/signin/signin-form.svelte';
	import Button from '$lib/components/ui/button/button.svelte';

	/**
	 * @type {import("./$types").LayoutData}
	 */
	export let data;
</script>

{#if !data.user}
	<SigninForm data={data.form} />
{:else}
	<div class="flex">
		<form action="/logout" method="post" class="ml-auto">
			<button>
				<Button variant="ghost">Sign out</Button>
			</button>
		</form>
	</div>
{/if}

<slot />
