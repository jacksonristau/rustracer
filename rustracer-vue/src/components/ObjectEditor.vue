<script setup>
  import { ref } from 'vue'
  const pos = ref({x: 0.0, y: 0.0, z: 0.0});
  const radius = ref(1.0);
</script>

<template>
  <Card>
    <template #title>Sphere</template>
    <template #content>
      <h3 class="pb-2">Position</h3>
      <div class="flex flex-wrap p-0">
        <div class="flex-auto">
          <label for="x-pos" class="font-bold"> x </label>
          <InputNumber size="small" style="width: 3rem" v-model="pos.x" :max-fraction-digits="2" input-id="x-pos" fluid/>
        </div>
        <div class="flex-auto">
          <label for="y-pos" class="font-bold"> y </label>
          <InputNumber size="small" style="width: 3rem" v-model="pos.y" :max-fraction-digits="2" input-id="y-pos" fluid/>
        </div>
        <div class="flex-auto">
          <label for="z-pos" class="font-bold"> z </label>
          <InputNumber size="small" style="width: 3rem" v-model="pos.z" :max-fraction-digits="2" input-id="z-pos" fluid/>
        </div>
      </div>
      <div class="flex flex-wrap mt-2">
        <div class="flex-auto">
          <label for="radius" class="font-bold"> Radius </label>
          <InputNumber size="small" style="width: 6rem" v-model="radius" :max-fraction-digits="2" fluid/>
        </div>
      </div>
    </template>
  </Card>
</template>

<style scoped>
</style>