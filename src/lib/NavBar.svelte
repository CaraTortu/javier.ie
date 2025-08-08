<script lang="ts">
    const links = [
        {
            name: "Home",
            href: "/",
        },

        {
            name: "Github",
            href: "https://github.com/CaraTortu",
        },
        {
            name: "LinkedIn",
            href: "https://www.linkedin.com/in/javier-díaz-094b33269",
        },
        {
            name: "Blog",
            href: "https://blog.javier.ie",
        },
    ];
</script>

<nav
    class="fixed top-0 w-full z-50 backdrop-blur-md border-b border-gray-600 h-14 flex gap-6 md:gap-10 items-center px-6 md:px-12 select-none"
>
    <a href="/" class="text-lg md:text-2xl font-bold">Javier Díaz</a>
    <p class="text-gray-500 text-2xl">/</p>
    <div class="flex gap-6 text-gray-300">
        {#each links as link}
            <a
                href={link.href}
                class=" hover:text-white transition-colors duration-200"
            >
                {link.name}
            </a>
        {/each}
    </div>
</nav>
