import { computed } from "vue";

import useReadAnnouncements from "./useReadAnnouncements";
import useRemoteData from "./useRemoteData";

export const useUnreadAnnouncements = () => {
  const readAnnouncementsSet = useReadAnnouncements();
  const {
    data: { announcements },
    status: { announcements: announcementsStatus },
  } = useRemoteData();

  const announcementsSet = computed(
    () => new Set(announcements.map((announcement) => announcement.id)),
  );

  const unreadAnnouncementsCount = computed(() => {
    return announcementsStatus.value !== "success"
      ? 0
      : [...announcementsSet.value].filter(
        (announcementId) => !readAnnouncementsSet.value.has(announcementId),
      ).length;
  });

  return unreadAnnouncementsCount;
};

export default useUnreadAnnouncements;
