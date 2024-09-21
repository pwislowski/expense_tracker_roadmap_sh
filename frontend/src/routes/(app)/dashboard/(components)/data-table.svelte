<script>
	import { createTable, Render, Subscribe, createRender } from 'svelte-headless-table';
	import SelectFilter from './select-filter.svelte';
	import { readable } from 'svelte/store';
	import * as Table from '$lib/components/ui/table';
	import DataTableActions from './data-table-actions.svelte';
	import {
		addPagination,
		addSortBy,
		addTableFilter,
		addSelectedRows,
		addColumnFilters
	} from 'svelte-headless-table/plugins';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import DataTableCheckbox from './data-table-checkbox.svelte';
	import { ArrowDown } from 'lucide-svelte';
	import { onMount } from 'svelte';
	// @ts-ignore
	import { custom } from 'zod';

	/**
	 *
	 * @param {Date} d
	 * @returns {number[]}
	 */
	function getWeekNumber(d) {
		// Copy date so don't modify original
		d = new Date(Date.UTC(d.getFullYear(), d.getMonth(), d.getDate()));
		// Set to nearest Thursday: current date + 4 - current day number
		// Make Sunday's day number 7
		d.setUTCDate(d.getUTCDate() + 4 - (d.getUTCDay() || 7));
		// Get first day of year
		var yearStart = new Date(Date.UTC(d.getUTCFullYear(), 0, 1));
		// Calculate full weeks to nearest Thursday
		// @ts-ignore
		var weekNo = Math.ceil(((d - yearStart) / 86400000 + 1) / 7);
		// Return array of year and week number
		return [d.getUTCFullYear(), weekNo];
	}

	/**
	 *
	 * @param {{
	 * filterValue: import("@internationalized/date").DateValue | import("bits-ui").DateRange | string,
	 * value: import("@internationalized/date").DateValue
	 * }} param0
	 */
	const matchFilter = ({ filterValue, value }) => {
		if (filterValue === undefined) return true;
		const today = new Date();

		let other = new Date(value.year, value.month - 1, value.day);

		switch (filterValue) {
			case 'Past week':
				const [_year, currentWeek] = getWeekNumber(today);
				const [_yearOther, filteredWeek] = getWeekNumber(other);

				return filteredWeek === currentWeek - 1;
			case 'Past month':
				return today.getFullYear() === other.getFullYear() && today.getMonth() === other.getMonth();
			case 'Last month':
				return (
					today.getFullYear() === other.getFullYear() && today.getMonth() - 1 === other.getMonth()
				);
			case 'Last 3 months':
				other = new Date(value.year, value.month - 1, value.day);
				return (
					today.getFullYear() === other.getFullYear() &&
					today.getMonth() >= other.getMonth() &&
					today.getMonth() - 3 <= other.getMonth()
				);
			case undefined:
				return true;
			default:
				/**
				 * @type {import("bits-ui").DateRange}
				 */
				// @ts-ignore
				const customRange = filterValue;
				// @ts-ignore
				const leftCustomDate = new Date(
					// @ts-ignore
					customRange.start?.year,
					// @ts-ignore
					customRange.start?.month - 1,
					customRange.start?.day
				);
				// @ts-ignore
				const rightCustomDate = new Date(
					// @ts-ignore
					customRange.end?.year,
					// @ts-ignore
					customRange.end?.month - 1,
					customRange.end?.day
				);

				return leftCustomDate <= other && other <= rightCustomDate;
		}
	};

	let currentRelativePath = '';
	onMount(() => {
		currentRelativePath = window.location.href;
	});

	/**
	 * @type {import("$lib/db/types").ExpenseRecord[]}
	 */
	// @ts-ignore
	export let data;

	const dateOptions = {
		// weekday: 'long',
		year: 'numeric',
		month: 'long',
		day: 'numeric'
	};

	const table = createTable(readable(data), {
		page: addPagination(),
		sort: addSortBy({ initialSortKeys: [{ id: 'value', order: 'asc' }] }),
		filter: addTableFilter({
			fn: ({ filterValue, value }) => {
				return value.toLowerCase().includes(filterValue.toLocaleLowerCase());
			}
		}),
		select: addSelectedRows(),
		colFilter: addColumnFilters()
	});
	const columns = table.createColumns([
		table.column({
			accessor: ({ id }) => id.id.String,
			header: (_, { pluginStates }) => {
				const { allPageRowsSelected } = pluginStates.select;
				return createRender(DataTableCheckbox, {
					checked: allPageRowsSelected
				});
			},
			cell: ({ row }, { pluginStates }) => {
				const { getRowState } = pluginStates.select;
				const { isSelected } = getRowState(row);

				return createRender(DataTableCheckbox, {
					checked: isSelected
				});
			}
		}),
		table.column({
			accessor: 'name',
			header: 'Name',
			cell: ({ value }) => {
				return value.charAt(0).toUpperCase() + value.slice(1);
			},
			plugins: {
				sort: {
					disable: true
				}
			}
		}),
		table.column({
			accessor: 'value',
			header: 'Value',
			cell: ({ value }) => {
				const float = parseFloat(value);
				const formatted = new Intl.NumberFormat('en-US', {
					style: 'currency',
					currency: 'GBP'
				}).format(float);

				return formatted;
			},
			plugins: {
				filter: {
					exclude: true
				},
				sort: {
					compareFn: (a, b) => {
						return a - b;
					}
				}
			}
		}),
		table.column({
			accessor: 'category',
			header: 'Category',
			cell: ({ value }) => {
				return value.charAt(0).toUpperCase() + value.slice(1);
			},
			plugins: {
				sort: {
					disable: true
				}
			}
		}),
		table.column({
			accessor: 'date',
			header: 'Date',
			cell: ({ value }) => {
				const dateObject = new Date();
				dateObject.setMonth(value.month - 1);
				dateObject.setDate(value.day);
				dateObject.setFullYear(value.year);

				// @ts-ignore
				return dateObject.toLocaleDateString('en-GB', dateOptions);
			},
			plugins: {
				sort: {
					disable: true
				},
				filter: {
					exclude: true
				},
				colFilter: {
					// @ts-ignore
					fn: matchFilter,
					render: ({ filterValue }) => {
						return createRender(SelectFilter, { filterValue });
					}
				}
			}
		}),
		table.column({
			accessor: ({ id }) => id.id.String,
			header: '',
			cell: ({ value }) => {
				return createRender(DataTableActions, { id: value, data: data });
			},
			plugins: {
				sort: {
					disable: true
				},
				filter: {
					exclude: true
				}
			}
		})
	]);

	const { headerRows, pageRows, tableAttrs, tableBodyAttrs, pluginStates, rows } =
		table.createViewModel(columns);

	const { hasNextPage, hasPreviousPage, pageIndex } = pluginStates.page;
	const { filterValue } = pluginStates.filter;
	const { selectedDataIds } = pluginStates.select;
