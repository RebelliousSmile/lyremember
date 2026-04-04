<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900 p-4">
    <div class="text-center space-y-6">
      <div class="text-8xl font-bold text-indigo-200 dark:text-indigo-900">404</div>
      <h1 class="text-2xl font-bold text-gray-900 dark:text-white">{{ $t('notFound.title') }}</h1>
      <p class="text-gray-600 dark:text-gray-400 max-w-md">
        {{ $t('notFound.description') }}
      </p>
      <div class="flex justify-center gap-3">
        <Button variant="primary" @click="$router.push('/dashboard')">
          <Home :size="18" />
          {{ $t('notFound.dashboard') }}
        </Button>
        <Button variant="ghost" @click="goBack">
          <ArrowLeft :size="18" />
          {{ $t('notFound.goBack') }}
        </Button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router';
import { Home, ArrowLeft } from 'lucide-vue-next';
import Button from '../components/ui/Button.vue';

const router = useRouter();

function goBack() {
  if (window.history.length > 1) {
    router.back();
  } else {
    router.push('/dashboard');
  }
}
</script>
