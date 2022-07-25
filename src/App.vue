<script setup lang="ts">
// This starter template is using Vue 3 <script setup> SFCs
// Check out https://vuejs.org/api/sfc-script-setup.html#script-setup
import Home from './pages/Home/Home.vue'
import Footer from './components/Footer.vue'
</script>

<template>
  <div class="app-wrapper">
    <Home />
    <Footer />
  </div>
</template>

<style>
@font-face {
  font-family: "pixel";
  src: local("pixel"),
    url(./assets/fonts/pixel.ttf) format("truetype");
}

@font-face {
  font-family: "roboto";
  src: local("roboto"),
    url(./assets/fonts/Roboto-Light.ttf.ttf) format("truetype");
}

:root {
  --primary: #36393f;
  --secondary: #2f3136;
  --tertiary: rgb(32, 34, 37);
  --quaternary: #292b2f;
  --quinary: #393d42;
  --senary: #828386;

  --white: #fff;
  --gray: #8a8c90;
  --chat-input: rgb(64, 68, 75);
  --symbol: #74777a;

  --notification: #f84a4b;
  --discord: #6e86d6;
  --mention-detail: #f9a839;
  --mention-message: #413f3f;

  --link: #5d80d6;

  --rocketseat: #6633cc;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

html,
body,
#app {
  height: 100vh;
  font-family: 'San Francisco Light', sans-serif;
  overflow-y: hidden;
  background-color: white;
}

h1 {
  font-size: 2rem;
}

h2 {
  font-size: 1.5rem;
}

h1,
h2,
span {
  font-family: roboto;
  font-weight: 300;
  margin: 0;
}

*,
button,
input {
  border: 0;
  outline: 0;
  margin: 0;
}

.row {
  display: flex;
  flex-direction: row;
}

button {
  cursor: pointer;
  background-color: transparent;
}

button:hover {
  opacity: 1;
}

li {
  list-style: none;
}

.app-wrapper {
  display: grid;

  grid-template-columns: 100%;
  grid-template-rows: 10% auto 10%;
  grid-template-areas:
    'header'
    'content'
    'footer';
  height: 100vh;
}
</style>
