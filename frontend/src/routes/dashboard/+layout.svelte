<script lang="ts">
    import { logout, userId } from "$lib/auth";
    import { goto } from "$app/navigation";
    import { onMount } from "svelte";

    onMount(async () => {
        if (!$userId) {
            await goto("/");
        }
    });

    async function handleLogout() {
        logout();
        await goto("/");
    }
</script>

<div class="min-h-screen bg-gray-50">
    <nav class="bg-white shadow">
        <div class="px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
            <div class="flex justify-between h-16">
                <div class="flex">
                    <div class="flex items-center shrink-0 font-bold">My Shop</div>
                    <div class="hidden sm:ml-6 sm:flex sm:space-x-8">
                        <a href="/dashboard/orders"
                           class="inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium border-transparent hover:border-gray-300">My Orders</a>
                        <a href="/dashboard/create"
                           class="inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium border-transparent hover:border-gray-300">New Order</a>
                    </div>
                </div>
                <div class="flex items-center">
                    <span class="mr-4 text-sm text-gray-500">ID: {$userId}</span>
                    <button on:click={handleLogout} class="text-sm text-red-600 hover:text-red-900">Logout</button>
                </div>
            </div>
        </div>
    </nav>

    <main class="py-10">
        <div class="px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
            <slot />
        </div>
    </main>
</div>