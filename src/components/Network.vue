<script lang="ts">
import { defineComponent } from 'vue'
import WifiOnIcon from 'vue-material-design-icons/Wifi.vue'
import WifiOffIcon from 'vue-material-design-icons/WifiOff.vue'

export default defineComponent({
  name: 'Network',
  components: {
    WifiOnIcon,
    WifiOffIcon
  },
  data() {
    return {
      isConnected: true,
    }
  },
})
</script>

<template>
  <wifi-on-icon v-if="isConnected" :size="30" />
  <wifi-off-icon v-else :size="30" />
</template>
