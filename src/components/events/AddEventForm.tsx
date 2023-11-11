import { Component, For, createSignal, onMount } from "solid-js";
import { CreateEvento, Sala } from "../../types";
import { invoke } from "@tauri-apps/api";

const AddEventForm:Component = () => {

  const [Salas, setSalas] = createSignal<Sala[]>([]);
  const [AddEventData, setAddEventData] = createSignal<CreateEvento>({
    sala_id: 1,
    nombre: ''
  });

  onMount(() => {
    invoke('get_salas').then(setSalas)
  })

  const submit = async () => {
    invoke("add_event", {event: AddEventData()})
  }

  return (
    <div class="add-event-form">
      <p>Sala</p>
      <select name="salas" id="salas" class="salas">
        <For each={Salas()}>
          {(sala) => (
            <option
              onclick={() => setAddEventData(AED => ({...AED!, sala_id: sala.sala_id}))}
              oninput={() => setAddEventData(AED => ({...AED!, sala_id: sala.sala_id}))}
              value={sala.sala_id}
            >{sala.sala_nom}</option>
          )}
        </For>
      </select>
      <p>Nombre</p>
      <input type="text" oninput={(e) => setAddEventData(AED => ({...AED!, nombre: e.target.value}))}/>
      <button onClick={() => submit()}>Enviar</button>
    </div>
  )
}

export default AddEventForm
