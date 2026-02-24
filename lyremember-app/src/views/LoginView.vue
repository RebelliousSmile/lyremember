<template>
  <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-indigo-500 to-purple-600 dark:from-indigo-900 dark:to-purple-900 p-4">
    <Card className="w-full max-w-md">
      <template #header>
        <h2 class="text-2xl font-bold text-center">Login to LyRemember</h2>
      </template>
      
      <form @submit.prevent="handleSubmit" class="space-y-4">
        <Alert
          v-model="showError"
          type="error"
          closable
        >
          {{ authStore.error }}
        </Alert>
        
        <Input
          v-model="form.username"
          label="Username"
          type="text"
          placeholder="Enter your username"
          required
        />
        
        <Input
          v-model="form.password"
          label="Password"
          type="password"
          placeholder="Enter your password"
          required
        />
        
        <Button
          type="submit"
          variant="primary"
          size="lg"
          className="w-full"
          :loading="authStore.loading"
        >
          Login
        </Button>
        
        <p class="text-center text-sm text-gray-600 dark:text-gray-400">
          Don't have an account?
          <router-link to="/register" class="text-indigo-600 dark:text-indigo-400 hover:underline">
            Register
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

watch(() => authStore.error, (error) => {
  showError.value = !!error;
});

async function handleSubmit() {
  try {
    await authStore.login(form.value.username, form.value.password);
    router.push('/dashboard');
  } catch (err) {
    // Error is already set in store
    console.error('Login failed:', err);
  }
}
</script>
