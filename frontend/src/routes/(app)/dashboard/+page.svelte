<script>
	import ExpenseForm from '$lib/components/ui/expense/expense-form.svelte';
	import DataTable from './(components)/data-table.svelte';
	import Button from '$lib/components/ui/button/button.svelte';
	import { invalidate, invalidateAll } from '$app/navigation';

	/** @type {import('./$types').PageData} */
	export let data;

	$: expenses = data.expenses;
</script>

<div class="container">
	<ExpenseForm data={data.form} />
</div>
<hr class="my-4 h-px border-0 bg-gray-200 dark:bg-gray-700" />
<div class="container">
	<h3 class="scroll-m-20 pb-3 text-xl font-extrabold tracking-tight">List of expenses</h3>
</div>
<div class="container mx-auto">
	<DataTable data={expenses} />
</div>
