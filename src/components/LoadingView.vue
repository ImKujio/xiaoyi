<template>
  <div v-if="loading" class="loading-bar-container">
    <div class="loading-bar" :style="{width: progress + '%'}"></div>
  </div>
</template>

<script setup>
import {ref, watchEffect} from 'vue';

const loading = ref(false);
const progress = ref(0);

const startLoading = () => {
  loading.value = true;
  progress.value = 0;

  const timer = setInterval(() => {
    progress.value += Math.random() * 10;

    if (progress.value >= 100) {
      clearInterval(timer);
      progress.value = 100;
      setTimeout(() => {
        loading.value = false;
      }, 300);
    }
  }, 200);
};

watchEffect(() => {
  if (loading.value) {
    document.body.style.overflow = 'hidden';
  } else {
    document.body.style.overflow = '';
  }
});

</script>

<style>
.loading-bar-container {
  position: fixed;
  top: 0;
  left: 0;
  width: 100%;
  height: 2px;
}

.loading-bar {
  height: 100%;
  background-color: #007aff;
}
</style>
