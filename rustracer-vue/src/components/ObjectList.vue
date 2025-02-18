<script setup>
  import { ref, defineEmits } from 'vue'
  import { material } from '../assets/default_material';
  import SphereEditor from './SphereEditor.vue';
  import LightEditor from './LightEditor.vue';

  const spheres = ref([]);
  const lights = ref([]);
  const materials = ref([]);
  var num_objects = 0;

  const emit = defineEmits(['updated']);

  function add_sphere() {
    spheres.value.push({
      center: {x:0.0, y:0.0, z:-20.0, w:1.0},
      radius: 2.0,
      material_index: num_objects
    });
    materials.value.push({
      diffuse:{r:255,g:0,b:0},
      specular:{r:255,g:255,b:255},
      k_a:0.2,k_d:0.6,k_s:0.2,
      alpha:1.0,
      index_of_refraction:0.0,
      n_val: 10,
      texture:null
    });
    num_objects += 1;
    emit('updated', spheres.value, lights.value, materials.value);
  }

  function add_light() {
    lights.value.push({
      v: {x:0.0, y:0.0, z:-8.0, w:1.0},
      attenuation: [1.0, 0.0, 0.0],
      i: 1.0
    });
    emit('updated', spheres.value, lights.value, materials.value);
  }

  function update_spheres(sphere, material, index) {
    spheres.value[index] = sphere;
    materials.value[sphere.material_index] = material;
    emit('updated', spheres.value, lights.value, materials.value);
  }

  function update_lights(light, index) {
    lights.value[index] = light;
    emit('updated', spheres.value, lights.value, materials.value);
  }
</script>

<template>
  <div class="object_list gap-4">
    <SphereEditor @updated="update_spheres" v-for="(s, i) in spheres" :key="i" :sphere="s" :index="i" :material="materials[i]"/>
    <LightEditor @updated="update_lights" v-for="(l, i) in lights" :key="i" :light="l" :index="i"/>
    <div class="flex gap-4">
      <Button @click="add_sphere()" class="mt-2 mb-4 grow">add sphere</Button>
      <Button @click="add_light()" class="mt-2 mb-4 grow">add light</Button>
    </div>
    
  </div>
</template>

<style scoped>
.object_list {
    display: flex;
    flex-direction: column;
    justify-content: start;
    width: 100%;
}
</style>
