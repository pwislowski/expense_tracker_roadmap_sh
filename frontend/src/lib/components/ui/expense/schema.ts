import { data } from 'autoprefixer';
import { z } from 'zod';

export const categories = ['homeware', 'food', 'bills', 'party'];

// const calendarSchema = z.object({
// 	identifier: z.string()
// });

// const dateSchema = z.object({
// 	calendar: calendarSchema.array(),
// 	era: z.string(),
// 	year: z.any(),
// 	month: z.number(),
// 	day: z.number()
// });

export const formSchema = z
	.object({
		name: z.any(),
		value: z.any(),
		category: z.any(),
		date: z.any()
	})
	.refine((data) => {
		categories.includes(data.category),
			{
				message: 'Incorrect category type',
				path: ['category']
			};
	});
