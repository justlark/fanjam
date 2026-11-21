<script setup lang="ts">
import { computed, ref, useId } from "vue";
import Dialog from "primevue/dialog";
import Password from "primevue/password";
import Button from "primevue/button";
import Message from "primevue/message";
import api from "@/utils/api";
import useEnvId from "@/composables/useEnvId";
import useRemoteData from "@/composables/useRemoteData";
import { isPasswordRequired, passwordAccepted, setStoredPassword } from "@/utils/password";

const envId = useEnvId();
const { reloadAll } = useRemoteData();

const visible = computed(() => isPasswordRequired(envId.value));

const password = ref("");
const submitting = ref(false);
const errorMessage = ref<string>();

const inputId = useId();

const submit = async () => {
  if (password.value === "" || submitting.value) return;

  submitting.value = true;
  errorMessage.value = undefined;

  // Remember which env we were asked about, in case the user navigates while
  // we're waiting on the server.
  const currentEnvId = envId.value;

  try {
    const result = await api.checkPassword(currentEnvId, password.value);

    if (result.ok) {
      setStoredPassword(currentEnvId, password.value);
      passwordAccepted(currentEnvId);
      password.value = "";
      await reloadAll();
    } else if (result.code === 401) {
      errorMessage.value = "That password isn't right. Try again.";
    } else if (result.code === "offline") {
      errorMessage.value = "You're offline. Connect to the internet and try again.";
    } else {
      errorMessage.value = "Something went wrong checking the password. Try again.";
    }
  } finally {
    submitting.value = false;
  }
};
</script>

<template>
  <Dialog
    class="w-100 m-4"
    :visible="visible"
    modal
    block-scroll
    :closable="false"
    :draggable="false"
    header="Password Required"
    pt:root:data-testid="password-dialog"
  >
    <form class="flex flex-col gap-4" @submit.prevent="submit">
      <label :for="inputId">This app is password-protected. Enter the password to continue.</label>
      <Password
        v-model="password"
        :input-id="inputId"
        :feedback="false"
        :invalid="errorMessage !== undefined"
        toggle-mask
        fluid
        autofocus
        :input-props="{ autocomplete: 'current-password' }"
        pt:pc-input-text:root:data-testid="password-dialog-input"
      />
      <Message
        v-if="errorMessage"
        severity="error"
        size="small"
        variant="simple"
        data-testid="password-dialog-error"
      >
        {{ errorMessage }}
      </Message>
      <Button
        type="submit"
        fluid
        label="Continue"
        icon="bi bi-unlock"
        :loading="submitting"
        :disabled="password === ''"
        data-testid="password-dialog-submit"
      />
    </form>
  </Dialog>
</template>
