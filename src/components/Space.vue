<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'Space',
  props: {
    vertical: String,
    horizontal: String
  },
  computed: {
    getSpaceStyle() {
      return `width: ${this.horizontal || 0}px; height: ${this.vertical || 0}px;`
    }
  }
})
</script>

<template>
  <div :style="getSpaceStyle"></div>
</template>
