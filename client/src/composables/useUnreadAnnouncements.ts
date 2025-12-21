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
    () => new Set(announcements.value.map((announcement) => announcement.id)),
  );

  const unreadAnnouncementsCount = computed(() => {
    return announcementsStatus.value !== "success"
      ? new Set()
      : new Set(
        [...announcementsSet.value].filter(
          (announcementId) => !readAnnouncementsSet.value.has(announcementId),
        ),
      );
  });

  return unreadAnnouncementsCount;
};

export default useUnreadAnnouncements;