</script>

<div>
	<div class="flex items-center space-x-8 py-4">
		<Input
			class="max-w-sm"
			placeholder="Filter expenses..."
			type="text"
			bind:value={$filterValue}
		/>
		<Button><a href={currentRelativePath} data-sveltekit-reload>Refresh</a></Button>
	</div>
	<div class="rounded-md border">
		<Table.Root {...$tableAttrs}>
			<Table.Header>
				{#each $headerRows as headerRow}
					<Subscribe rowAttrs={headerRow.attrs()}>
						<Table.Row>
							{#each headerRow.cells as cell (cell.id)}
								<Subscribe attrs={cell.attrs()} let:attrs props={cell.props()} let:props>
									<Table.Head {...attrs} class="[&:has([role=checkbox])]:pl-3">
										{#if cell.id === 'value'}
											<Button variant="ghost" on:click={props.sort.toggle}>
												<Render of={cell.render()} />
												<ArrowDown />
											</Button>
										{:else}
											<Render of={cell.render()} />
										{/if}
										{#if props.colFilter?.render}
											<Render of={props.colFilter.render} />
										{/if}
									</Table.Head>
								</Subscribe>
							{/each}
						</Table.Row>
					</Subscribe>
				{/each}
			</Table.Header>
			<Table.Body {...$tableBodyAttrs}>
				{#each $pageRows as row (row.id)}
					<Subscribe rowAttrs={row.attrs()} let:rowAttrs>
						<Table.Row {...rowAttrs} data-state={$selectedDataIds[row.id] && 'selected'}>
							{#each row.cells as cell (cell.id)}
								<Subscribe attrs={cell.attrs()} let:attrs>
									<Table.Cell {...attrs}>
										<Render of={cell.render()} />
									</Table.Cell>
								</Subscribe>
							{/each}
						</Table.Row>
					</Subscribe>
				{/each}
			</Table.Body>
		</Table.Root>
	</div>
	<div class="flex items-center justify-end space-x-4 py-4">
		<div class="flex-1 text-sm text-muted-foreground">
			{Object.keys($selectedDataIds).length} of {' '}
			{$rows.length} row(s) selected.
		</div>

		<Button
			variant="outline"
			size="sm"
			on:click={() => ($pageIndex = $pageIndex - 1)}
			disabled={!$hasPreviousPage}>Previous</Button
		>
		<Button
			variant="outline"
			size="sm"
			disabled={!$hasNextPage}
			on:click={() => ($pageIndex = $pageIndex + 1)}>Next</Button
		>
	</div>
</div>

<!-- <p>selected:</p>
{#each Object.keys($selectedDataIds) as idx}
	<p>{JSON.stringify(data[idx])}</p>
{/each} -->
