<script setup>
  import { ref, defineEmits } from 'vue';
  const emit = defineEmits(['updatedScene'])
  const background_color = ref({"r":51,"g":51,"b":51});
</script>

<template>
  <Card>
    <template #title>Scene</template>
    <template #content>
      <h3 class="pb-2 font-bold">Background</h3>
      <div class="flex flex-wrap mt-2">
        <label for="color" class="mr-2"> Color: </label> 
        <ColorPicker @value-change="$emit('updatedScene', background_color)" input-id="color" v-model="background_color" format="rgb"></ColorPicker>
        <p class="ml-2">{{ background_color }}</p>
      </div>
    </template>
  </Card>
</template>

<style scoped>

</style>