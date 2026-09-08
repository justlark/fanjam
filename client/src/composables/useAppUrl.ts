import { envContext } from "@/context";
import useEnvId from "./useEnvId";

// Build a path scoped to the current app's mount point, which is different for
// custom domains vs the default domain.
const buildPath = (envId: string, segment: string): string => {
  const cleaned = segment.startsWith("/") ? segment.slice(1) : segment;
  return envContext.mode === "custom" ? `/${cleaned}` : `/app/${envId}/${cleaned}`;
};

// Returns a function that takes a path segment and returns a path scoped to
// the current environment.
export const useAppPath = () => {
  const envId = useEnvId();
  return (segment: string) => buildPath(envId.value, segment);
};

// Returns a function that takes a path segment and returns an absolute URL
// scoped to the current environment.
export const useAppUrl = () => {
  const path = useAppPath();
  return (segment: string) => `${window.location.origin}${path(segment)}`;
};
