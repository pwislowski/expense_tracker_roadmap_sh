import { keyToken } from '$lib/auth/auth';
import { formSchema } from '$lib/components/ui/expense/schema';
import { superValidate, fail } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';
import { getExpenses } from '$lib/db/getExpenses';
import { date } from 'zod';

export async function load({ cookies }) {
	const expenses = getExpenses(cookies);

	return {
		form: await superValidate(zod(formSchema)),
		expenses: await expenses
	};
}

/** @type {import('./$types').Actions} */
export const actions = {
	default: async (event) => {
		const token = event.cookies.get(keyToken);
		// const form = await superValidate(event, zod(formSchema));
		const form = await event.request.formData();

		const name = form.get('name');
		const value = form.get('value');
		const category = form.get('category');
		const dateJson = form.get('date');
		// @ts-ignore
		const date = JSON.parse(dateJson);

		let res = await event.fetch('http://backend:8080/expense/new', {
			method: 'POST',
			headers: {
				'Content-type': 'application/json'
			},
			body: JSON.stringify({
				name: name,
				value: value,
				category: category,
				date: date,
				jwt_token: token
			})
		});
	}
};
