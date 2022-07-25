<script lang="ts">
import { defineComponent } from 'vue'
import Gamepad from 'vue-material-design-icons/GamepadSquare.vue'
import ACircle from 'vue-material-design-icons/AlphaACircle.vue'
import BCircle from 'vue-material-design-icons/AlphaBCircle.vue'

export default defineComponent({
  name: 'Footer',
  components: {
    Gamepad,
    ACircle,
    BCircle,
  }
})
</script>

<template>
  <div class="wrapper">
    <hr />
    <div class="items-wrapper">
      <gamepad :size="70" />
      <div class="row">
        <div class="row">
          <a-circle :size="35" />
          <space horizontal="5" />
          <h2>Start</h2>
        </div>
        <space horizontal="20" />
        <div class="row">
          <b-circle :size="35" />
          <space horizontal="5" />
          <h2>Back</h2>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.items-wrapper {
  display: flex;
  padding: 0 20px;
  align-items: center;
  justify-content: space-between;
}

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
</style>
