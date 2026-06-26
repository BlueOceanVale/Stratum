<script lang="ts">
    import { API } from "$lib/api";

    let email = $state("");
    let password = $state("");

    async function register() {
        const response = await fetch(`${API}/register`, {
            method: "POST",
            headers: {
                "Content-Type": "application/json"
            },
            body: JSON.stringify({
                email,
                password,
            })
        });

        const resp = await response.json();

        console.log(resp);

        window.location.href = "/login";
    }
</script>

<h1 class="text-3xl font-bold mb-4 p-2">Stratum Register</h1>

<hr class="border-gray-400 mt-5 mb-5">

<input
	class="border-1 border-gray-400 rounded p-2 w-full mb-3 ml-2"
	type="email"
	placeholder="Email"
	bind:value={email}
/>

<input
	class="border-1 border-gray-400 rounded p-2 w-full mb-3 ml-2"
	type="password"
	placeholder="Password"
	bind:value={password}
/>

<button
	class="border rounded px-4 py-2 ml-2 mt-2"
	onclick={register}
>
	Register
</button>