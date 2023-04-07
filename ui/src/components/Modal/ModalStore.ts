import { writable } from "svelte/store";

export interface ModalProps {
  title: string;
  description: string;
  type: "error" | "info";
}

export const modal_info = writable<ModalProps | null>(null);
