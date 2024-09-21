import { writeJwt } from '$lib/auth/auth';
import { redirect } from '@sveltejs/kit';
import { formSchema } from './schema';
import { superValidate, fail, setError } from 'sveltekit-superforms';
import { zod } from 'sveltekit-superforms/adapters';

/** @type {import('./$types').PageServerLoad} */
export async function load({ locals }) {
	return {
		form: await superValidate(zod(formSchema)),
		user: locals.userToken
	};
}

/** @type {import('./$types').Actions} */
export const actions = {
	register: async (event) => {
		const form = await superValidate(event, zod(formSchema));

		if (!form.valid) {
			return fail(400, { form });
		}

		let res = await event.fetch('http://backend:8080/register', {
			method: 'POST',
			headers: {
				'Contenty-type': 'application/json'
			},
			body: JSON.stringify({ username: form.data.username, password: form.data.password })
		});

		if (res.status == 409) {
			return setError(form, 'username', 'Username already in use');
		}

		if (res.status == 202) {
			let jwt = await res.json();
			writeJwt(event, jwt);

			throw redirect(302, '/dashboard');
		}

		return { form };
	}
};
