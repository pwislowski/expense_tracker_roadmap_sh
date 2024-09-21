export type ExpenseRecord = {
	id: {
		tb: string;
		id: {
			String: string;
		};
	};
	username: string;
	name: string;
	value: string;
	category: string;
	date: {
		calendar: {
			identifier: string;
		};
		era: string;
		year: number;
		month: number;
		day: number;
	};
};
