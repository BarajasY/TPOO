//internal
import { useNavigate } from "@solidjs/router";
import { Match, Show, Switch, createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api";
import "./EventsAttendance.scss";
//animation
//@ts-ignore
import { Motion, Presence } from "@motionone/solid";
//components
import Return from "../../utils/return/Return";
import TopText from "../../utils/topText/TopText";
import TopRightContainer from "../../utils/TopRightContainer/TopRightContainer";
//types
import { RegisterResponse } from "../../../types";
//signals
import { CurrentEvent } from "../../../sharedSignals";

const EventsAttendance = () => {
  const navigator = useNavigate()


  const [AttendanceResponse, setAttendanceResponse] = createSignal<RegisterResponse>({
    visitante_id: 0,
    message: 'Idle',
    register_type: 'Idle'
  })
  const [Id, setId] = createSignal<number>(0);

  onMount(() => {
    CurrentEvent() ? null : navigator('/events')
  })

  const submit = () => {
    const EventRegistrationData = {
      evento_id: CurrentEvent()?.id,
      evento_invitados_id: Id(),
      fecha: new Date().getTime()
    }
    invoke('add_event_registration', {data: EventRegistrationData}).then(res => {
      setAttendanceResponse(res as RegisterResponse)

      setTimeout(() => {
        setAttendanceResponse(() => ({
          message: 'Idle',
          register_type: 'Idle',
          visitante_id: 0
        }))
      }, 1500);
    })
  }

  return (
    <div class="events-attendance">
        <TopRightContainer>
          <Return />
        </TopRightContainer>
        <TopText text={CurrentEvent()?.nombre}/>

        <div class="content">
          <p>Ingrese su matrícula</p>
          <input
            type="number"
            pattern="<[0-9]+>"
            placeholder="Ej. 2128081"
            oninput={(e) => setId(Number(e.target.value))}/>
          <button onClick={() => submit()}>Enviar</button>
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
  )
}

export default EventsAttendance;
