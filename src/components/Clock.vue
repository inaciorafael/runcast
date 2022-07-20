<script lang="ts">
import { defineComponent } from 'vue'
import dayjs from 'dayjs'

export default defineComponent({
  name: 'Clock',
  data() {
    return {
      time: '',
      interval: 0,
      period: '',
    }
  },
  created() {
    this.interval = setInterval(() => {
      this.time = dayjs().format('HH:mm')
      this.period = dayjs().format('A')
    }, 1000)
  },
  beforeDestroy() {
    clearInterval(this.interval)
  },
  computed: {
    getCurrentTime() {
      return this.time
    },
    getCurrentPeriod() {
      return this.period
    }
  },
})
</script>

<template>
  <div class="clock-wrapper">
    <h1>{{ getCurrentTime }}</h1>
    <h2 class="period">{{ getCurrentPeriod }}</h2>
  </div>
</template>

<style scoped>
.clock-wrapper {
  display: flex;

  flex-direction: row;
  justify-content: center;
  align-items: flex-end;
}

.period {
  opacity: 0.8;
}
</style>
