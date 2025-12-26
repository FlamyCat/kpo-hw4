<script lang="ts">
    import { userId } from '$lib/auth';
    import { env } from '$env/dynamic/public';
    import { onMount } from 'svelte';

    let orders: any[] = [];

    onMount(async () => {
        if (!$userId) return;
        const res = await fetch(`${env.PUBLIC_API_URL}/orders?user_id=${$userId}`);
        if (res.ok) {
            orders = await res.json();
        }
    });
</script>

<h1 class="mb-6 text-2xl font-bold">My Orders</h1>

{#if orders.length === 0}
    <p>No orders found.</p>
{:else}
    <div class="overflow-hidden bg-white shadow sm:rounded-md">
        <ul class="divide-y divide-gray-200">
            {#each orders as order}
                <li class="px-4 py-4 sm:px-6">
                    <div class="flex items-center justify-between">
                        <p class="text-sm font-medium text-blue-600 truncate">{order.description}</p>
                        <div class="ml-2 shrink-0 flex">
                            <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full
                                {order.status === 'Finished' ? 'bg-green-100 text-green-800' :
                                 order.status === 'Cancelled' ? 'bg-red-100 text-red-800' : 'bg-yellow-100 text-yellow-800'}">
                                {order.status}
                            </span>
                        </div>
                    </div>
                    <div class="mt-2 sm:flex sm:justify-between">
                        <div class="sm:flex">
                            <p class="flex items-center text-sm text-gray-500">
                                Amount: ${order.amount}
                            </p>
                        </div>
                        <div class="mt-2 flex items-center text-sm text-gray-500 sm:mt-0">
                            <p>Order ID: {order.id}</p>
                        </div>
                    </div>
                </li>
            {/each}
        </ul>
    </div>
{/if}