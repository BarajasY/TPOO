import { createSignal } from "solid-js";
import { DBConfigInterface, Evento, RegisterData, RegisterResponse, Sala } from "./types";

export const SectionsEnum = {
  Attendance: Symbol("Attendance"),
  Statistics: Symbol("Statistics")
}

export const [Salas, setSalas] = createSignal<Sala[]>([]);

export const [ToggleMenu, setToggleMenu] = createSignal<boolean>(false)
export const [CurrentSection, setCurrentSection] = createSignal<Symbol>(SectionsEnum.Attendance);

export const [AttendanceData, setAttendanceData] = createSignal<RegisterData>({
  sala_id: 0,
  visitante_id: 0,
  fecha: 0
});

export const [AttendanceResponse, setAttendanceResponse] = createSignal<RegisterResponse>({
  visitante_id: 0,
  message: 'Idle',
  register_type: 'Idle'
})

export const [CurrentDay, setCurrentDay] = createSignal<Date>(new Date())

//DB Setup
export const [DatabaseCredentials, setDatabaseCredentials] = createSignal<DBConfigInterface>();
export const [TempDatabaseCredentials, setTempDatabaseCredentials] = createSignal<DBConfigInterface>({
  db_name: "tpoo",
  db_host: "localhost",
  db_pass: "placeholder",
  db_user: "postgres",
  db_port: 1402,
  db_table: "Hemeroteca"
});

//Evento
export const [CurrentEvent, setCurrentEvent] = createSignal<Evento>();

export const QueryStateEnum = {
  Success: Symbol('Success'),
  Error: Symbol('Error')
}

export const [AllEvents, setAllEvents] = createSignal<Evento[]>([])

export const [ShowAddEvent, setShowAddEvent] = createSignal<boolean>(false);
