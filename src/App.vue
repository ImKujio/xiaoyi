<template>
  <div class="container">
    {{ transText }}
  </div>
</template>

<script setup>
import {listen} from "@tauri-apps/api/event"
import { appWindow } from '@tauri-apps/api/window';
import {onMounted, onUnmounted, reactive, ref} from "vue";

const events = reactive({})
const transText = ref("")

onMounted(async () => {
  events.focus = await listen("tauri://focus", (e) => {
    console.log("focus event", e)
  })
  events.blur = await listen("tauri://blur", async (e) => {
    console.log("blur event:", e);
    await appWindow.hide();
  })
  events.translate = await listen("translate",(e) => {
    transText.value = e.payload
  })
})

onUnmounted(async () => {
  events.focus()
  events.blur()
  events.translate()
})
</script>

<style scoped>
</style>
