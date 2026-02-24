<template>
  <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-indigo-500 to-purple-600 dark:from-indigo-900 dark:to-purple-900 p-4">
    <Card className="w-full max-w-md">
      <template #header>
        <h2 class="text-2xl font-bold text-center">Create Account</h2>
      </template>
      
      <form @submit.prevent="handleSubmit" class="space-y-4">
        <Alert
          v-model="showError"
          type="error"
          closable
        >
          {{ error }}
        </Alert>
        
        <Input
          v-model="form.username"
          label="Username"
          type="text"
          placeholder="Choose a username"
          required
        />
        
        <Input
          v-model="form.email"
          label="Email"
          type="email"
          placeholder="Enter your email"
          required
        />
        
        <Input
          v-model="form.password"
          label="Password"
          type="password"
          placeholder="Choose a password"
          required
        />
        
        <Input
          v-model="form.confirmPassword"
          label="Confirm Password"
          type="password"
          placeholder="Confirm your password"
          required
          :error="passwordError"
        />
        
        <Button
          type="submit"
          variant="primary"
          size="lg"
          className="w-full"
          :loading="authStore.loading"
          :disabled="!!passwordError"
        >
          Register
        </Button>
        
        <p class="text-center text-sm text-gray-600 dark:text-gray-400">
          Already have an account?
          <router-link to="/login" class="text-indigo-600 dark:text-indigo-400 hover:underline">
            Login
          </router-link>
        </p>
      </form>
    </Card>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue';
import { useRouter } from 'vue-router';
import { useAuthStore } from '../stores/auth';
import Card from '../components/ui/Card.vue';
import Input from '../components/ui/Input.vue';
import Button from '../components/ui/Button.vue';
import Alert from '../components/ui/Alert.vue';
import type { RegisterForm } from '../types';

const router = useRouter();
const authStore = useAuthStore();

const form = ref<RegisterForm>({
  username: '',
  email: '',
  password: '',
  confirmPassword: '',
});

const showError = ref(false);
const error = ref('');

const passwordError = computed(() => {
  if (!form.value.confirmPassword) return '';
  if (form.value.password !== form.value.confirmPassword) {
    return 'Passwords do not match';
  }
  return '';
});

watch(() => authStore.error, (err) => {
  error.value = err || '';
  showError.value = !!err;
});

async function handleSubmit() {
  if (passwordError.value) {
    error.value = passwordError.value;
    showError.value = true;
    return;
  }

  try {
    await authStore.register(form.value.username, form.value.email, form.value.password);
    // Auto-login after registration
    await authStore.login(form.value.username, form.value.password);
    router.push('/dashboard');
  } catch (err) {
    console.error('Registration failed:', err);
  }
}
</script>
