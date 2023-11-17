import { Component, For, createSignal, onMount } from "solid-js";
import { CreateEvento, Evento } from "../../types";
import { ImCancelCircle } from 'solid-icons/im'
import { invoke } from "@tauri-apps/api";
//@ts-ignore
import { Motion, Presence } from "@motionone/solid";
import "./AddEventForm.scss";
import { Salas, setAllEvents, setSalas, setShowAddEvent } from "../../sharedSignals";

const AddEventForm: Component = () => {

  const [AddEventData, setAddEventData] = createSignal<CreateEvento>({
    sala_id: 1,
    nombre: ''
  });

  onMount(() => {
    if(Salas().length < 1) {
      invoke('get_salas').then(setSalas)
    }
  })

  const submit = async () => {
    invoke("add_event", { event: AddEventData() }).then(d => {
      setAllEvents(e => [...e, d as Evento])
    })
  }

  return (
    <Presence>
      <Motion.div
        initial={{ opacity: 0 }}
        inView={{ opacity: 1 }}
        exit={{ opacity: 0 }}
        class="add-event-form">
        <div class="close">
          <ImCancelCircle class="icon" onclick={() => setShowAddEvent(false)}/>
        </div>
        <p>Sala</p>
        <select name="salas" id="salas" class="salas">
          <For each={Salas()}>
            {(sala) => (
              <option
                onclick={() => setAddEventData(AED => ({ ...AED!, sala_id: sala.sala_id }))}
                oninput={() => setAddEventData(AED => ({ ...AED!, sala_id: sala.sala_id }))}
                value={sala.sala_id}
              >{sala.sala_nom}</option>
            )}
          </For>
        </select>
        <p>Nombre</p>
        <input
          type="text"
          oninput={(e) => setAddEventData(AED => ({ ...AED!, nombre: e.target.value }))}
          onKeyPress={(e) => e.key === 'Enter' ? submit() : null}
          />
        <button onClick={() => submit()}>Enviar</button>
      </Motion.div>
    </Presence>
  )
}

export default AddEventForm
