import { ref, computed, watchEffect } from "vue";
import { useRoute } from "vue-router";

const readAnnouncements = ref<Array<string>>();
const readAnnouncementsSet = ref<Set<string>>(new Set());

const useReadAnnouncements = () => {
  const route = useRoute();
  const envId = computed(() => route.params.envId as string);
  const storageKey = computed(() => `announcements:${envId.value}`);

  watchEffect(() => {
    if (readAnnouncements.value === undefined) {
      const storedValue = localStorage.getItem(storageKey.value);
      if (storedValue) {
        try {
          readAnnouncements.value = JSON.parse(storedValue);
          readAnnouncementsSet.value = new Set(readAnnouncements.value);
        } catch {
          readAnnouncements.value = [];
          readAnnouncementsSet.value.clear();
        }
      } else {
        readAnnouncements.value = [];
        readAnnouncementsSet.value.clear();
      }

      return;
    }

    localStorage.setItem(storageKey.value, JSON.stringify(readAnnouncements.value));
  });

  watchEffect(() => {
    readAnnouncements.value = Array.from(readAnnouncementsSet.value);
  });

  return readAnnouncementsSet;
};

export default useReadAnnouncements;
