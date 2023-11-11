import { Component, For, Show, createSignal, onMount } from 'solid-js';
import './Events.scss'
import { invoke } from '@tauri-apps/api';
import { Evento } from '../../types';
import AddEventForm from './AddEventForm';
// @ts-ignore
import { Motion } from "@motionone/solid";
import { A } from '@solidjs/router';
import { setCurrentEvent } from '../../sharedSignals';

const Events: Component = () => {

  const [Events, setEvents] = createSignal<Evento[]>([])
  const [ShowAddEvent, setShowAddEvent] = createSignal<boolean>(false);

  onMount(() => {
    invoke('get_events').then((ev) => {
      setEvents(ev as Evento[])
    })
  })

  return (
    <div class='events-container'>
      <div class="add">
        <button class="add-button" onclick={() => setShowAddEvent(!ShowAddEvent())}>Añadir un Evento</button>
      </div>

      <Show when={ShowAddEvent()}>
        <AddEventForm />
      </Show>

      <Motion.div class="content" initial={{opacity: 0, y: -30}} inView={{opacity: 1, y: 0}}>
        <p>Listado de eventos disponibles</p>
        <div class='list'>
          <Show when={Events().length > 0}>
            <For each={Events()}>
              {(v) => (
                <div class='event'>
                  <A href={`./${v.id}`} onclick={() => setCurrentEvent(v)}>
                    <button>{v.nombre}</button>
                  </A>
                </div>
              )}
            </For>
          </Show>
        </div>
      </Motion.div>
    </div>
  );
};

export default Events
