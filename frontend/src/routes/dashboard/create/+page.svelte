<script lang="ts">
    import { userId } from "$lib/auth";
    import { goto } from "$app/navigation";
    import { env } from "$env/dynamic/public";

    let description = "";
    let amount: number | null = null;
    let isSubmitting = false;
    let errorMessage = "";

    async function handleSubmit() {
        if (!description || !amount || amount <= 0) {
            errorMessage = "Please provide valid description and amount";
            return;
        }
        isSubmitting = true;
        errorMessage = "";
        try {
            const res = await fetch(`${env.PUBLIC_API_URL}/orders`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    user_id: $userId,
                    description: description,
                    amount: Number(amount),
                }),
            });
            if (res.ok) {
                await goto("/dashboard/orders");
            } else {
                const text = await res.text();
                errorMessage = `Failed to create order: ${text}`;
            }
        } catch (e) {
            errorMessage = "Network error occurred";
        } finally {
            isSubmitting = false;
        }
    }
</script>

<div class="max-w-lg mx-auto bg-white p-6 rounded-lg shadow dark:bg-gray-800 transition-colors">
    <h1 class="text-2xl font-bold mb-6 dark:text-white">Create New Order</h1>

    <form on:submit|preventDefault={handleSubmit}>
        <div class="mb-4">
            <label class="block text-gray-700 text-sm font-bold mb-2 dark:text-gray-300"
                   for="desc"> Description </label>
            <input id="desc"
                   type="text"
                   class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline
                       dark:bg-gray-700 dark:border-gray-600 dark:text-white dark:placeholder-gray-400"
                   placeholder="Laptop, Coffee, etc."
                   bind:value={description}
                   required />
        </div>

        <div class="mb-6">
            <label class="block text-gray-700 text-sm font-bold mb-2 dark:text-gray-300"
                   for="amount"> Amount ($) </label>
            <input id="amount"
                   type="number"
                   step="0.01"
                   min="0.01"
                   class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline
                       dark:bg-gray-700 dark:border-gray-600 dark:text-white dark:placeholder-gray-400"
                   placeholder="0.00"
                   bind:value={amount}
                   required />
        </div>

        {#if errorMessage}
            <div class="mb-4 p-3 bg-red-100 text-red-700 rounded text-sm dark:bg-red-900 dark:text-red-200">
                {errorMessage}
            </div>
        {/if}

        <div class="flex items-center justify-between">
            <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline disabled:opacity-50 dark:bg-blue-600 dark:hover:bg-blue-800"
                    type="submit"
                    disabled={isSubmitting}>
                {isSubmitting ? 'Creating...' : 'Place Order'}
            </button>

            <a href="/dashboard/orders"
               class="inline-block align-baseline font-bold text-sm text-blue-500 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300"> Cancel </a>
        </div>
    </form>
</div>