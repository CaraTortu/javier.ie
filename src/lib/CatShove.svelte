<script lang="ts">
    import { scale } from "svelte/transition";
    import { onMount } from "svelte";
    import { MousePointer2, Pointer } from "@lucide/svelte";

    let showPaw = false;
    let kickPaw = false;
    let timer: number | null = null;
    let isOverInteractive = false; // Track if cursor is over button or link

    let cursorX = 0;
    let cursorY = 0;
    let fakeCursorX = 0;
    let fakeCursorY = 0;
    let animateCursor = false;

    const cornerSize = 200;
    const delayMs = 1000;

    function isInBottomLeftCorner(x: number, y: number) {
        const withinX = x < cornerSize;
        const withinY = y > window.innerHeight - cornerSize;
        return withinX && withinY;
    }

    function moveCursorRight() {
        let startTime: number;
        const startX = fakeCursorX;
        const endX = startX + 200;

        const duration = 600;

        const animate = (timestamp: number) => {
            if (!startTime) startTime = timestamp;
            const elapsed = timestamp - startTime;

            const progress = Math.min(elapsed / duration, 1);
            const easeInOut = 0.5 * (1 - Math.cos(Math.PI * progress));

            fakeCursorX = startX + (endX - startX) * easeInOut;

            if (progress < 1) {
                requestAnimationFrame(animate);
            } else {
                animateCursor = false;
                showPaw = false;
                kickPaw = false;
                timer = null;
            }
        };

        animateCursor = true;
        kickPaw = true;
        setTimeout(() => {
            requestAnimationFrame(animate);
        }, 400); // Duration of the kick animation
    }

    onMount(() => {
        const handleMouseMove = (e: MouseEvent) => {
            cursorX = e.clientX;
            cursorY = e.clientY;

            if (!animateCursor) {
                fakeCursorX = cursorX;
                fakeCursorY = cursorY;
            }

            // Check if cursor is over a button or link
            const target = e.target as HTMLElement;
            isOverInteractive = target.closest("a, button") !== null;

            if (isInBottomLeftCorner(cursorX, cursorY)) {
                if (!timer) {
                    timer = setTimeout(() => {
                        showPaw = true;
                        moveCursorRight();
                    }, delayMs);
                }
            } else {
                if (timer) {
                    clearTimeout(timer);
                    timer = null;
                }
            }
        };

        window.addEventListener("mousemove", handleMouseMove);
        return () => window.removeEventListener("mousemove", handleMouseMove);
    });
</script>

<!-- Paw appears and kicks -->
{#if showPaw}
    <div
        class="cat-paw"
        class:kick={kickPaw}
        transition:scale={{ duration: 200 }}
    ></div>
{/if}

<!-- Fake cursor with conditional icon -->
<div class="fake-cursor" style="top: {fakeCursorY}px; left: {fakeCursorX}px;">
    {#if isOverInteractive}
        <Pointer fill="black" />
    {:else}
        <MousePointer2 fill="black" />
    {/if}
</div>

<style>
    .fake-cursor {
        position: fixed;
        width: 20px;
        height: 20px;
        border-radius: 50%;
        z-index: 999;
        pointer-events: none;
        transition: transform 0.05s;
    }

    @media (pointer: coarse) {
        .fake-cursor {
            display: none; /* Hide fake cursor on touch devices */
        }
    }

    .cat-paw {
        position: fixed;
        bottom: 0px;
        left: -140px;
        width: 185px;
        height: 200px;
        background: url("/catpaw.webp") no-repeat center center;
        background-size: contain;
        z-index: 1000;
        pointer-events: none;
        transform-origin: bottom;
    }

    .cat-paw.kick {
        animation: kick-paw 0.8s linear;
    }

    @keyframes kick-paw {
        0% {
            transform: rotate(-90deg);
        }
        100% {
            transform: rotate(150deg);
        }
    }
</style>
