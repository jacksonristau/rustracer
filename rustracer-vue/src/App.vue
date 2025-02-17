<script setup>
  import { ref } from 'vue'
  import { scene } from './assets/default_scene'
  const image_url = ref("");

  scene['spheres'] = [{
    center:{x:0.0,y:0.0,z:-8.0,w:1.0},
    radius:2.0,
    material_index:0
  }];

  async function render() {
    const request = new Request("/api/render", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        "Accept": "image/jpeg",
      },
      body: JSON.stringify(scene),
    });
    const response = await fetch(request);
    const image = await response.blob();
    image_url.value = URL.createObjectURL(image);
  }

</script>

<template>
  <div class="container">
    <div class="sidebar">
      <h2>Scene</h2>
      <Card>
        <template #title>Sphere</template>
        <template #content>
          <
        </template>
      </Card>
    </div>
    <div class="main">
      <h1>raytracer</h1>
      <div class="image_box">
        <Image :src="image_url" />
      </div>
      <div class="footer">
        <button @click="render()">send</button>
      </div>
    </div>
  </div>
  
</template>

<style scoped>

.container {
  display: flex;
  flex-direction: row;
  width: 100vw;
  height: 100vh;
}
.image_box {
  border: 2px solid #ffffff;
}
.main{
  width: 100%;
  display: flex;
  flex-direction: column;
  justify-content: space-between;
}
.sidebar{
  height: 100vh;
  width: 30vw;  
  text-align: left;
  background-color: #2b2b2b;
  padding: 0 1rem;
}
.footer{
  width: 100%;
  height: 10vh;
  background-color: #2b2b2b;
}
</style>
