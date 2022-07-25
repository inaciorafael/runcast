<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'Option',
})
</script>

<template>
  <div class="option-container">
    <slot></slot>
  </div>
</template>

<style scoped>
.option-container {
  background-color: #0002;
  width: 55px;
  height: 55px;
  border-radius: 100%;
  border: solid 2px #0001;
  display: flex;
  align-items: center;
  justify-content: center;
  box-shadow: rgba(0, 0, 0, 0.15) 0px 5px 15px 0px;
}
</style>
