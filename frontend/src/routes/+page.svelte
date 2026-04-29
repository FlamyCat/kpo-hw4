<script lang="ts">
    import { goto } from "$app/navigation";
    import { userId } from "$lib/auth";
    import { env } from "$env/dynamic/public";

    let inputId = "";
    let errorMessage = "";

    function login() {
        if (!inputId) return;
        userId.set(inputId);
        goto("/dashboard/orders");
    }

    async function register() {
        try {
            const res = await fetch(`${env.PUBLIC_API_URL}/accounts`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ balance: 0.0 }),
            });

            if (res.ok) {
                const data = await res.json();
                userId.set(data.id);
                await goto("/dashboard/orders");
            } else {
                errorMessage = "Registration failed";
            }
        } catch (e) {
            errorMessage = "Network error";
        }
    }
</script>

<div class="flex flex-col items-center justify-center min-h-screen bg-gray-100 dark:bg-gray-900 transition-colors duration-200">
    <div class="p-8 bg-white rounded shadow-md w-96 dark:bg-gray-800 transition-colors duration-200">
        <h1 class="mb-6 text-2xl font-bold text-center dark:text-white">Welcome to Shop</h1>

        <div class="mb-4">
            <label for="user-id" class="block mb-2 text-sm font-bold text-gray-700 dark:text-gray-300">Account ID</label>
            <input id="user-id" class="w-full px-3 py-2 border rounded dark:bg-gray-700 dark:border-gray-600 dark:text-white dark:placeholder-gray-400 focus:outline-none focus:ring-2 focus:ring-blue-500"
                   bind:value={inputId}
                   placeholder="Enter your ID" />
        </div>

        <button class="w-full px-4 py-2 mb-2 font-bold text-white bg-blue-500 rounded hover:bg-blue-700 dark:bg-blue-600 dark:hover:bg-blue-800"
                on:click={login}>
            Login
        </button>

        <div class="relative my-4">
            <div class="absolute inset-0 flex items-center">
                <div class="w-full border-t dark:border-gray-600"></div>
            </div>
            <div class="relative flex justify-center text-sm">
                <span class="px-2 bg-white text-gray-500 dark:bg-gray-800 dark:text-gray-400">Or</span></div>
        </div>

        <button class="w-full px-4 py-2 font-bold text-white bg-green-500 rounded hover:bg-green-700 dark:bg-green-600 dark:hover:bg-green-800"
                on:click={register}>
            Register New Account
        </button>

        {#if errorMessage}
            <p class="mt-4 text-red-500 text-center dark:text-red-400">{errorMessage}</p>
        {/if}
    </div>
</div>