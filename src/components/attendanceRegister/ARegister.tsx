import { Component, Match, Show, Switch, createSignal } from "solid-js";
import { AiFillCaretLeft } from 'solid-icons/ai'
import "./ARegister.scss"
import { currentSalaName, setCurrentSalaId, setCurrentSalaName, setIsSalaSelected } from "../attendance/attendanceState";
import { AttendanceData, AttendanceResponse, setAttendanceData, setAttendanceResponse } from "../../sharedSignals";
import { invoke } from "@tauri-apps/api/tauri";
import { RegisterResponse } from "../../types";
//@ts-ignore
import { Motion, Presence } from "@motionone/solid"

const ARegister: Component = () => {

  const [Matricula, setMatricula] = createSignal<number>(0);

  const returnToAttendance = () => {
    setCurrentSalaId(0);
    setAttendanceData(() => ({
      sala_id: 0,
      visitante_id: 0,
      fecha: 0
    }))
    setCurrentSalaName('')
    setIsSalaSelected(false)
  }

  const submitAttendance = () => {
    setAttendanceData((prev) => ({
      ...prev,
      visitante_id: Matricula(),
      fecha: new Date().getTime()
    }))

    invoke('add_registration', { data: AttendanceData() })
      .then((response) => {
        if (response) {
          const temp: RegisterResponse = response as RegisterResponse;
          setAttendanceResponse(() => ({
            message: temp.message,
            register_type: temp.register_type,
            visitante_id: temp.visitante_id
          }));
          setTimeout(() => {
            setAttendanceResponse(() => ({
              message: 'Idle',
              register_type: 'Idle',
              visitante_id: 0
            }))
          }, 1500);
        }
      })
      .catch(err => {
        console.error(err)
      });
  }

  return (
    <div class="a-register">
      <div class="header">
        <button
          onclick={() => returnToAttendance()}
          class="back-sala">
          <AiFillCaretLeft class="icon" />
          <p>Escoger otra sala</p>
        </button>
      </div>
      <div class="content">
        <h1 class="name">{currentSalaName()}</h1>
        <h1 class="text">Escriba su matrícula</h1>
        <input
          type="number"
          class="mat_input"
          pattern="[0-9]+"
          oninput={(e) => setMatricula(Number(e.target.value))} />
        <button class="send-button" onclick={() => submitAttendance()}>Enviar</button>
      </div>
      <Presence>
        <Show when={AttendanceResponse().message !== 'Idle'}>
          <Motion.div initial={{ opacity: 0, y: -30 }} inView={{ opacity: 1, y: 0 }} exit={{opacity: 0, y: -30}} class="message">
            <Switch>
              <Match when={AttendanceResponse().register_type === 'Entrada'}>
                <p class="entrada">Bienvenido {AttendanceResponse().visitante_id}</p>
              </Match>
              <Match when={AttendanceResponse().register_type === 'Salida'}>
                <p class="salida">Hasta luego {AttendanceResponse().visitante_id}</p>
              </Match>
            </Switch>
          </Motion.div>
        </Show>
      </Presence>
    </div>
  );
};

export default ARegister;
