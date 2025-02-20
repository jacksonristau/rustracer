<script setup>
  import { ref, defineEmits, defineProps, watch, computed } from 'vue';
  const emit = defineEmits(['updated']);
  const props = defineProps(['clear_scene']);
  const bkg_color = ref({"r":51,"g":51,"b":51});
  const hfov = ref(60);
  const eye_pos = ref({"x":0.0, "y":0.0, "z":0.0, "w":1.0});
  const view_dir = ref({"x":0.0, "y":0.0, "z":-1.0, "w":0.0});

  const bkg = computed(()=>{
    return `rgb(${bkg_color.value.r}, ${bkg_color.value.g}, ${bkg_color.value.b})`
  });

  watch(() => props.clear_scene, (newVal) => {
    if (newVal) {
      bkg_color.value = {"r":51,"g":51,"b":51};
      hfov.value = 60;
      eye_pos.value = {"x":0.0, "y":0.0, "z":0.0, "w":1.0};
      view_dir.value = {"x":0.0, "y":0.0, "z":-1.0, "w":0.0};
      emit('updated', bkg_color.value, hfov.value, eye_pos.value, view_dir.value);
    }
  });
</script>

<template>
  <Card>
    <template #title><h3 class="font-bold text-2xl">scene</h3></template>
    <template #content>
      <h3 class="font-bold">camera position</h3>
      <div class="flex p-0 gap-2">
        <div>
          <label for="x-pos"> x: </label>
          <InputNumber @value-change="$emit('updated', bkg_color, hfov, eye_pos, view_dir)" showButtons size="small"  v-model="eye_pos.x" :max-fraction-digits="2" input-id="x-pos" :step="0.1" fluid/>
        </div>
        <div>
          <label for="y-pos" class=""> y: </label>
          <InputNumber @value-change="$emit('updated', bkg_color, hfov, eye_pos, view_dir)" showButtons size="small"  v-model="eye_pos.y" :max-fraction-digits="2" input-id="y-pos" :step="0.1" fluid/>
        </div>
        <div>
          <label for="z-pos"> z: </label>
          <InputNumber @value-change="$emit('updated', bkg_color, hfov, eye_pos, view_dir)" showButtons size="small" v-model="eye_pos.z" :max-fraction-digits="2" input-id="z-pos" :step="0.1" fluid/>
        </div>
      </div>
      <h3 class="mt-4 font-bold">look direction</h3>
      <div class="flex p-0 gap-2">
        <div>
          <label for="x-pos"> x: </label>
          <InputNumber @value-change="$emit('updated', bkg_color, hfov, eye_pos, view_dir)" showButtons size="small"  v-model="view_dir.x" :max-fraction-digits="2" input-id="x-pos" :step="0.1" fluid/>
        </div>
        <div>
          <label for="y-pos" class=""> y: </label>
          <InputNumber @value-change="$emit('updated', bkg_color, hfov, eye_pos, view_dir)" showButtons size="small"  v-model="view_dir.y" :max-fraction-digits="2" input-id="y-pos" :step="0.1" fluid/>
        </div>
        <div>
          <label for="z-pos"> z: </label>
          <InputNumber @value-change="$emit('updated', bkg_color, hfov, eye_pos, view_dir)" showButtons size="small"  v-model="view_dir.z" :max-fraction-digits="2" input-id="z-pos" :step="0.1" fluid/>
        </div>
      </div>
      <div class="flex flex-wrap mt-2">
        <label for="color" class="mr-2"> background color: </label>
        <ColorPicker @value-change="$emit('updated', bkg_color, hfov, eye_pos, view_dir)" input-id="color" v-model="bkg_color" format="rgb"></ColorPicker>
        <p class="ml-2">{{ bkg }}</p>
      </div>
      <div class="flex flex-wrap mt-2">
        <label for="fov" class="mr-2"> fov: </label>
        <InputNumber @value-change="$emit('updated', bkg_color, hfov, eye_pos, view_dir)" input-id="fov" v-model="hfov" size="small" style="width: 6rem" fluid />
      </div>
    </template>
  </Card>
</template>

<style scoped>

</style>