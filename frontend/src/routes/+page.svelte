<script lang="ts">
    import { goto } from "$app/navigation";
    import { userId } from "$lib/auth";
    import { env } from "$env/dynamic/public";

    let inputId = "";
    let errorMessage = "";

    // Вход (просто сохраняем ID и переходим)
    function login() {
        if (!inputId) return;
        userId.set(inputId);
        goto("/dashboard/orders");
    }

    // Регистрация (запрос к API)
    async function register() {
        try {
            const res = await fetch(`${env.PUBLIC_API_URL}/accounts`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ balance: 0.0 }), // Начальный баланс 0
            });

            if (res.ok) {
                const data = await res.json();
                userId.set(data.id); // Сохраняем полученный ID
                goto("/dashboard/orders");
            } else {
                errorMessage = "Registration failed";
            }
        } catch (e) {
            errorMessage = "Network error";
        }
    }
</script>

<div class="flex flex-col items-center justify-center min-h-screen bg-gray-100">
    <div class="p-8 bg-white rounded shadow-md w-96">
        <h1 class="mb-6 text-2xl font-bold text-center">Welcome to Shop</h1>

        <div class="mb-4">
            <label class="block mb-2 text-sm font-bold text-gray-700">Account ID</label>
            <input class="w-full px-3 py-2 border rounded" bind:value={inputId} placeholder="Enter your ID" />
        </div>

        <button class="w-full px-4 py-2 mb-2 font-bold text-white bg-blue-500 rounded hover:bg-blue-700"
                on:click={login}>
            Login
        </button>

        <div class="relative my-4">
            <div class="absolute inset-0 flex items-center">
                <div class="w-full border-t"></div>
            </div>
            <div class="relative flex justify-center text-sm"><span class="px-2 bg-white text-gray-500">Or</span></div>
        </div>

        <button class="w-full px-4 py-2 font-bold text-white bg-green-500 rounded hover:bg-green-700"
                on:click={register}>
            Register New Account
        </button>

        {#if errorMessage}
            <p class="mt-4 text-red-500 text-center">{errorMessage}</p>
        {/if}
    </div>
</div>