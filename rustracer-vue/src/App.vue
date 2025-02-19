<script setup>
  import { ref, onMounted } from 'vue'
  import { scene } from './assets/default_scene'
  import ObjectList from './components/ObjectList.vue';
  import SceneEditor from './components/SceneEditor.vue';

  const image_url = ref("");
  const clear_scene = ref(false);
  let fetching = false;

  onMounted(async () => {
    await render();
  });

  async function render() {
    if (fetching) {
      return;
    }
    fetching = true;
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
    fetching = false;
  }

  async function update_config(spheres, lights, materials){
    scene['spheres'] = spheres;
    scene['lights'] = lights;
    scene['materials'] = materials;
    clear_scene.value = false;
    await render();
  }

  async function update_scene(bkg_color, hfov){
    let norm_color = {
        "r": bkg_color.r / 255,
        "g": bkg_color.g / 255,
        "b": bkg_color.b / 255,
    }
    scene['bkg_color'] = norm_color;
    scene['hfov'] = hfov;
    clear_scene.value = false;
    setTimeout(await render(), 250);
  }
  function clear() {
    console.log("clearing...")
    clear_scene.value = true;
  }
</script>

<template>
  <div class="sidebar">
    <h1 class="mb-2 text-center">objects</h1>
    <ObjectList @updated="update_config" :clear_scene />
  </div>
  <div class="main">
    <h1 class="mb-auto">raytracer</h1>
    <Image class="border-2 border-solid mb-auto" :src="image_url" />
  </div>
  <div class="sidebar">
    <h1 class="mb-2 text-center">camera</h1>
    <SceneEditor @updated="update_scene" :clear_scene />
    <Button label="reset scene" @click="clear()" class="mt-2 w-full" />
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
