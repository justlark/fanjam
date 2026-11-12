<script setup lang="ts">
const props = defineProps<{
  hideNav?: boolean;
}>();
</script>

<template>
  <div class="flex flex-col gap-6 p-6 min-h-screen">
    <header v-if="!props.hideNav" class="mx-auto">
      <nav clas="flex flex-col lg:flex-row gap-2">
        <RouterLink :to="{ name: 'home' }" class="flex flex-col gap-2">
          <div class="text-3xl font-black text-center">
            <span class="text-orange-400 dark:text-orange-300">Fan</span
            ><span class="text-sky-400 dark:text-sky-300">Jam</span>
          </div>
          <span>An event planning app for conventions.</span>
        </RouterLink>
      </nav>
    </header>
    <main>
      <slot />
    </main>
    <footer class="mt-auto flex flex-col items-center gap-1 text-muted-color">
      <small class="flex gap-2">
        <a class="text-link-sm" href="https://github.com/justlark/fanjam" target="_blank">
          Source Code
        </a>
        <span>•</span>
        <RouterLink class="text-link-sm" :to="{ name: 'privacy' }">Privacy Policy</RouterLink>
      </small>
      <small>© 2025-2026 Lark Aster</small>
    </footer>
  </div>
</template>
