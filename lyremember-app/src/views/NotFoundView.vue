<template>
  <div class="min-h-screen flex items-center justify-center bg-deep p-4">
    <div class="text-center space-y-6">
      <div class="text-8xl font-bold text-gold/20">404</div>
      <h1 class="text-2xl font-bold text-[#F5F0EB]">{{ $t('notFound.title') }}</h1>
      <p class="text-[#8A82A0] max-w-md">
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
