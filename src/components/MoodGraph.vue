<template>
    <div class="mood-graph-container">
        <div class="graph-header">
            <button @click="resetData" class="reset-btn" title="Reset all data">
                🗑️
            </button>
        </div>
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
            <!-- Grid (now based on moodScale indices) -->
            <g class="grid-lines">
                <line
                    v-for="(mood, i) in moodScale"
                    :key="`grid-${mood}`"
                    :x1="padding"
                    :y1="getYForIndex(i)"
                    :x2="svgWidth - padding"
                    :y2="getYForIndex(i)"
                    stroke="#e0e0e0"
                    stroke-width="1"
                    opacity="0.5"
                />
            </g>

            <!-- Axis labels -->
            <g class="mood-labels">
                <text
                    v-for="(mood, i) in moodScale"
                    :key="`label-${mood}`"
                    :x="padding - 14"
                    :y="getYForIndex(i)"
                    text-anchor="end"
                    dominant-baseline="middle"
                    font-size="24"
                    fill="#666"
                >
                    {{ getMoodEmoji(mood) }}
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

const moodScale = ["Excited", "Happy", "Neutral", "Sad", "Angry"];

const moodIndexMap: Record<string, number> = moodScale.reduce(
    (acc, m, i) => ((acc[m] = i), acc),
    {} as Record<string, number>,
);

const steps = moodScale.length - 1;
const getYForIndex = (i: number) => padding + (i / steps) * graphHeight;
const getYForMood = (mood: string) =>
    getYForIndex(moodIndexMap[mood] ?? moodIndexMap["Neutral"]);

const dataPoints = computed(() => {
    if (moodEntries.value.length === 0) return [];

    return moodEntries.value.map((entry, index) => {
        const x =
            padding +
            (index * graphWidth) / Math.max(moodEntries.value.length - 1, 1);
        const y = getYForMood(entry.mood);

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

const emojiMap: Record<string, string> = {
    Excited: "😄",
    Happy: "😊",
    Neutral: "😐",
    Sad: "😢",
    Angry: "😠",
};
const getMoodEmoji = (mood: string) => emojiMap[mood] || "😐";

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

const resetData = async () => {
    if (confirm("?")) {
        try {
            await invoke("reset_store");
            await loadTodaysMoods();
        } catch (error) {
            console.error("Failed to reset data:", error);
        }
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
    display: flex;
    justify-content: flex-end;
    align-items: center;
    margin-bottom: 30px;
    padding: 0 20px;
}

.reset-btn {
    background: rgba(231, 76, 60, 0.1);
    border: 2px solid #e74c3c;
    color: #e74c3c;
    padding: 8px 16px;
    border-radius: 12px;
    font-size: 0.9rem;
    cursor: pointer;
    transition: all 0.3s ease;
    display: inline-flex;
    align-items: center;
    gap: 8px;
}

.reset-btn:hover {
    background: #e74c3c;
    color: white;
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(231, 76, 60, 0.3);
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
    font-size: 1.1rem;
    cursor: pointer;
    transition: all 0.3s ease;
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    margin-top: 20px;
    min-width: 120px;
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

    .reset-btn {
        background: rgba(255, 107, 107, 0.2);
        border-color: #ff6b6b;
        color: #ff6b6b;
    }

    .reset-btn:hover {
        background: #ff6b6b;
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
