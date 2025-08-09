<script async setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

const emojis = await invoke("get_possible_emoji");

async function moodCheck(mood: string) {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  await invoke("invoke_mood", { mood });
}
</script>

<template>
    <button v-for="value in emojis" :key="value" @click="moodCheck(value)">
        {{ value }}
    </button>
</template>