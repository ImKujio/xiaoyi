<template>
  <div>
    <div>{{ src }}</div>
    <div>{{ dst }}</div>
  </div>
</template>

<script setup>
import {listen} from "@tauri-apps/api/event"
import {appWindow} from '@tauri-apps/api/window';
import {onMounted, onUnmounted, reactive, ref} from "vue";
import baidu from "./api/baidu.js";

const events = reactive({})
const src = ref("")
const dst = ref("")
onMounted(async () => {
  events.focus = await listen("tauri://focus", (e) => {
    console.log("focus t", e)
  })
  events.blur = await listen("tauri://blur", async (e) => {
    console.log("blur event:", e);
    await appWindow.hide();
  })
  events.translate = await listen("translate", (e) => {
    src.value = e.payload
    if (src.value.trim() === "") return
    dst.value = "翻译中..."
    baidu(src.value.trim()).then(value => {
      dst.value = value.trans_result.map(d => d.dst).join("\n")
    }).catch(reason => dst.value = reason)
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
