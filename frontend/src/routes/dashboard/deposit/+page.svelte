<script lang="ts">
    import { userId } from "$lib/auth";
    import { env } from "$env/dynamic/public";
    import { onMount } from "svelte";

    let amount: number | null = null;
    let currentBalance: number | null = null;
    let isSubmitting = false;
    let message = "";
    let isError = false;

    async function fetchBalance() {
        if (!$userId) return;
        try {
            const res = await fetch(`${env.PUBLIC_API_URL}/accounts/${$userId}`);
            if (res.ok) {
                const data = await res.json();
                currentBalance = data.balance;
            }
        } catch (e) {
            console.error("Failed to fetch balance");
        }
    }

    onMount(fetchBalance);

    async function handleDeposit() {
        if (!amount || amount <= 0) {
            message = "Please enter a valid positive amount";
            isError = true;
            return;
        }

        isSubmitting = true;
        message = "";
        isError = false;

        try {
            const res = await fetch(`${env.PUBLIC_API_URL}/accounts/${$userId}/deposit`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ amount: Number(amount) }),
            });

            if (res.ok) {
                const data = await res.json();
                currentBalance = data.balance;
                message = "Deposit successful!";
                amount = null;
            } else {
                const text = await res.text();
                message = `Deposit failed: ${text}`;
                isError = true;
            }
        } catch (e) {
            message = "Network error occurred";
            isError = true;
        } finally {
            isSubmitting = false;
        }
    }
</script>

<div class="max-w-lg mx-auto bg-white p-6 rounded-lg shadow">
    <h1 class="text-2xl font-bold mb-4">Deposit Funds</h1>

    <div class="mb-6 p-4 bg-gray-50 rounded border border-gray-200 flex justify-between items-center">
        <span class="text-gray-600 font-medium">Current Balance:</span> <span class="text-2xl font-bold text-green-600">
            {#if currentBalance !== null}
                ${currentBalance.toFixed(2)}
            {:else}
                Loading...
            {/if}
        </span>
    </div>

    <form on:submit|preventDefault={handleDeposit}>
        <div class="mb-6">
            <label class="block text-gray-700 text-sm font-bold mb-2" for="amount"> Amount to Deposit ($) </label>
            <input id="amount"
                   type="number"
                   step="0.01"
                   min="0.01"
                   class="shadow appearance-none border rounded w-full py-2 px-3 text-gray-700 leading-tight focus:outline-none focus:shadow-outline"
                   placeholder="100.00"
                   bind:value={amount}
                   required />
        </div>

        {#if message}
            <div class={`mb-4 p-3 rounded text-sm ${isError ? 'bg-red-100 text-red-700' : 'bg-green-100 text-green-700'}`}>
                {message}
            </div>
        {/if}

        <button class="w-full bg-green-500 hover:bg-green-700 text-white font-bold py-2 px-4 rounded focus:outline-none focus:shadow-outline disabled:opacity-50"
                type="submit"
                disabled={isSubmitting}>
            {isSubmitting ? 'Processing...' : 'Deposit'}
        </button>
    </form>
</div>