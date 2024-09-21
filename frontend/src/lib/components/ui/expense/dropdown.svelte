<script>
	import * as Select from '$lib/components/ui/select';
	import { categories } from './schema';

	/**
	 * @type {import("bits-ui").Selected<string>[]}
	 */
	const items = [];

	for (const category of categories.toSorted()) {
		const item = {
			value: category,
			label: category.at(0)?.toUpperCase() + category.slice(1)
		};
		items.push(item);
	}

	let placeholder = 'Category';

	/**
	 * @type {import("bits-ui").Selected<string>}
	 */
	export let selected;
</script>

<Select.Root {items} bind:selected>
	<Select.Trigger class="w-[180px]">
		<Select.Value {placeholder} />
	</Select.Trigger>
	<Select.Content>
		{#each items as item}
			<Select.Item value={item.value} label={item.label} />
		{/each}
	</Select.Content>
</Select.Root>
