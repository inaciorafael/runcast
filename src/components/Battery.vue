<script lang="ts">
import { defineComponent } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
// import Battery from 'vue-material-design-icons/Battery.vue';
import Battery90 from 'vue-material-design-icons/Battery90.vue';
import Battery80 from 'vue-material-design-icons/Battery80.vue';
import Battery70 from 'vue-material-design-icons/Battery70.vue';
import Battery60 from 'vue-material-design-icons/Battery60.vue';
import Battery50 from 'vue-material-design-icons/Battery50.vue';
import Battery40 from 'vue-material-design-icons/Battery40.vue';
import Battery30 from 'vue-material-design-icons/Battery30.vue';
import Battery20 from 'vue-material-design-icons/Battery20.vue';
import Battery10 from 'vue-material-design-icons/Battery10.vue';

export default defineComponent({
  name: 'Battery',
  components: {
    // Battery,
    Battery10,
    Battery20,
    Battery30,
    Battery40,
    Battery50,
    Battery60,
    Battery70,
    Battery80,
    Battery90,
  },
  data() {
    return {
      percentage: '',
      interval: 0,
      mock_percentage: 10,
    }
  },
  computed: {
    getBatteryPercentage() {
      return this.percentage
    },
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
  <div class="battery-container">
    <battery-60 :size="30"></battery-60>
    <h2>{{ getBatteryPercentage }}</h2>
  </div>
</template>

<style scoped>
.battery-container {
  display: flex;
  align-items: center;
}
</style>
