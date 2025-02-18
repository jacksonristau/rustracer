<script setup>
  import { ref } from 'vue'
  import { scene } from './assets/default_scene'
  import ObjectList from './components/ObjectList.vue';
  import SceneEditor from './components/SceneEditor.vue';

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

  async function update_config(spheres, lights, materials){
    scene['spheres'] = spheres;
    scene['lights'] = lights;
    scene['materials'] = materials;
    await render();
  }

  async function update_scene(background_color){
    let norm_color = {
        "r": background_color.r / 255,
        "g": background_color.g / 255,
        "b": background_color.b / 255,
    }
    scene['bkg_color'] = norm_color;
    await render();
  }

</script>

<template>
  <div class="sidebar">
    <h1 class="mb-2 text-center">objects</h1>
    <ObjectList @updated="update_config" />
  </div>
  <div class="main">
    <h1 class="mb-auto">raytracer</h1>
    <Image class="border-2 border-solid mb-auto" :src="image_url" />
    <!-- <div class="footer">
      <Button @click="render()">send</Button>
    </div> -->
  </div>
  <div class="sidebar">
    <h1 class="mb-2 text-center">camera</h1>
    <SceneEditor @updatedScene="update_scene"/>
  </div>
</template>

<style scoped>
  .main{
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    width: 100%;
  }
  .sidebar{
    height: 100vh;
    width: 40rem;
    text-align: left; 
    background-color: #2b2b2b;
    padding: 0 1rem;
    overflow-y: auto;
  }
  .footer{
    width: 100%;
    height: 10vh;
    background-color: #2b2b2b;
  }
</style>
