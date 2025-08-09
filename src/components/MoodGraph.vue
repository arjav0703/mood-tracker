<template>
  <div class="mood-graph-container">
    <div class="graph-header">
      <h2>Today's Mood Pattern</h2>
    </div>
    <div v-if="moodEntries.length === 0" class="no-data">
      <p>No mood entries for today</p>
      <p>Start tracking your mood! 😊</p>
    </div>
    <svg v-else class="mood-graph" :viewBox="`0 0 ${svgWidth} ${svgHeight}`">
      <!-- Grid lines -->
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

      <!-- Mood level labels -->
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

      <!-- Line path -->
      <path
        :d="linePath"
        stroke="#4a90e2"
        stroke-width="3"
        fill="none"
        stroke-linecap="round"
        stroke-linejoin="round"
      />

      <!-- Data points -->
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

      <!-- Time labels -->
      <g class="time-labels">
        <text
          v-for="(point, index) in dataPoints"
          :key="`time-${index}`"
          :x="point.x"
          :y="svgHeight - padding + 20"
          text-anchor="middle"
          font-size="12"
          fill="#666"
        >
          {{ formatTime(point.timestamp) }}
        </text>
      </g>
    </svg>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';

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

const moodLabels = ['😄', '😊', '😐', '😢', '😠'];

const moodValues: { [key: string]: number } = {
  'Excited': 4,
  'Happy': 3,
  'Neutral': 2,
  'Sad': 1,
  'Angry': 0
};

const dataPoints = computed(() => {
  if (moodEntries.value.length === 0) return [];

  return moodEntries.value.map((entry, index) => {
    const x = padding + (index * graphWidth) / Math.max(moodEntries.value.length - 1, 1);
    const moodValue = moodValues[entry.mood] || 2;
    const y = padding + (4 - moodValue) * (graphHeight / 4);

    return {
      x,
      y,
      mood: entry.mood,
      timestamp: entry.timestamp
    };
  });
});

const linePath = computed(() => {
  if (dataPoints.value.length === 0) return '';

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
    'Excited': '#ff6b6b',
    'Happy': '#4ecdc4',
    'Neutral': '#45b7d1',
    'Sad': '#96ceb4',
    'Angry': '#fd7e14'
  };
  return colors[mood] || '#45b7d1';
};

const getMoodEmoji = (mood: string): string => {
  const emojis: { [key: string]: string } = {
    'Excited': '😄',
    'Happy': '😊',
    'Neutral': '😐',
    'Sad': '😢',
    'Angry': '😠'
  };
  return emojis[mood] || '😐';
};

const formatTime = (timestamp: string): string => {
  const date = new Date(timestamp);
  return date.toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit',
    hour12: false
  });
};

const loadTodaysMoods = async () => {
  try {
    const moods = await invoke<MoodEntry[]>('get_todays_moods');
    moodEntries.value = moods.sort((a, b) =>
      new Date(a.timestamp).getTime() - new Date(b.timestamp).getTime()
    );
  } catch (error) {
    console.error('Failed to load today\'s moods:', error);
    moodEntries.value = [];
  }
};

onMounted(() => {
  loadTodaysMoods();
});
</script>

<style scoped>
.mood-graph-container {
  padding: 20px;
  max-width: 100%;
  margin: 0 auto;
}

.graph-header {
  text-align: center;
  margin-bottom: 20px;
}

.graph-header h2 {
  color: #333;
  font-size: 1.5rem;
  margin: 0;
}

.no-data {
  text-align: center;
  padding: 40px;
  color: #666;
}

.no-data p {
  margin: 10px 0;
  font-size: 1.1rem;
}

.mood-graph {
  width: 100%;
  height: auto;
  max-width: 400px;
  margin: 0 auto;
  display: block;
  background: white;
  border-radius: 8px;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
}

.grid-lines line {
  pointer-events: none;
}

.mood-labels text {
  pointer-events: none;
}

.data-points circle {
  transition: r 0.2s ease;
  cursor: pointer;
}

.data-points circle:hover {
  r: 8;
}

.emoji-labels text {
  pointer-events: none;
}

.time-labels text {
  pointer-events: none;
}

@media (prefers-color-scheme: dark) {
  .graph-header h2 {
    color: #f6f6f6;
  }

  .no-data {
    color: #ccc;
  }

  .mood-graph {
    background: #1a1a1a;
  }

  .grid-lines line {
    stroke: #444;
  }

  .mood-labels text,
  .time-labels text {
    fill: #ccc;
  }
}
</style>
