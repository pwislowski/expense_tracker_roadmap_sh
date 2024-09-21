import { keyToken } from '$lib/auth/auth';
import { redirect } from '@sveltejs/kit';

/** @type {import('./$types').RequestHandler} */
export async function POST({ cookies }) {
	cookies.delete(keyToken, {
		path: '/'
	});
	return redirect(303, '/');
}
