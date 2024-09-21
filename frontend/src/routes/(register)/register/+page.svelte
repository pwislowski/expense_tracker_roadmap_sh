<script>
	import RegisterForm from './register-form.svelte';

	/** @type {import('./$types').PageData} */
	export let data;
</script>

<RegisterForm data={data.form} />
