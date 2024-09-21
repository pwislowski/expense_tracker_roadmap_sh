import { type RequestHandler } from '@sveltejs/kit';

export const DELETE: RequestHandler = async ({ request }) => {
	let { id } = await request.json();

	const endpoint = 'http://backend:8080/expense/remove';
	return await fetch(endpoint, {
		method: 'DELETE',
		headers: {
			'Content-type': 'application/json'
		},
		body: JSON.stringify({ id: id })
	});
};
