<script setup>
  import { ref } from 'vue'
  import { scene } from './assets/default_scene'
  import ObjectEditor from './components/ObjectEditor.vue';
  const image_url = ref("");

  scene['spheres'] = [];

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
  <div class="sidebar">
    <ObjectEditor />
  </div>
  <div class="main">
    <h1>raytracer</h1>
    <div class="image_box">
      <Image :src="image_url" />
    </div>
    <div class="footer">
      <Button @click="render()">send</Button>
    </div>
  </div>
</template>

<style scoped>
.image_box {
  border: 2px solid #ffffff;
}
.main{
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  width: 100%;
}
.sidebar{
  height: 100vh;
  width: 25rem;
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
