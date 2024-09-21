<script>
	import Button from '$lib/components/ui/button/button.svelte';
	import * as Form from '$lib/components/ui/form/index.js';
	import { page } from '$app/stores';
	import Input from '$lib/components/ui/input/input.svelte';
	import { formSchema } from './schema';
	import { superForm } from 'sveltekit-superforms';
	import { zodClient } from 'sveltekit-superforms/adapters';
	import SuperDebug from 'sveltekit-superforms/client/SuperDebug.svelte';
	import { redirect } from '@sveltejs/kit';

	export let data;

	const form = superForm(data, {
		validators: zodClient(formSchema)
	});

	const { form: formData, enhance } = form;
</script>

<form method="POST" action="?/register" use:enhance>
	<h3
		class="mb-5 mt-10 scroll-m-20 text-2xl font-semibold tracking-tight transition-colors first:mt-0"
	>
		Sign up
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
	<Form.Field {form} name="passwordConfirm">
		<Form.Control let:attrs>
			<Input
				placeholder="Confirm Password"
				type="password"
				{...attrs}
				bind:value={$formData.passwordConfirm}
			/>
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<div class="flex flex-col space-y-2">
		<Form.Button>Sign up</Form.Button>
	</div>
</form>

<!-- <SuperDebug data={$formData} /> -->
