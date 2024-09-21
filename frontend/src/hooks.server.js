import { authenticateUser } from '$lib/auth/auth';
import { redirect } from '@sveltejs/kit';

//* @type {import("@sveltejs/kit").Handle} */
export const handle = async ({ event, resolve }) => {
	event.locals.userToken = await authenticateUser(event);

	if (event.url.pathname.startsWith('/dashboard')) {
		if (!event.locals.userToken) {
			throw redirect(303, '/');
		}
	}

	if (event.url.pathname == '/') {
		if (event.locals.userToken) {
			throw redirect(303, '/dashboard');
		}
	}

	// const res = await resolve(event)
	return resolve(event);
};
