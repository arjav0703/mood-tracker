<template>
    <div class="mood-graph-container">
        <div class="graph-header"></div>
        <div v-if="moodEntries.length === 0" class="no-data">
            <div class="no-data-icon">📊</div>
            <button @click="generateSampleData" class="sample-data-btn">
                📈
            </button>
        </div>
        <svg
            v-else
            class="mood-graph"
            :viewBox="`0 0 ${svgWidth} ${svgHeight}`"
        >
            <!-- Grid -->
            <g class="grid-lines">
                <line
                    v-for="i in 5"
                    :key="`grid-${i}`"
                    :x1="padding"
                    :y1="padding + (i - 1) * (graphHeight / 4)"
                    :x2="svgWidth - padding"
                    :y2="padding + (i - 1) * (graphHeight / 4)"
                    stroke="#e0e0e0"
                    stroke-width="1"
                    opacity="0.5"
                />
            </g>

            <!-- labels -->
            <g class="mood-labels">
                <text
                    v-for="(label, index) in moodLabels"
                    :key="`label-${index}`"
                    :x="padding - 10"
                    :y="padding + index * (graphHeight / 4) + 5"
                    text-anchor="end"
                    font-size="24"
                    fill="#666"
                >
                    {{ label }}
                </text>
            </g>

            <path
                :d="linePath"
                stroke="#4a90e2"
                stroke-width="3"
                fill="none"
                stroke-linecap="round"
                stroke-linejoin="round"
            />

            <g class="data-points">
                <circle
                    v-for="(point, index) in dataPoints"
                    :key="`point-${index}`"
                    :cx="point.x"
                    :cy="point.y"
                    r="6"
                    :fill="getMoodColor(point.mood)"
                    stroke="white"
                    stroke-width="2"
                />
            </g>

            <!-- Emoji labels at data points -->
            <g class="emoji-labels">
                <text
                    v-for="(point, index) in dataPoints"
                    :key="`emoji-${index}`"
                    :x="point.x"
                    :y="point.y - 15"
                    text-anchor="middle"
                    font-size="16"
                >
                    {{ getMoodEmoji(point.mood) }}
                </text>
            </g>
        </svg>
    </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onActivated, computed } from "vue";
import { invoke } from "@tauri-apps/api/core";

interface MoodEntry {
    mood: string;
    timestamp: string;
}

const moodEntries = ref<MoodEntry[]>([]);
const svgWidth = 400;
const svgHeight = 300;
const padding = 60;
const graphWidth = svgWidth - 2 * padding;
const graphHeight = svgHeight - 2 * padding;

const moodLabels = ["😄", "😊", "😐", "😢", "😠"];

const moodValues: { [key: string]: number } = {
    Excited: 4,
    Happy: 3,
    Neutral: 2,
    Sad: 1,
    Angry: 0,
};

const dataPoints = computed(() => {
    if (moodEntries.value.length === 0) return [];

    return moodEntries.value.map((entry, index) => {
        const x =
            padding +
            (index * graphWidth) / Math.max(moodEntries.value.length - 1, 1);
        const moodValue = moodValues[entry.mood] || 2;
        const y = padding + (4 - moodValue) * (graphHeight / 4);

        return {
            x,
            y,
            mood: entry.mood,
            timestamp: entry.timestamp,
        };
    });
});

const linePath = computed(() => {
    if (dataPoints.value.length === 0) return "";

    let path = `M ${dataPoints.value[0].x} ${dataPoints.value[0].y}`;

    for (let i = 1; i < dataPoints.value.length; i++) {
        const current = dataPoints.value[i];
        const previous = dataPoints.value[i - 1];

        // Create smooth curves between points
        const controlX1 = previous.x + (current.x - previous.x) / 3;
        const controlY1 = previous.y;
        const controlX2 = current.x - (current.x - previous.x) / 3;
        const controlY2 = current.y;

        path += ` C ${controlX1} ${controlY1}, ${controlX2} ${controlY2}, ${current.x} ${current.y}`;
    }

    return path;
});

