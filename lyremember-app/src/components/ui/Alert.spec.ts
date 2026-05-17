import { describe, it, expect } from 'vitest';
import { mount } from '@vue/test-utils';
import Alert from './Alert.vue';

describe('Alert', () => {
  it('renders nothing when modelValue is false', () => {
    const wrapper = mount(Alert, {
      props: { modelValue: false },
      slots: { default: 'Hidden message' },
    });
    expect(wrapper.find('[role="alert"]').exists()).toBe(false);
  });

  it('renders the message and applies the variant class', () => {
    const wrapper = mount(Alert, {
      props: { modelValue: true, type: 'error' },
      slots: { default: 'Something failed' },
    });
    const root = wrapper.find('[role="alert"]');
    expect(root.exists()).toBe(true);
    expect(root.text()).toContain('Something failed');
    expect(root.classes()).toContain('alert-error');
  });

  it('emits update:modelValue=false when the close button is clicked', async () => {
    const wrapper = mount(Alert, {
      props: { modelValue: true, closable: true, type: 'success' },
      slots: { default: 'Saved' },
    });
    await wrapper.find('button[aria-label="Close alert"]').trigger('click');
    expect(wrapper.emitted('update:modelValue')).toEqual([[false]]);
  });
});
