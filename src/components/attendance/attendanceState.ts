import { createSignal } from "solid-js";

export const [currentSalaId, setCurrentSalaId] = createSignal<number>(0);
export const [currentSalaName, setCurrentSalaName] = createSignal<string>('');
export const [IsSalaSelected, setIsSalaSelected] = createSignal<boolean>(false);