const getMoodColor = (mood: string): string => {
    const colors: { [key: string]: string } = {
        Excited: "#ff6b6b",
        Happy: "#4ecdc4",
        Neutral: "#45b7d1",
        Sad: "#96ceb4",
        Angry: "#fd7e14",
    };
    return colors[mood] || "#45b7d1";
};

const getMoodEmoji = (mood: string): string => {
    const emojis: { [key: string]: string } = {
        Excited: "😄",
        Happy: "😊",
        Neutral: "😐",
        Sad: "😢",
        Angry: "😠",
    };
    return emojis[mood] || "😐";
};

const loadTodaysMoods = async () => {
    try {
        const moods = await invoke<MoodEntry[]>("get_todays_moods");
        moodEntries.value = moods.sort(
            (a, b) =>
                new Date(a.timestamp).getTime() -
                new Date(b.timestamp).getTime(),
        );
    } catch (error) {
        moodEntries.value = [];
    }
};

const generateSampleData = async () => {
    try {
        await invoke("generate_sample_data");
        await loadTodaysMoods();
    } catch (error) {
        // Silent fail
    }
};

onMounted(() => {
    loadTodaysMoods();
});

onActivated(() => {
    loadTodaysMoods();
});
</script>

<style scoped>
.mood-graph-container {
    padding: 20px;
    max-width: 500px;
    margin: 0 auto;
}

.graph-header {
    text-align: center;
    margin-bottom: 30px;
    padding: 0 20px;
}

.no-data {
    text-align: center;
    padding: 60px 20px;
    color: #7f8c8d;
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(10px);
    border-radius: 20px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
}

.no-data-icon {
    font-size: 3rem;
    margin-bottom: 20px;
    opacity: 0.7;
}

.sample-data-btn {
    background: rgba(74, 144, 226, 0.1);
    border: 2px solid #4a90e2;
    color: #4a90e2;
    padding: 16px 20px;
    border-radius: 16px;
    font-size: 1.5rem;
    cursor: pointer;
    transition: all 0.3s ease;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    margin-top: 20px;
    min-width: 80px;
    min-height: 60px;
}

.sample-data-btn:hover {
    background: #4a90e2;
    transform: translateY(-2px) scale(1.05);
    box-shadow: 0 8px 24px rgba(74, 144, 226, 0.3);
}

.mood-graph {
    width: 100%;
    height: auto;
    max-width: 450px;
    margin: 0 auto;
    display: block;
    background: rgba(255, 255, 255, 0.9);
    backdrop-filter: blur(10px);
    border-radius: 20px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.1);
    border: 1px solid rgba(255, 255, 255, 0.2);
}

.grid-lines line {
    pointer-events: none;
    opacity: 0.3;
}

.mood-labels text {
    pointer-events: none;
    font-weight: 500;
}

.data-points circle {
    transition: all 0.3s cubic-bezier(0.4, 0, 0.2, 1);
    cursor: pointer;
    filter: drop-shadow(0 2px 4px rgba(0, 0, 0, 0.1));
}

.data-points circle:hover {
    r: 8;
    filter: drop-shadow(0 4px 8px rgba(0, 0, 0, 0.2));
    transform-origin: center;
}

.emoji-labels text {
    pointer-events: none;
    filter: drop-shadow(0 1px 2px rgba(0, 0, 0, 0.1));
}

@media (max-width: 480px) {
    .mood-graph-container {
        padding: 15px;
    }

    .no-data {
        padding: 40px 15px;
    }

    .no-data-icon {
        font-size: 2.5rem;
    }
}

@media (prefers-color-scheme: dark) {
    .no-data {
        color: #bdc3c7;
        background: rgba(52, 73, 94, 0.8);
    }

    .sample-data-btn {
        background: rgba(116, 185, 255, 0.2);
        border-color: #74b9ff;
        color: #74b9ff;
    }

    .sample-data-btn:hover {
        background: #74b9ff;
    }

    .mood-graph {
        background: rgba(52, 73, 94, 0.8);
        border-color: rgba(255, 255, 255, 0.1);
    }

    .grid-lines line {
        stroke: #444;
        opacity: 0.4;
    }

    .mood-labels text {
        fill: #bdc3c7;
    }
}
</style>
