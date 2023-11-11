import { useNavigate, useParams } from "@solidjs/router";
import "./EventsInvite.scss";
import Return from "../../utils/return/Return";
import { Show, createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api";
import { QueryState } from "../../../types";
import { CurrentEvent, QueryStateEnum } from "../../../sharedSignals";
// @ts-ignore
import { Motion } from "@motionone/solid";
import TopText from "../../utils/topText/TopText";
import TopRightContainer from "../../utils/TopRightContainer/TopRightContainer";

const EventsInvite = () => {
  const params = useParams();
  const navigator = useNavigate()

  onMount(() => {
    CurrentEvent() ? null : navigator('/events')
  })

  const [Id, setId] = createSignal<number>()
  const [QueryState, setQueryState] = createSignal<QueryState>()

  const submit = () => {
    invoke('add_invitado', {id: Id(), eventoId: Number(params.id)})
      .then(d => {
        if(d) {
          setQueryState({type: QueryStateEnum.Success, message: 'Invitado agregado!'})
        } else {
          setQueryState({
            type: QueryStateEnum.Error,
            message: 'Hubo un error agregando al invitado'
          })
        }

        setTimeout(() => {
          setQueryState()
        }, 1000);
      })
  }

  return (
    <div class="events-invite">
      <TopRightContainer>
        <Return />
      </TopRightContainer>
      <TopText text={CurrentEvent()?.nombre} />

      <p class="label">Añadir invitado</p>
      <p class="disclaimer">En caso de no contar con matrícula escribir 0 ó dejar en blanco</p>
      <input
        type="number"
        placeholder="Ej. 2128081"
        class="input"
        oninput={(e) => setId(Number(e.target.value))}
        />
      <button class="submit" onclick={() => submit()}>Enviar</button>

      <Show when={QueryState()}>
        <Motion.div>
          <p>{QueryState()?.message}</p>
        </Motion.div>
      </Show>

    </div>
  )
}

export default EventsInvite
