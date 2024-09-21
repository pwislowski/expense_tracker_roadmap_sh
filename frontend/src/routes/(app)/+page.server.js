import { writeJwt } from '$lib/auth/auth';
import { formSchema } from '$lib/components/ui/signin/schema';
import { fail, redirect } from '@sveltejs/kit';
import { setError, superValidate } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';

/** @type {import('./$types').PageServerLoad} */
export async function load() {
	return {};
}

/** @type {import('./$types').Actions} */
export const actions = {
	login: async (event) => {
		const form = await superValidate(event, zod(formSchema));
		if (!form.valid) {
			return fail(400, { form });
		}

		let res = await event.fetch('http://backend:8080/login', {
			method: 'POST',
			headers: {
				'Content-type': 'application/json'
			},
			body: JSON.stringify({ username: form.data.username, password: form.data.password })
		});

		if (res.status < 299) {
			let jwt = await res.json();
			writeJwt(event, jwt);

			throw redirect(302, '/dashboard');
		}

		return setError(form, 'username', 'Either user does not exist or credentials invalid.');
	}
};
