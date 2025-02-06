<template>
  <div class="flex flex-col w-full h-[100vh]">
    <NavFlyout />
    <main class="px-6">
      <slot />
    </main>
  </div>
</template>

<script setup lang="ts">
useColorMode()
</script>
