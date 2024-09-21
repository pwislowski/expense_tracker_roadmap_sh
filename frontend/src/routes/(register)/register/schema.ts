import { z } from 'zod';

export const formSchema = z
	.object({
		username: z.string().min(2).max(50),
		password: z.string().min(2).max(50),
		passwordConfirm: z.string().min(2).max(50)
	})
	.refine((data) => data.password === data.passwordConfirm, {
		message: 'Passwords do not match',
		path: ['passwordConfirm']
	});
