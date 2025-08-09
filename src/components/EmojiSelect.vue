<script async setup lang="ts">
import { invoke } from "@tauri-apps/api/core";

const emojis = await invoke("get_possible_emoji");

async function moodCheck(mood: string) {
    await invoke("invoke_mood", { mood });
}
</script>

<template>
    <div class="emoji-grid">
        <button
            v-for="value in emojis"
            :key="value"
            @click="moodCheck(value)"
            class="emoji-button"
        >
            {{ value }}
        </button>
    </div>
</template>

<style scoped>
.emoji-grid {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(80px, 1fr));
    gap: 16px;
    max-width: 400px;
    margin: 0 auto;
    padding: 20px;
}

.emoji-button {
    aspect-ratio: 1;
    border-radius: 20px;
    border: none;
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(10px);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
    cursor: pointer;
    font-size: 2.5rem;
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    overflow: hidden;
}

.emoji-button:hover {
    transform: translateY(-4px) scale(1.05);
    box-shadow: 0 16px 48px rgba(0, 0, 0, 0.2);
    background: rgba(255, 255, 255, 0.95);
}

.emoji-button:active {
    transform: translateY(-2px) scale(1.02);
    box-shadow: 0 8px 24px rgba(0, 0, 0, 0.15);
}

.emoji-button::before {
    content: "";
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(
        90deg,
        transparent,
        rgba(255, 255, 255, 0.4),
        transparent
    );
    transition: left 0.5s;
}

.emoji-button:hover::before {
    left: 100%;
}

@media (max-width: 480px) {
    .emoji-grid {
        grid-template-columns: repeat(auto-fit, minmax(70px, 1fr));
        gap: 12px;
        padding: 15px;
    }

    .emoji-button {
        font-size: 2rem;
        border-radius: 16px;
    }
}

@media (prefers-color-scheme: dark) {
    .emoji-button {
        background: rgba(52, 73, 94, 0.8);
        color: #ecf0f1;
    }

    .emoji-button:hover {
        background: rgba(52, 73, 94, 0.9);
    }
}
</style>
