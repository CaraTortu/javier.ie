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
    class="flex fixed top-0 z-50 gap-6 items-center px-6 w-full h-14 border-b border-gray-600 select-none md:gap-10 md:px-12 backdrop-blur-md"
>
    <a href="/" class="hidden text-lg font-bold md:flex md:text-2xl">Javier Díaz</a>
    <p class="hidden text-2xl text-gray-500 md:flex">/</p>
    <div class="flex gap-6 justify-center items-center text-gray-300 grow md:grow-0">
        {#each links as link}
            <a
                href={link.href}
                class="transition-colors duration-200 hover:text-white"
            >
                {link.name}
            </a>
        {/each}
    </div>
</nav>
