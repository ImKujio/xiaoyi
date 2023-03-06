<template>
  <div style="display: flex;flex-direction: column;position: relative; overflow: hidden" >
    <div style="width: 100%;height: 100%;position: absolute; box-sizing: border-box; overflow: hidden;" :style="{padding:props.padding}">
      <slot></slot>
    </div>
  </div>
</template>

<script setup>
const props = defineProps({
  padding: {
    type: String,
    default: '0'
  }
})
</script>

<style scoped>
</style>