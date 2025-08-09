<script lang="ts">
import { ArrowDown, ArrowUpLeft } from "@lucide/svelte";
import Saos from "saos";
import { scrollRef, scrollTo } from "svelte-scrolling";
import CatShove from "./lib/CatShove.svelte";
import NavBar from "./lib/NavBar.svelte";
import Separator from "./lib/Separator.svelte";
import Wave from "./lib/Wave.svelte";

type CardItem = {
    name: string;
    description: string;
    tags: string[];
};

const whatIDo: CardItem[] = [
    {
        name: "🕵 Pentesting",
        description:
            "I perform penetration tests on web applications, APIs, and networks to identify vulnerabilities and help secure systems.",
        tags: ["Linux", "Windows", "Web", "Forensics", "Rev"],
    },
    {
        name: "🖥 Development",
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
        description: "A website for students to learn quotes for their exams.",
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
    class="flex flex-col items-center min-h-screen font-sans text-white bg-gradient-to-b from-gray-900 to-black select-none"
>
    <div class="sticky top-0 z-50 w-full">
        <NavBar />
    </div>
    <CatShove />
    <section
        class="flex flex-col gap-6 justify-center items-center px-4 text-center md:px-8 h-svh"
    >
        <h1
            class="text-4xl font-extrabold tracking-tight md:text-6xl animate-fade-in"
        >
            <Wave /> Hi, I'm Javier
        </h1>
        <p
            class="max-w-2xl text-lg italic leading-relaxed text-gray-200 md:text-2xl"
        >
            I'm a software engineer and cyber security researcher based in
            Ireland.
        </p>
        <button
            use:scrollTo={"whatIDo"}
            class="flex gap-3 justify-center items-center py-3 px-6 mt-6 text-lg font-medium text-white bg-orange-500 rounded-full transition-all duration-300 hover:bg-orange-600 hover:shadow-lg hover:-translate-y-1 group hover:shadow-orange-500/50"
        >
            <ArrowDown
                class="transition-all duration-300 group-hover:text-white group-hover:animate-bounce size-6"
            />
            <span>Learn More</span>
        </button>
    </section>
    <Separator />
    <section
        use:scrollRef={"whatIDo"}
        class="flex flex-col gap-12 justify-center items-center py-12 px-4 md:px-8 min-h-svh"
    >
        <h1
            class="text-4xl font-bold tracking-tight md:text-5xl animate-slide-up"
        >
            🔨 What I Do
        </h1>
        <div
            class="grid grid-cols-1 gap-6 max-w-6xl sm:grid-cols-2 lg:grid-cols-3"
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
                        <p class="leading-relaxed text-gray-200">
                            {item.description}
                        </p>
                        <div class="flex flex-wrap gap-2 mt-4">
                            {#each item.tags as tag}
                                <span
                                    class="py-1 px-3 text-sm font-medium text-orange-300 rounded-full transition-colors duration-200 bg-orange-500/20 hover:bg-orange-500/30"
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
        class="flex flex-col gap-12 justify-center items-center py-12 px-4 md:px-8 min-h-svh"
    >
        <h1
            class="text-4xl font-bold tracking-tight md:text-5xl animate-slide-up"
        >
            📂 Projects
        </h1>
        <div
            class="grid grid-cols-1 gap-6 max-w-6xl sm:grid-cols-2 lg:grid-cols-3"
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
                            class="flex gap-2 items-center text-2xl font-semibold text-orange-400"
                        >
                            <ArrowUpLeft
                                class="text-orange-400 size-6"
                            />
                            <span>{project.name}</span>
                        </h2>
                        <p class="leading-relaxed text-gray-200">
                            {project.description}
                        </p>
                        <div class="flex flex-wrap gap-2 mt-4">
                            {#each project.tags as tag}
                                <span
                                    class="py-1 px-3 text-sm font-medium text-orange-300 rounded-full transition-colors duration-200 bg-orange-500/20 hover:bg-orange-500/30"
                                    >{tag}</span
                                >
                            {/each}
                        </div>
                    </a>
                </Saos>
            {/each}
        </div>
    </section>
    <Separator />
    <section
        class="flex flex-col gap-12 justify-center items-center py-12 px-4 md:px-8 min-h-svh"
    >
        <h1
            class="text-4xl font-bold tracking-tight md:text-5xl animate-slide-up"
        >
            ✉ Contact Me
        </h1>
        <div class="flex flex-col gap-4 items-center px-4 text-center text-gray-200">
            <p class="text-lg md:text-xl">
                Feel free to reach out to me via email at
                <a
                    href="mailto:contact@javier.ie"
                    class="text-orange-400 hover:underline"
                >
                    contact@javier.ie
                </a>
            </p>
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
