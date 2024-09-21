import { formSchema } from '$lib/components/ui/signin/schema';
import { superValidate } from 'sveltekit-superforms/server';
import { zod } from 'sveltekit-superforms/adapters';

/** @type {import('./$types').LayoutServerLoad} */
export async function load({ locals }) {
	return {
		form: await superValidate(zod(formSchema)),
		user: locals.userToken
	};
}
