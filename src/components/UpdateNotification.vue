<script setup lang="ts">
import { computed } from 'vue';
import type { UpdatePhase, UpdateInfo } from '@/composables/useUpdater';

interface Props {
  phase: UpdatePhase;
  updateInfo: UpdateInfo | null;
  downloadPercent: number;
  errorMessage: string | null;
}

const props = defineProps<Props>();
const emit = defineEmits<{
  install: [];
  dismiss: [];
  restart: [];
}>();

const isVisible = computed(() => props.phase !== 'idle');

const title = computed(() => {
  switch (props.phase) {
    case 'available': return `Update available — v${props.updateInfo?.version ?? ''}`;
    case 'downloading': return 'Downloading update...';
    case 'ready': return 'Update ready to install';
    case 'error': return 'Update failed';
    default: return '';
  }
});
</script>

<template>
  <Transition name="slide-up">
    <div v-if="isVisible" class="update-notification">
      <div class="update-header">
        <span class="update-title">{{ title }}</span>
        <button
          v-if="phase !== 'downloading'"
          class="dismiss-btn"
          aria-label="Dismiss"
          @click="emit('dismiss')"
        >
          &#x2715;
        </button>
      </div>

      <p v-if="phase === 'available' && updateInfo?.body" class="update-body">
        {{ updateInfo.body }}
      </p>

      <p v-if="phase === 'error'" class="update-error">
        {{ errorMessage }}
      </p>

      <div v-if="phase === 'downloading'" class="progress-track">
        <div
          class="progress-fill"
          :style="{ width: downloadPercent > 0 ? `${downloadPercent}%` : '100%' }"
          :class="{ indeterminate: downloadPercent === 0 }"
        />
      </div>

      <div class="update-actions">
        <button v-if="phase === 'available'" class="btn-primary" @click="emit('install')">
          Install
        </button>
        <button v-if="phase === 'ready'" class="btn-primary" @click="emit('restart')">
          Restart now
        </button>
      </div>
    </div>
  </Transition>
</template>

<style scoped>
.update-notification {
  position: fixed;
  bottom: 24px;
  right: 24px;
  width: 340px;
  background: #161616;
  border: 1px solid #2a2a2a;
  border-radius: 10px;
  padding: 16px;
  z-index: 9999;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.7);
}

.update-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
}

.update-title {
  font-size: 13px;
  font-weight: 600;
  color: #ff6b35;
}

.dismiss-btn {
  background: none;
  border: none;
  color: #666;
  cursor: pointer;
  font-size: 14px;
  padding: 0;
  line-height: 1;
  transition: color 0.15s;
}

.dismiss-btn:hover {
  color: #e0e0e0;
}

.update-body {
  font-size: 12px;
  color: #888;
  margin-bottom: 10px;
  line-height: 1.5;
  max-height: 72px;
  overflow-y: auto;
}

.update-error {
  font-size: 12px;
  color: #e05555;
  margin-bottom: 10px;
}

.progress-track {
  height: 4px;
  background: #2a2a2a;
  border-radius: 2px;
  overflow: hidden;
  margin-bottom: 4px;
}

.progress-fill {
  height: 100%;
  background: #ff6b35;
  border-radius: 2px;
  transition: width 0.2s ease;
}

.progress-fill.indeterminate {
  animation: indeterminate 1.4s ease-in-out infinite;
}

@keyframes indeterminate {
  0% { transform: translateX(-100%); width: 60%; }
  100% { transform: translateX(200%); width: 60%; }
}

.update-actions {
  display: flex;
  justify-content: flex-end;
  margin-top: 10px;
}

.btn-primary {
  background: #ff6b35;
  color: #fff;
  border: none;
  border-radius: 6px;
  padding: 6px 16px;
  font-size: 13px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.15s;
}

.btn-primary:hover {
  opacity: 0.85;
}

.slide-up-enter-active,
.slide-up-leave-active {
  transition: opacity 0.25s ease, transform 0.25s ease;
}

.slide-up-enter-from,
.slide-up-leave-to {
  opacity: 0;
  transform: translateY(12px);
}
</style>
