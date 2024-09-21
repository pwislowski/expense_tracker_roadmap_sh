<script>
	import * as Form from '../form/index';
	import { Input } from '../input';
	// import { formSchema } from './schema';
	import { superForm } from 'sveltekit-superforms';
	// import { zodClient } from 'sveltekit-superforms/adapters';
	import Dropdown from './dropdown.svelte';
	import DatePicker from './date-picker.svelte';
	import SuperDebug from 'sveltekit-superforms/client/SuperDebug.svelte';
	import { goto, invalidateAll } from '$app/navigation';

	export let data;

	const form = superForm(data, {
		// validators: zodClient(formSchema),
		dataType: 'form',
		onSubmit: ({ formData }) => {
			formData.set('category', valueCategory.value);
			formData.set('date', JSON.stringify(valueDate));
		}
	});

	const { form: formData, enhance } = form;
	$: $formData.email
		? {
				value: $formData.email
			}
		: undefined;

	/**
	 * @type {string}
	 */
	let valueDate;
	$: $formData.date = valueDate;

	/**
	 * @type {{
	 * value: string,
	 * label: string
	 * }}
	 */
	let valueCategory = {
		value: '',
		label: ''
	};
	$: $formData.category = valueCategory ? valueCategory.value : undefined;
</script>

<form method="POST" use:enhance data-sveltekit-reload on:submit|preventDefault>
	<h3
		class="mb-5 mt-10 scroll-m-20 text-2xl font-semibold tracking-tight transition-colors first:mt-0"
	>
		New Expense
	</h3>
	<Form.Field {form} name="name">
		<Form.Control let:attrs>
			<Input placeholder="Name" {...attrs} bind:value={$formData.name} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="value">
		<Form.Control let:attrs>
			<Input placeholder="Value" {...attrs} bind:value={$formData.value} />
		</Form.Control>
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="category">
		<Dropdown bind:selected={valueCategory} />
		<Form.FieldErrors />
	</Form.Field>
	<Form.Field {form} name="date">
		<DatePicker bind:value={valueDate} />
		<Form.FieldErrors />
	</Form.Field>
	<Form.Button>Add</Form.Button>
</form>

<!-- <SuperDebug data={$formData} /> -->
