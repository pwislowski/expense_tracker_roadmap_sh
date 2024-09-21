import { type RequestHandler } from '@sveltejs/kit';
import { type ExpenseRecord } from '$lib/db/types';

export const PUT: RequestHandler = async ({ request }) => {
	let expenseRecord: ExpenseRecord = await request.json();

	const endpoint = 'http://backend:8080/expense/update';
	return await fetch(endpoint, {
		method: 'PUT',
		headers: {
			'Content-type': 'application/json'
		},
		body: JSON.stringify(expenseRecord)
	});
};
