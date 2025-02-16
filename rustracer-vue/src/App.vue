<script setup>
  import { ref } from 'vue'
  import { scene } from './assets/default_scene'
  const count = ref(0)  
  scene['spheres'] = [{
    center:{x:0.0,y:0.0,z:-8.0,w:1.0},
    radius:2.0,
    material_index:0
  }]
  const request = new Request("/api/render", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(scene),
  });
  var result = fetch(request)
  console.log(result);
</script>

<template>
  <div class="">
    <div class="main">
      <h1 class="text-3xl font-bold underline">raytracer</h1>
      <input type="text">
      <button>send</button>
      <img>
    </div>
  </div>
  
</template>

<style scoped>
</style>
