import { Component } from "solid-js"
import "./EventsControls.scss";
import { A } from "@solidjs/router";
import Return from "../utils/return/Return";
import { CurrentEvent } from "../../sharedSignals";

const EventsControls: Component = () => {

  return (
    <div class="events-controls">
      <Return />
      <div class="name">
        <p>{CurrentEvent()?.nombre}</p>
      </div>
      <div class="controls">
        <A href="./invite">
          <button>Añadir invitados</button>
        </A>
        <A href="./attendance">
          <button>Tomar Asistencia</button>
        </A>
      </div>
    </div>
  )
}

export default EventsControls
