import { Component, For, Show, onMount } from 'solid-js';
import './Events.scss'
import { invoke } from '@tauri-apps/api';
import { Evento } from '../../types';
import AddEventForm from './AddEventForm';
// @ts-ignore
import { Motion, Presence } from "@motionone/solid";
import { A } from '@solidjs/router';
import { AllEvents, ShowAddEvent, setAllEvents, setCurrentEvent, setShowAddEvent } from '../../sharedSignals';

const Events: Component = () => {

  onMount(() => {
    invoke('get_events').then((ev) => {
      setAllEvents(ev as Evento[])
    })
  })

  return (
    <Presence>

      <div class='events-container'>
        <div class="add">
          <button class="add-button" onclick={() => setShowAddEvent(!ShowAddEvent())}>Añadir un Evento</button>
        </div>

        <Show when={ShowAddEvent()}>
          <AddEventForm />
        </Show>

        <Motion.div class="content" initial={{ opacity: 0, y: -30 }} inView={{ opacity: 1, y: 0 }}>
          <p>Listado de eventos disponibles</p>
          <div class='list'>
            <Show when={AllEvents().length > 0}>
              <For each={AllEvents()}>
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
    </Presence>
  );
};

export default Events
