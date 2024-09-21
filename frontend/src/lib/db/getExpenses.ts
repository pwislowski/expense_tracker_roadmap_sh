import type { Cookies } from '@sveltejs/kit';
import type { ExpenseRecord } from './types';
import { keyToken } from '$lib/auth/auth';

export async function getExpenses(cookies: Cookies): Promise<ExpenseRecord[]> {
	const token = cookies.get(keyToken);

	let res = fetch('http://backend:8080/expense/get', {
		method: 'POST',
		headers: {
			'Content-type': 'application/json'
		},
		body: JSON.stringify({ auth: token })
	});

	return (await res).json();
}
