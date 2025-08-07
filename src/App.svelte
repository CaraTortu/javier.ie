<script lang="ts">
    import { scrollTo, scrollRef } from "svelte-scrolling";
    import Saos from "saos";
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
                "A website to help students with their leaving certificate computer science subject.",
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
    class="flex flex-col min-h-screen bg-gradient-to-b from-gray-900 to-black text-white items-center select-none font-sans"
>
    <div class="sticky top-0 z-50 w-full">
        <NavBar />
    </div>
    <CatShove />
    <section
        class="h-svh flex flex-col items-center justify-center gap-6 px-4 md:px-8 text-center"
    >
        <h1
            class="text-4xl md:text-6xl font-extrabold tracking-tight animate-fade-in"
        >
            <Wave /> Hi, I'm Javier
        </h1>
        <p
            class="italic text-lg md:text-2xl text-gray-200 max-w-2xl leading-relaxed"
        >
            I'm a software engineer and cyber security researcher based in
            Ireland.
        </p>
        <button
            use:scrollTo={"whatIDo"}
            class="rounded-full bg-orange-500 text-white px-6 py-3 flex gap-3 items-center justify-center group mt-6 hover:bg-orange-600 hover:shadow-lg hover:shadow-orange-500/50 hover:-translate-y-1 transition-all duration-300 text-lg font-medium"
        >
            <ArrowDown
                class="group-hover:animate-bounce size-6 group-hover:text-white transition-all duration-300"
            />
            <span>Learn More</span>
        </button>
    </section>
    <Separator />
    <section
        use:scrollRef={"whatIDo"}
        class="min-h-svh flex flex-col items-center justify-center gap-12 px-4 md:px-8 py-12"
    >
        <h1
            class="text-4xl md:text-5xl font-bold tracking-tight animate-slide-up"
        >
            🔨 What I Do
        </h1>
        <div
            class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 max-w-6xl gap-6"
        >
            {#each whatIDo as item, index}
                <Saos
                    animation={"fade-in 1s cubic-bezier(0.175, 0.885, 0.320, 1.275) both"}
                    animation_out={"fade-out 1s cubic-bezier(0.550, 0.085, 0.680, 0.530) both"}
                    top={200}
                    bottom={200}
                >
                    <div
                        class="p-6 bg-gray-800/50 backdrop-blur-sm border border-gray-600 rounded-xl hover:border-orange-400 transition-all duration-300 hover:shadow-xl hover:shadow-orange-500/30 hover:scale-[102%] flex flex-col gap-4"
                        style="animation-delay: {index * 100}ms"
                    >
                        <h2 class="text-2xl font-semibold text-orange-400">
                            {item.name}
                        </h2>
                        <p class="text-gray-200 leading-relaxed">
                            {item.description}
                        </p>
                        <div class="mt-4 flex flex-wrap gap-2">
                            {#each item.tags as tag}
                                <span
                                    class="bg-orange-500/20 text-orange-300 px-3 py-1 rounded-full text-sm font-medium hover:bg-orange-500/30 transition-colors duration-200"
                                    >{tag}</span
                                >
                            {/each}
                        </div>
                    </div>
                </Saos>
            {/each}
        </div>
    </section>
    <Separator />
    <section
        class="min-h-svh flex flex-col items-center justify-center gap-12 px-4 md:px-8 py-12"
    >
        <h1
            class="text-4xl md:text-5xl font-bold tracking-tight animate-slide-up"
        >
            📂 Projects
        </h1>
        <div
            class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 max-w-6xl gap-6"
        >
            {#each projects as project, index}
                <Saos
                    animation={"fade-in 1s cubic-bezier(0.175, 0.885, 0.320, 1.275) both"}
                    animation_out={"fade-out 1s cubic-bezier(0.550, 0.085, 0.680, 0.530) both"}
                    top={200}
                    bottom={200}
                >
                    <a
                        href={project.url}
                        target="_blank"
                        rel="noopener noreferrer"
                        class="p-6 bg-gray-800/50 backdrop-blur-sm border border-gray-600 rounded-xl hover:border-orange-400 transition-all duration-300 hover:shadow-xl hover:shadow-orange-500/30 hover:scale-[102%] flex flex-col gap-4"
                        style="animation-delay: {index * 100}ms"
                    >
                        <h2
                            class="text-2xl font-semibold text-orange-400 flex gap-2 items-center"
                        >
                            <ArrowUpLeft
                                class="size-6 text-orange-400"
                            />{project.name}
                        </h2>
                        <p class="text-gray-200 leading-relaxed">
                            {project.description}
                        </p>
                        <div class="mt-4 flex flex-wrap gap-2">
                            {#each project.tags as tag}
                                <span
                                    class="bg-orange-500/20 text-orange-300 px-3 py-1 rounded-full text-sm font-medium hover:bg-orange-500/30 transition-colors duration-200"
                                    >{tag}</span
                                >
                            {/each}
                        </div>
                    </a>
                </Saos>
            {/each}
        </div>
    </section>
</main>

<style>
    @keyframes -global-fade-in {
        from {
            opacity: 0;
            transform: translateY(20px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }

    @keyframes -global-fade-out {
        from {
            opacity: 1;
            transform: translateY(0px);
        }
        to {
            opacity: 0;
            transform: translateY(-30px);
        }
    }

    @keyframes fade-in {
        from {
            opacity: 0;
            transform: translateY(20px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
    @keyframes slide-up {
        from {
            opacity: 0;
            transform: translateY(30px);
        }
        to {
            opacity: 1;
            transform: translateY(0);
        }
    }
    .animate-fade-in {
        animation: fade-in 0.8s ease-out forwards;
    }
    .animate-slide-up {
        animation: slide-up 0.8s ease-out forwards;
    }
</style>
