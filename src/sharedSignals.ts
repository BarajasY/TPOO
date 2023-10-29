import { createSignal } from "solid-js";
import { RegisterData, RegisterResponse } from "./types";

export const SectionsEnum = {
  Attendance: Symbol("Attendance"),
  Statistics: Symbol("Statistics")
}

export const [ToggleMenu, setToggleMenu] = createSignal<boolean>(false)
export const [CurrentSection, setCurrentSection] = createSignal<Symbol>(SectionsEnum.Attendance);

export const [AttendanceData, setAttendanceData] = createSignal<RegisterData>({
  sala_id: 0,
  visitante_mat: 0,
  fecha: 0
});

export const [AttendanceResponse, setAttendanceResponse] = createSignal<RegisterResponse>({
  visitant_mat: 0,
  message: 'Idle',
  register_type: 'Idle'
})

export const [CurrentDay, setCurrentDay] = createSignal<Date>(new Date())
