<div class="hand-container">
    <span class="hand">👋</span>
</div>

<style>
    .hand-container {
        display: inline-block;
    }

    .hand {
        display: inline-block;
        transform-origin: bottom center;
        animation: waveAnimation 0.6s ease-in-out 4 alternate forwards;
    }

    @keyframes waveAnimation {
        0% {
            transform: rotate(0deg);
        }
        50% {
            transform: rotate(10deg);
        }
        100% {
            transform: rotate(0deg);
        }
    }
</style>
