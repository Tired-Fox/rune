<template>
  <div class="flex flex-col w-full h-[100vh]">
    <NavBar />
    <main class="py-4 px-6">
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
useColorMode()
</script>
