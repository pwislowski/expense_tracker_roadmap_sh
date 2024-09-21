<script>
	import { DateFormatter, getLocalTimeZone } from '@internationalized/date';
	import { Calendar as CalendarIcon } from 'svelte-radix';
	import { cn } from '../../../utils';
	import { Button } from '../button/index';
	import { Calendar } from '../calendar/index';
	import * as Popover from '../popover/index';

	const df = new DateFormatter('en-GB', {
		dateStyle: 'long'
	});

	/**
	 * @type {import("@internationalized/date").DateValue}
	 */
	export let value;
</script>

<Popover.Root>
	<Popover.Trigger asChild let:builder>
		<Button
			variant="outline"
			class={cn('w-[280px] justify-start text-left font-normal', !value && 'text-muted-foreground')}
			builders={[builder]}
		>
			<CalendarIcon class="mr-2 h-4 w-4" />
			{value ? df.format(value.toDate(getLocalTimeZone())) : 'Pick a date'}
		</Button>
	</Popover.Trigger>
	<Popover.Content class="w-auto p-0">
		<Calendar bind:value initialFocus />
	</Popover.Content>
</Popover.Root>
