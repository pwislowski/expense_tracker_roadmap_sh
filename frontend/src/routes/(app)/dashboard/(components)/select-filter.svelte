<script>
	import * as Dialog from '$lib/components/ui/dialog';
	import DateRangePicker from '$lib/components/ui/expense/date-range-picker.svelte';

	export let filterValue;
	$: uniqueValues = ['Past week', 'Past month', 'Last month', 'Last 3 months', 'Custom date range'];

	let dialogOpen = false;

	/**
	 * @type {import("bits-ui").DateRange | undefined}
	 */
	let valueDateRange;
	$: $filterValue = valueDateRange;
</script>

<select
	bind:value={$filterValue}
	on:click|stopPropagation
	on:change={() => {
		if ($filterValue === uniqueValues[uniqueValues.length - 1]) {
			dialogOpen = true;
		}
	}}
>
	<option value={undefined}>All</option>
	{#each uniqueValues as value}
		<option {value}>{value}</option>
	{/each}
</select>

<Dialog.Root bind:open={dialogOpen}>
	<Dialog.Content>
		<Dialog.Header>
			<!-- <Dialog.Title>Select a date range</Dialog.Title> -->
			<DateRangePicker bind:value={valueDateRange} />
		</Dialog.Header>
	</Dialog.Content>
</Dialog.Root>
