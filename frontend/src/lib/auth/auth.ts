import type { RequestEvent } from '@sveltejs/kit';

export const keyToken = 'auth';

export async function authenticateUser(event: RequestEvent): Promise<string | null> {
	const token = event.cookies.get(keyToken);

	if (typeof token === undefined) {
		return null;
	}

	let res = await fetch('http://backend:8080/verify', {
		method: 'POST',
		headers: {
			'Content-Type': 'application/json'
		},
		body: JSON.stringify({ auth: token })
	});

	if (res.status == 202) {
		return 'user';
	}
	return null;
}

export async function writeJwt(event: RequestEvent, token: string) {
	event.cookies.set(keyToken, token, {
		path: '/',
		httpOnly: true,
		sameSite: 'strict',
		secure: false,
		maxAge: 60 * 60 * 24 * 7 // 1 week
	});
}
