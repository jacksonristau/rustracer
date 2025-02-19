<script setup>
  import { ref, defineProps, defineEmits } from 'vue'
  const props = defineProps(['light', 'index']);
  const emit = defineEmits(['updated', 'deleted']);
  const light = ref(props.light);
  const step = ref(1)
  
  function delete_light() {
    emit('deleted', props.index);
  }
</script>

<template>
  <Card>
    <template #title>
      <div class="flex justify-between mt-1">
        <h3 class="font-bold text-2xl">light</h3>
        <Button @click="delete_light" label="Delete" severity="danger" outlined />
      </div>
    </template>
    <template #content>
      <h3 class="pb-2 font-bold ">position</h3>
      <div class="flex flex-wrap p-0 gap-2">
        <div>
          <label for="x-pos"> x: </label>
          <InputNumber @value-change="$emit('updated', light, index)" showButtons size="small" style="width: 5rem" v-model="light.v.x" :max-fraction-digits="2" input-id="x-pos" :step fluid/>
        </div>
        <div>
          <label for="y-pos" class=""> y: </label>
          <InputNumber @value-change="$emit('updated', light, index)" showButtons size="small" style="width: 5rem" v-model="light.v.y" :max-fraction-digits="2" input-id="y-pos" :step fluid/>
        </div>
        <div>
          <label for="z-pos"> z: </label>
          <InputNumber @value-change="$emit('updated', light, index)" showButtons size="small" style="width: 5rem" v-model="light.v.z" :max-fraction-digits="2" input-id="z-pos" :step fluid/>
        </div>
      </div>
      <div class="flex flex-wrap mt-2">
        <div>
          <label for="intensity"> intensity: </label>
          <InputNumber @value-change="$emit('updated', light, index)" showButtons :min="0" :max="1" size="small" style="width: 6rem" v-model="light.i" :max-fraction-digits="2" input-id="intensity" :step="0.1" fluid/>
        </div>
      </div>
    </template>
  </Card>
</template>

<style scoped>
</style>