<script>
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import { CalendarDate } from '@internationalized/date';
	import { Button } from '$lib/components/ui/button';
	import { Ellipsis, XCircle, Edit } from 'lucide-svelte';
	import * as Dialog from '$lib/components/ui/dialog';
	import Input from '$lib/components/ui/input/input.svelte';
	import Dropdown from '$lib/components/ui/expense/dropdown.svelte';
	import DatePicker from '$lib/components/ui/expense/date-picker.svelte';

	/**
	 * @type {string}
	 */
	export let id;

	/**
	 * @type {import("$lib/db/types").ExpenseRecord[]}
	 */
	export let data;

	/**
	 *
	 * @param {import("$lib/db/types").ExpenseRecord} expenseRecord
	 */
	async function updateExpense(expenseRecord) {
		let res = await fetch('/api/db/update_expense', {
			method: 'PUT',
			headers: {
				'Content-type': 'application/json'
			},
			body: JSON.stringify(expenseRecord)
		});
	}

	/**
	 *
	 * @param {string} id
	 */
	async function removeExpense(id) {
		let res = await fetch('/api/db/remove_expense', {
			method: 'DELETE',
			headers: {
				'Content-type': 'application/json'
			},
			body: JSON.stringify({ id: id })
		});
	}

	const thisRecord = data.filter((o) => o.id.id.String === id)[0];
	let dialogOpen = false;

	let dateValue = new CalendarDate(
		thisRecord.date.year,
		thisRecord.date.month,
		thisRecord.date.day
	);
	$: thisRecord.date = JSON.parse(JSON.stringify(dateValue));

	/**
	 * @type {import("bits-ui").Selected<string>}
	 */
	let categoryValue = {
		value: thisRecord.category,
		label: thisRecord.category.at(0)?.toUpperCase() + thisRecord.category.slice(1)
	};
	$: thisRecord.category = categoryValue ? categoryValue.value : '';
</script>

<DropdownMenu.Root>
	<DropdownMenu.Trigger asChild let:builder>
		<Button variant="ghost" builders={[builder]} size="icon" class="relative h-8 w-8 p-0">
			<span class="sr-only">Open menu</span>
			<Ellipsis />
		</Button>
	</DropdownMenu.Trigger>
	<DropdownMenu.Content>
		<DropdownMenu.Group>
			<DropdownMenu.Label>Actions</DropdownMenu.Label>
			<DropdownMenu.Item
				on:click={() => {
					removeExpense(id);
				}}
			>
				<XCircle class="mr-0.5 scale-50" /> Remove
			</DropdownMenu.Item>
		</DropdownMenu.Group>
		<!-- <DropdownMenu.Separator /> -->
		<DropdownMenu.Item on:click={() => (dialogOpen = true)}>
			<Edit class="mr-0.5 scale-50" />
			Edit</DropdownMenu.Item
		>
	</DropdownMenu.Content>
</DropdownMenu.Root>

<Dialog.Root bind:open={dialogOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<Dialog.Title>Edit a record</Dialog.Title>
		</Dialog.Header>
		<!-- <Dialog.Description>
			This action cannot be undone. This will permanently delete your account and remove your data
			from our servers.
		</Dialog.Description> -->
		<div class="space-y-2">
			<Input placeholder="Name" bind:value={thisRecord.name} />
			<Input placeholder="Value" bind:value={thisRecord.value} />
			<Dropdown bind:selected={categoryValue} />
			<DatePicker bind:value={dateValue} />
		</div>
		<div class="flex items-center justify-center">
			<Button
				class="max-w-fit"
				on:click={async () => {
					await updateExpense(thisRecord);
					dialogOpen = false;
				}}>Save</Button
			>
		</div>
	</Dialog.Content>
</Dialog.Root>
