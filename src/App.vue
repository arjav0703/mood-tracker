<script setup lang="ts">
import EmojiSelect from "./components/EmojiSelect.vue";
import MoodGraph from "./components/MoodGraph.vue";
</script>

<script lang="ts">
import NavBar from "./components/NavBar.vue";
export default {
    components: {
        NavBar,
        MoodGraph,
        EmojiSelect,
    },
    data() {
        return {
            currentTab: "Home",
        };
    },
    methods: {
        handleTabSelection(tab: string) {
            this.currentTab = tab;
        },
    },
};
</script>

<template>
    <div class="app">
        <NavBar :currentTab="currentTab" @tab-selected="handleTabSelection" />

        <div class="content-wrapper">
            <transition name="slide-fade" mode="out-in">
                <div
                    v-if="currentTab === 'Home'"
                    key="home"
                    class="tab-content"
                >
                    <div class="welcome-section"></div>
                    <div class="emoji-container">
                        <Suspense>
                            <template #default>
                                <EmojiSelect />
                            </template>
                            <template #fallback>
                                <div class="loading">⏳</div>
                            </template>
                        </Suspense>
                    </div>
                </div>

                <div
                    v-else-if="currentTab === 'Graphs'"
                    key="graphs"
                    class="tab-content"
                >
                    <MoodGraph />
                </div>

                <div
                    v-else-if="currentTab === 'contact'"
                    key="contact"
                    class="tab-content"
                >
                    <div class="coming-soon">📞</div>
                </div>
            </transition>
        </div>
    </div>
</template>

<style scoped>
.app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}

.content-wrapper {
    flex: 1;
    padding: 20px;
    padding-bottom: 120px; /* Space for navbar */
    overflow-x: hidden;
}

.tab-content {
    max-width: 500px;
    margin: 0 auto;
    padding: 20px 0;
}

.welcome-section {
    text-align: center;
    margin-bottom: 40px;
    padding: 0 20px;
}

.emoji-container {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 20px;
    padding: 20px;
}

.loading {
    text-align: center;
    color: #7f8c8d;
    font-size: 3rem;
    padding: 20px;
}

.coming-soon {
    text-align: center;
    padding: 60px 20px;
    font-size: 4rem;
}

/* Smooth transitions */
.slide-fade-enter-active,
.slide-fade-leave-active {
    transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
}

.slide-fade-enter-from {
    opacity: 0;
    transform: translateX(30px);
}

.slide-fade-leave-to {
    opacity: 0;
    transform: translateX(-30px);
}

@media (max-width: 480px) {
    .content-wrapper {
        padding: 15px;
        padding-bottom: 120px;
    }

    .welcome-section {
        margin-bottom: 30px;
        padding: 0 10px;
    }
}
</style>
<style>
:root {
    font-family:
        "SF Pro Display",
        -apple-system,
        BlinkMacSystemFont,
        "Segoe UI",
        Roboto,
        Helvetica,
        Arial,
        sans-serif;
    font-size: 16px;
    line-height: 1.6;
    font-weight: 400;

    color: #2c3e50;
    background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
    min-height: 100vh;

    font-synthesis: none;
    text-rendering: optimizeLegibility;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
    -webkit-text-size-adjust: 100%;
}

* {
    box-sizing: border-box;
}

body {
    margin: 0;
    padding: 0;
    overflow-x: hidden;
}

button {
    border-radius: 16px;
    border: none;
    padding: 12px 20px;
    font-size: 3rem;
    font-weight: 500;
    font-family: inherit;
    color: #2c3e50;
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(10px);
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
    cursor: pointer;
    border: 2px solid transparent;
    min-width: 80px;
    min-height: 80px;
    display: inline-flex;
    align-items: center;
    justify-content: center;
}

button:hover {
    transform: translateY(-2px);
    box-shadow: 0 12px 40px rgba(0, 0, 0, 0.15);
    background: rgba(255, 255, 255, 0.95);
    border-color: rgba(74, 144, 226, 0.3);
}

button:active {
    transform: translateY(0px);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.1);
    background: rgba(255, 255, 255, 0.8);
}

button:focus {
    outline: none;
    border-color: #4a90e2;
}

/* Smooth scrolling */
html {
    scroll-behavior: smooth;
}

/* Loading animation */
@keyframes pulse {
    0%,
    100% {
        opacity: 0.6;
    }
    50% {
        opacity: 1;
    }
}

.loading {
    animation: pulse 2s infinite;
}

@media (prefers-color-scheme: dark) {
    :root {
        color: #ecf0f1;
        background: linear-gradient(135deg, #2c3e50 0%, #34495e 100%);
    }

    .app-title {
        color: #ecf0f1;
    }

    .subtitle {
        color: #bdc3c7;
    }

    button {
        color: #ecf0f1;
        background: rgba(52, 73, 94, 0.8);
        backdrop-filter: blur(10px);
    }

    button:hover {
        background: rgba(52, 73, 94, 0.9);
        border-color: rgba(74, 144, 226, 0.4);
    }

    button:active {
        background: rgba(52, 73, 94, 0.7);
    }
}

@media (max-width: 480px) {
    button {
        font-size: 2.5rem;
        min-width: 70px;
        min-height: 70px;
        padding: 10px 16px;
    }
}
</style>
