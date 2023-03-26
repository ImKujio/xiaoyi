<template>
  <div class="flex-col">
    <div style="height: 32px;display: flex;flex-direction: row;align-items: center">
      <svg-icon style="width: 16px;height: 16px;margin:0 8px" name="logo"/>
      <span>小译设置</span>
    </div>
    <div data-tauri-drag-region style="height: 32px; width: 100%;position: absolute;top: 0"/>
    <button  class="btn-rect inactive" style="position:absolute;top:4px;right:4px;padding: 6px 12px" v-click="async () => {await appWindow.close()}">
      <svg-icon style="width: 16px;height: 16px" name="close"/>
    </button>
    <div style="position: absolute;top:0;left: 50%;transform:translateX(-50%);display: flex;flex-direction: row;align-items: center">
      <button class="btn-rect left trans-tab" :class="{active:active===0,inactive:active!==0}" v-click="() => active=0">基础设置</button>
      <button class="btn-rect left trans-tab" :class="{active:active===1,inactive:active!==1}" v-click="() => active=1">翻译接口</button>
      <button class="btn-rect left trans-tab" :class="{active:active===2,inactive:active!==2}" v-click="() => active=2">OCR接口</button>
      <button class="btn-rect left trans-tab" :class="{active:active===3,inactive:active!==3}" v-click="() => active=3">关于</button>
    </div>
    <div class="flex-col">
      123
    </div>
  </div>
</template>

<script setup>
import {ref} from "vue";
import {appWindow} from "@tauri-apps/api/window";

const active = ref(0)

</script>

<style scoped>
button.active {
  font-weight: bold;
  color: var(--pColor);
}

button.inactive {
  color: #727272;
}

.trans-tab{
  transition: color,font-size 0.2s linear;
}

</style>