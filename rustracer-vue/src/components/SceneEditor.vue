<script setup>
  import { ref, defineEmits, defineProps, watch } from 'vue';
  const emit = defineEmits(['updated']);
  const props = defineProps(['clear_scene']);
  const bkg_color = ref({"r":51,"g":51,"b":51});
  const hfov = ref(60);

  watch(() => props.clear_scene, (newVal) => {
    if (newVal) {
      bkg_color.value = {"r":51,"g":51,"b":51};
      hfov.value = 60;
      emit('updated', bkg_color.value, hfov.value);
    }
  });
</script>

<template>
  <Card>
    <template #title><h3 class="font-bold text-2xl">scene</h3></template>
    <template #content>
      <div class="flex flex-wrap mt-2">
        <label for="color" class="mr-2"> background: </label> 
        <ColorPicker @value-change="$emit('updated', bkg_color, hfov)" input-id="color" v-model="bkg_color" format="rgb"></ColorPicker>
        <p class="ml-2">{{ bkg_color }}</p>
      </div>
      <div class="flex flex-wrap mt-2">
        <label for="fov" class="mr-2"> fov: </label> 
        <InputNumber @value-change="$emit('updated', bkg_color, hfov)" input-id="fov" v-model="hfov" size="small" style="width: 6rem" fluid />
      </div>
    </template>
  </Card>
</template>

<style scoped>

</style>