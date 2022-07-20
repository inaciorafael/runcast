<script lang="ts">
import { defineComponent } from 'vue'

import Clock from '../components/Clock.vue'
import Battery from '../components/Battery.vue'

export default defineComponent({
  name: 'HomeHeader',
  components: {
    Clock,
    Battery,
  },
})
</script>

<template>
  <div class="wrapper">
    <div class="items-container">
      <h1>options</h1>
    </div>
    <div class="items-container">
      <Clock />
      <div style="width: 30px;" />
      <v-icon name="eye"></v-icon>
      <Battery />
    </div>
  </div>
</template>

<style scoped>
.wrapper {
  grid-area: header;

  display: flex;
  flex-direction: row;
  align-items: center;
  justify-content: space-between;
  padding: 0 30px;
}

.items-container {
  display: flex;
  flex-direction: row;
  align-items: center;
}
</style>
