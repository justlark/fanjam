import { reactive } from "vue";

// Convention organizers can password-protect an environment. Once the user
// enters the password, we keep it in local storage so they only have to enter
// it once per environment, and send it with every API request that needs it.

const passwordKey = (envId: string): string => `password:${envId}`;

export const getStoredPassword = (envId: string): string | undefined =>
  localStorage.getItem(passwordKey(envId)) ?? undefined;

export const setStoredPassword = (envId: string, password: string): void => {
  localStorage.setItem(passwordKey(envId), password);
};

export const clearStoredPassword = (envId: string): void => {
  localStorage.removeItem(passwordKey(envId));
};

// The environments where the API has rejected our request for want of a valid
// password, and the user needs to be prompted for one.
const passwordRequiredEnvs = reactive(new Set<string>());

export const requirePassword = (envId: string): void => {
  passwordRequiredEnvs.add(envId);
};

export const isPasswordRequired = (envId: string): boolean => passwordRequiredEnvs.has(envId);

export const passwordAccepted = (envId: string): void => {
  passwordRequiredEnvs.delete(envId);
};
