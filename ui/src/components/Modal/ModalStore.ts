import { writable } from "svelte/store";

export interface ModalProps {
  title: string;
  description: string;
  type: "info" | "fixed-info" | "question" | "error";

  error: object; // only use with error type
  show_error_info: boolean; // only use with error type

  yes_callback: () => {} | Promise<void> | null; // only use with question type
  keep_open_after_yes: boolean;
}

export const modal_info = writable<ModalProps | null>(null);
