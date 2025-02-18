<script setup>
  import { ref, defineProps, defineEmits, computed } from 'vue'
  const props = defineProps(['sphere', 'material', 'index']);
  const emit = defineEmits(['updated']);
  const sphere = ref(props.sphere);
  const material = ref(props.material);
  const step = ref(1)

</script>

<template>
  <Card>
    <template #title><h3 class="font-bold text-2xl">sphere</h3></template>
    <template #content>
      <h3 class="pb-2 font-bold">position</h3>
      <div class="flex flex-wrap p-0 gap-2">
        <div>
          <label for="x-pos"> x: </label>
          <InputNumber @value-change="$emit('updated', sphere, material, index)" size="small" style="width: 3rem" v-model="sphere.center.x" :max-fraction-digits="2" input-id="x-pos" :step fluid/>
        </div>
        <div >
          <label for="y-pos" class=""> y: </label>
          <InputNumber @value-change="$emit('updated', sphere, material, index)" size="small" style="width: 3rem" v-model="sphere.center.y" :max-fraction-digits="2" input-id="y-pos" :step fluid/>
        </div>
        <div>
          <label for="z-pos"> z: </label>
          <InputNumber @value-change="$emit('updated', sphere, material, index)" size="small" style="width: 3rem" v-model="sphere.center.z" :max-fraction-digits="2" input-id="z-pos" :step fluid/>
        </div>
      </div>
      <div class="flex flex-wrap mt-2">
        <div class="flex-auto">
          <label for="radius"> radius: </label>
          <InputNumber @value-change="$emit('updated', sphere, material, index)" size="small" style="width: 6rem" v-model="sphere.radius" :max-fraction-digits="2" input-id="radius" :step fluid/>
        </div>
      </div>
      <h3 class="mt-4 font-bold">material</h3>
      <div class="flex flex-wrap mt-2">
        <label for="color" class="mr-2"> diffuse: </label> 
        <ColorPicker @value-change="$emit('updated', sphere, material, index)" input-id="color" v-model="material.diffuse" format="rgb"></ColorPicker>
        <p class="ml-2">{{ material.diffuse }}</p>
      </div>
      <div class="flex flex-wrap mt-2">
        <label for="color" class="mr-2"> specular: </label> 
        <ColorPicker @value-change="$emit('updated', sphere, material, index)" input-id="color" v-model="material.specular" format="rgb"></ColorPicker>
        <p class="ml-2">{{ material.specular }}</p>
      </div>
      <div class="flex flex-wrap p-0 gap-2 mt-2">
        <div>
          <label for="k-a">ka: </label>
          <InputNumber @value-change="$emit('updated', sphere, material, index)" :min="0" :max="1" size="small" style="width: 3rem" v-model="material.k_a" :max-fraction-digits="1" input-id="k-a" :step="0.1" fluid/>
        </div>
        <div>
          <label for="k-d">kd: </label>
          <InputNumber @value-change="$emit('updated', sphere, material, index)" :min="0" :max="1" size="small" style="width: 3rem" v-model="material.k_d" :max-fraction-digits="1" input-id="k-d" :step="0.1" fluid/>
        </div>
        <div>
          <label for="k-s">ks: </label>
          <InputNumber @value-change="$emit('updated', sphere, material, index)" :min="0" :max="1" size="small" style="width: 3rem" v-model="material.k_s" :max-fraction-digits="1" input-id="k-s" :step="0.1" fluid/>
        </div>
        <div>
          <label for="n_val">n: </label>
          <InputNumber @value-change="$emit('updated', sphere, material, index)" :min="10" :max="999" size="small" style="width: 3rem" v-model="material.n_val" :max-fraction-digits="1" input-id="n_val" :step="0.1" fluid/>
        </div>
      </div>
    </template>
  </Card>
</template>

<style scoped>
</style>