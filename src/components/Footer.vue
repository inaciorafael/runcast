<script lang="ts">
import { defineComponent } from 'vue'

export default defineComponent({
  name: 'Footer',
})
</script>

<template>
  <div class="wrapper">
    <hr />
    <h1>Footer</h1>
  </div>
</template>

<style scoped>
.wrapper {
  grid-area: footer;
}

.lineContainer {
  display: flex;
  align-items: center;
  justify-content: center;
  margin: 0;
  padding: 0;
}

hr {
  width: 97%;
  margin-left: auto;
  margin-right: auto;
  height: 1px;
  background-color: black;
  opacity: 0.7;
}


.line {
  height: 0.7px;
  background-color: black;
  width: 98%;
  margin: 0;
  padding: 0;
}
</style>
