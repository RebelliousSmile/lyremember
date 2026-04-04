<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 p-4">
    <div class="text-center space-y-6">
      <div class="text-8xl font-bold text-indigo-200 dark:text-indigo-900">404</div>
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">Page not found</h1>
      <p class="text-gray-600 dark:text-gray-400 max-w-md">
        The page you're looking for doesn't exist or has been moved.
      </p>
      <div class="flex justify-center gap-3">
        <Button variant="primary" @click="$router.push('/dashboard')">
          <Home :size="18" />
          Dashboard
        </Button>
        <Button variant="ghost" @click="$router.back()">
          <ArrowLeft :size="18" />
          Go Back
        </Button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Home, ArrowLeft } from 'lucide-vue-next';
import Button from '../components/ui/Button.vue';
</script>
