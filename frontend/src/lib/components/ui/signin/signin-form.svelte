<script>
	import Button from '../button/button.svelte';
	import * as Form from '../form/index';
	import { page } from '$app/stores';
	import { Input } from '../input';
	import { formSchema } from './schema';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import SuperDebug from 'sveltekit-superforms/client/SuperDebug.svelte';
	import { redirect } from '@sveltejs/kit';

	export let data;

	const form = superForm(data, {
		validators: zodClient(formSchema)
	});

	const { form: formData, enhance, errors } = form;
	/**
	 * @type {string}
	 */
	let message;
</script>

<form method="POST" action="?/login" use:enhance>
	<h3
		class="mb-5 mt-10 scroll-m-20 text-2xl font-semibold tracking-tight transition-colors first:mt-0"
	>
		Sign in
	</h3>
	<Form.Field {form} name="username">
		<Form.Control let:attrs>
			<Input placeholder="Username" {...attrs} bind:value={$formData.username} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="password">
		<Form.Control let:attrs>
			<Input placeholder="Password" type="password" {...attrs} bind:value={$formData.password} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<div class="flex flex-col space-y-2">
		<Form.Button>Sign in</Form.Button>
		<Button variant="secondary" href="/register">Create a new account?</Button>
	</div>
</form>

<!-- <SuperDebug data={$formData} /> -->
