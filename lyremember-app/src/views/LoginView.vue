<template>
  <div class="min-h-screen flex items-center justify-center bg-deep p-4">
    <Card className="w-full max-w-md">
      <template #header>
        <h2 class="text-2xl font-bold text-center">{{ $t('auth.loginTitle') }}</h2>
      </template>

      <form @submit.prevent="handleSubmit" class="space-y-4">
        <Alert v-model="showError" type="error" closable>
          {{ authStore.error }}
        </Alert>

        <Input
          v-model="form.username"
          :label="$t('auth.username')"
          type="text"
          required
        />

        <Input
          v-model="form.password"
          :label="$t('auth.password')"
          type="password"
          required
        />

        <Button type="submit" variant="primary" size="lg" className="w-full" :loading="authStore.loading">
          {{ $t('auth.login') }}
        </Button>

        <div class="relative flex items-center my-2">
          <div class="flex-grow border-t border-deep-border"></div>
          <span class="mx-3 text-sm text-[#8A82A0]">{{ $t('auth.or') }}</span>
          <div class="flex-grow border-t border-deep-border"></div>
        </div>

        <Button
          type="button"
          variant="secondary"
          size="lg"
          className="w-full"
          :loading="guestLoading"
          @click="handleGuestLogin"
        >
          {{ $t('auth.continueAsGuest') }}
        </Button>

        <p class="text-center text-sm text-[#8A82A0]">
          {{ $t('auth.noAccount') }}
          <router-link to="/register" class="text-gold hover:underline">
            {{ $t('auth.register') }}
          </router-link>
        </p>
      </form>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '../stores/auth';
import Card from '../components/ui/Card.vue';
import Input from '../components/ui/Input.vue';
import Button from '../components/ui/Button.vue';
import Alert from '../components/ui/Alert.vue';
import type { LoginForm } from '../types';

const router = useRouter();
const authStore = useAuthStore();

const form = ref<LoginForm>({
  username: '',
  password: '',
});

const showError = ref(false);
const guestLoading = ref(false);

watch(() => authStore.error, (error) => {
  showError.value = !!error;
});

async function handleSubmit() {
  try {
    await authStore.login(form.value.username, form.value.password);
    router.push('/dashboard');
  } catch (err) {
    console.error('Login failed:', err);
  }
}

async function handleGuestLogin() {
  guestLoading.value = true;
  try {
    await authStore.loginAsGuest();
    router.push('/dashboard');
  } catch (err) {
    console.error('Guest login failed:', err);
  } finally {
    guestLoading.value = false;
  }
}
</script>
