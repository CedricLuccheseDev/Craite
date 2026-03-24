import { ref, watch, onMounted, onUnmounted, type Ref } from 'vue';

export function useScanTimer(isScanning: Ref<boolean>) {
  const elapsedSeconds = ref(0);
  let timerInterval: ReturnType<typeof setInterval> | null = null;

  function startTimer() {
    elapsedSeconds.value = 0;
    timerInterval = setInterval(() => {
      elapsedSeconds.value++;
    }, 1000);
  }

  function stopTimer() {
    if (timerInterval) {
      clearInterval(timerInterval);
      timerInterval = null;
    }
  }

  watch(isScanning, scanning => {
    if (scanning) startTimer();
    else stopTimer();
  });

  onMounted(() => {
    if (isScanning.value) startTimer();
  });

  onUnmounted(stopTimer);

  return { elapsedSeconds };
}
