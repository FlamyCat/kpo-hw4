<script lang="ts">
    import { logout, userId } from "$lib/auth";
    import { goto } from "$app/navigation";
    import { page } from "$app/state";
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

    function navClass(href: string, currentPath: string) {
        const isActive = currentPath.includes(href);

        const base = "inline-flex items-center px-1 pt-1 border-b-2 text-sm font-medium transition-colors duration-200";

        const activeState = "border-blue-500 text-gray-900 dark:border-blue-400 dark:text-white";

        const inactiveState = "border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 dark:text-gray-300 dark:hover:text-white dark:hover:border-gray-600";

        return `${base} ${isActive ? activeState : inactiveState}`;
    }
</script>

<div class="min-h-screen bg-gray-50 dark:bg-gray-900 transition-colors duration-200">
    <nav class="bg-white shadow dark:bg-gray-800 dark:border-b dark:border-gray-700">
        <div class="px-4 mx-auto max-w-7xl sm:px-6 lg:px-8">
            <div class="flex justify-between h-16">
                <div class="flex">
                    <div class="flex items-center shrink-0 font-bold dark:text-white">Gozon</div>
                    <div class="hidden sm:ml-6 sm:flex sm:space-x-8">

                        <a href="/dashboard/orders"
                           class={navClass('/dashboard/orders', page.url.pathname)}> My Orders </a>

                        <a href="/dashboard/create"
                           class={navClass('/dashboard/create', page.url.pathname)}> New Order </a>

                        <a href="/dashboard/deposit"
                           class={navClass('/dashboard/deposit', page.url.pathname)}> Deposit </a>

                    </div>
                </div>
                <div class="flex items-center">
                    <span class="mr-4 text-sm text-gray-500 dark:text-gray-400">ID: {$userId}</span>
                    <button on:click={handleLogout}
                            class="text-sm text-red-600 hover:text-red-900 dark:text-red-400 dark:hover:text-red-300">
                        Logout
                    </button>
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