import { For, Match, Switch, createSignal, onMount } from "solid-js";
import "./attendance.scss";
import { invoke } from "@tauri-apps/api";
import { Sala } from "../../types";
import { IsSalaSelected, setCurrentSalaId, setCurrentSalaName, setIsSalaSelected } from "./attendanceState";
import ARegister from "../attendanceRegister/ARegister";
import { setAttendanceData } from "../../sharedSignals";

const Attendance = () => {
  const [Salas, setSalas] = createSignal<Sala[]>([]);

  onMount(() => {
    invoke("get_salas").then((salas) => {
      if (salas) {
        setSalas(salas as Sala[])
      }
    })
  })

  const selectSala = (id: number, name: string) => {
    setCurrentSalaId(id);
    setCurrentSalaName(name);
    setAttendanceData((prev) => ({
      ...prev,
      sala_id: id
    }))
    setIsSalaSelected(true);
  }

  return (
    <div class="attendanceContainer">
      <Switch>
        <Match when={IsSalaSelected()}>
          <ARegister />
        </Match>
        <Match when={!IsSalaSelected()}>
          <p>Listado de salas disponibles</p>
          <div class="salas">
            <For each={Salas()}>
              {(sala) => (
                <div class="sala" onclick={() => selectSala(sala.sala_id, sala.sala_nom)}>
                  <h1 onclick={() => selectSala(sala.sala_id, sala.sala_nom)}>{sala.sala_nom}</h1>
                </div>
              )}
            </For>
          </div>
        </Match>
      </Switch>
    </div>
  );
};

export default Attendance;
