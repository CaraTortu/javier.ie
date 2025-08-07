<script lang="ts">
    import { scrollTo, scrollRef } from "svelte-scrolling";
    import { ArrowDown, ArrowUpLeft } from "@lucide/svelte";
    import NavBar from "./lib/NavBar.svelte";
    import Separator from "./lib/Separator.svelte";
    import Wave from "./lib/Wave.svelte";
    import CatShove from "./lib/CatShove.svelte";

    type CardItem = {
        name: string;
        description: string;
        tags: string[];
    };

    const whatIDo: CardItem[] = [
        {
            name: "🕵️ Pentesting",
            description:
                "I perform penetration tests on web applications, APIs, and networks to identify vulnerabilities and help secure systems.",
            tags: ["Linux", "Windows", "Web", "Forensics", "Rev"],
        },
        {
            name: "🖥️ Development",
            description:
                "I develop web applications and APIs using modern technologies.",
            tags: ["Typescript", "React", "Next.js", "tRPC", "Go", "Rust"],
        },
        {
            name: "📚 Learning",
            description:
                "I'm always learning new things, whether it's a new technology, a new programming language, or a new skill.",
            tags: ["AI", "Physics", "Math", "Hardware"],
        },
        {
            name: "🎮 Skills",
            description:
                "I have experience with a variety of tools and technologies that help me in my work and personal projects, as well as social skills to go along with it.",
            tags: [
                "Leadership",
                "Communication",
                "Teamwork",
                "Problem Solving",
                "Time Management",
                "Adaptability",
            ],
        },
    ];

    const projects: (CardItem & { url: string })[] = [
        {
            name: "LC Computer Science",
            description:
                "A website to help students with their leaving certificate copmuter science subject.",
            tags: ["React", "Next.js", "tRPC", "PostgreSQL", "BetterAuth"],
            url: "https://github.com/CaraTortu/lccomputerscience",
        },
        {
            name: "GP Booking system",
            description:
                "A booking system for a GP practice that allows patients to book appointments online and receive reminders, as well as allowing doctors to manage their appointments.",
            tags: ["React", "Next.js", "tRPC", "PostgreSQL", "BetterAuth"],
            url: "https://github.com/CaraTortu/appointment_system",
        },
        {
            name: "QuoteLearner",
            description:
                "A website for students to learn quotes for their exams.",
            tags: ["React", "Next.js", "tRPC", "PostgreSQL", "BetterAuth"],
            url: "https://github.com/CaraTortu/quotelearner",
        },
        {
            name: "Sentiment Analyser",
            description:
                "An application powered by a custom AI model that uses sentiment analysis to predict relationships between people through email conversations.",
            tags: [
                "TensorFlow",
                "PostgreSQL",
                "Rust",
                "Neo4J",
                "Python",
                "Next.js",
            ],
            url: "https://github.com/CaraTortu/SentimentAnalyser",
        },
    ];
</script>

<main
    class="flex flex-col min-h-screen bg-black text-white items-center select-none"
>
    <NavBar />
    <CatShove />
    <section
        class="h-svh flex flex-col items-center justify-center gap-3 px-4 text-center"
    >
        <h1 class="text-4xl md:text-5xl font-bold"><Wave /> Hi, I'm Javier</h1>
        <p class="italic text-lg md:text-2xl text-gray-300">
            I'm a software engineer and cyber security researcher based in
            Ireland.
        </p>
        <button
            use:scrollTo={"whatIDo"}
            class="rounded-full border-2 border-white hover:border-orange-300 px-4 py-2 flex gap-2 items-center justify-center group mt-4 hover:shadow-md hover:translate-y-[-0.25rem] shadow-orange-300 transition-all duration-200"
        >
            <ArrowDown
                class="group-hover:animate-pulse size-5 group-hover:text-orange-300 transition-all duration-200"
            />
            <span>Learn more</span>
        </button>
    </section>
    <Separator />
    <section
        use:scrollRef={"whatIDo"}
        class="min-h-svh flex flex-col items-center justify-center gap-10 px-8 py-8 pt-22"
    >
        <h1 class="text-4xl">🔨 What I do</h1>
        <div class="grid grid-cols-1 md:grid-cols-2 max-w-5xl gap-8">
            {#each whatIDo as item}
                <div
                    class="p-4 border border-gray-600 rounded-lg hover:border-orange-300 transition-all duration-200 hover:shadow-md hover:shadow-orange-300 hover:scale-[102%] flex flex-col gap-2 size-fit"
                >
                    <h2 class="text-xl font-semibold">{item.name}</h2>
                    <p class="text-gray-300">{item.description}</p>
                    <div class="mt-2 flex flex-wrap gap-2">
                        {#each item.tags as tag}
                            <span
                                class="bg-gray-700 text-gray-300 px-2 py-1 rounded-full text-sm"
                                >{tag}</span
                            >
                        {/each}
                    </div>
                </div>
            {/each}
        </div>
    </section>
    <Separator />
    <section
        class="min-h-svh flex flex-col items-center justify-center gap-10 px-8 py-8 pt-22"
    >
        <h1 class="text-4xl">📂 Projects</h1>
        <div class="grid grid-cols-1 md:grid-cols-2 max-w-5xl gap-8">
            {#each projects as project}
                <a
                    href={project.url}
                    class="p-4 border border-gray-600 rounded-lg hover:border-orange-300 transition-all duration-200 hover:shadow-md hover:shadow-orange-300 hover:scale-[102%] flex flex-col gap-2"
                >
                    <h2 class="text-xl font-semibold flex gap-1">
                        <ArrowUpLeft class="size-5" />{project.name}
                    </h2>
                    <p class="text-gray-300">{project.description}</p>
                    <div class="mt-2 flex flex-wrap gap-2">
                        {#each project.tags as tag}
                            <span
                                class="bg-gray-700 text-gray-300 px-2 py-1 rounded-full text-sm"
                                >{tag}</span
                            >
                        {/each}
                    </div>
                </a>
            {/each}
        </div>
    </section>
</main>
