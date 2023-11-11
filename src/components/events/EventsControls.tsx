import { Component, onMount } from "solid-js"
import "./EventsControls.scss";
import { A, useNavigate } from "@solidjs/router";
import Return from "../utils/return/Return";
import { CurrentEvent } from "../../sharedSignals";
import TopText from "../utils/topText/TopText";
import TopRightContainer from "../utils/TopRightContainer/TopRightContainer";

const EventsControls: Component = () => {
  const navigator = useNavigate();

  onMount(() => {
    CurrentEvent() ? null : navigator('/events')
  })

  return (
    <div class="events-controls">
      <TopRightContainer>
        <Return />
      </TopRightContainer>
      <TopText text={CurrentEvent()?.nombre}/>

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
