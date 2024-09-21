<script>
	import { CalendarIcon } from 'lucide-svelte';
	import { CalendarDate, DateFormatter, getLocalTimeZone } from '@internationalized/date';
	import { cn } from '$lib/utils';
	import { Button } from '$lib/components/ui/button';
	import { RangeCalendar } from '$lib/components/ui/range-calendar';
	import * as Popover from '$lib/components/ui/popover';

	const df = new DateFormatter('en-GB', {
		dateStyle: 'medium'
	});

	const today = new Date();
	const startCalendar = new CalendarDate(
		today.getFullYear(),
		today.getMonth() + 1,
		today.getDate()
	);

	/**
	 * @type {import("bits-ui").DateRange | undefined}
	 */
	export let value = {
		start: startCalendar.subtract({ days: 20 }),
		end: startCalendar
	};

	/**
	 * @type {import("@internationalized/date").DateValue | undefined}
	 */
	let startValue = undefined;
</script>

<div class="gird gap-2">
	<div class="w-auto p-0">
		<RangeCalendar
			bind:value
			bind:startValue
			handleCalendarInitialFocus
			numberOfMonths={2}
			placeholder={value?.start}
		/>
	</div>
</div>
