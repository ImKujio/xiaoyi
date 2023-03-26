<template>
  <svg >
    <use :xlink:href="`#icon-${props.name}`"></use>
  </svg>
</template>

<script setup>
const props = defineProps({
  name: String,
})
</script>

<style scoped>
.svg-icon {
  /* width: 1em;
   height: 1em;*/
  fill: currentColor;
  vertical-align: middle;
  overflow: hidden;
}
</style>