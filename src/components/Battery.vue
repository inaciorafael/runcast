<script>
import { defineComponent } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'

export default defineComponent({
  name: 'Battery',
  data() {
    return {
      percentage: '',
      interval: 0,
    }
  },
  computed: {
    getBatteryPercentage() {
      return this.percentage
    }
  },
  created() {
    this.interval = setInterval(async () => {
      this.percentage = await invoke('get_battery_percentage')
    }, 1000)
  },
  beforeDestroy() {
    clearInterval(this.interval)
  },
})
</script>

<template>
  <div>
    <v-icon icon="fa-solid fa-user-secret" />
    <h2>{{ getBatteryPercentage }}</h2>
  </div>
</template>
