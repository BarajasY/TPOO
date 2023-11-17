import { invoke } from "@tauri-apps/api";
import { appLocalDataDir } from "@tauri-apps/api/path";
import { Component, Show, onMount } from "solid-js";
import { Asistencia } from "../../types";
import {
  AsistenciaEntrada,
  AsistenciaSalida,
  TotalEntradas,
  TotalSalidas,
  countTransactionEntradaSalida,
  groupDatesByHour,
  setAsistenciaEntrada,
  setAsistenciaSalida,
  setTotalEntradas,
  setTotalSalidas,
  setTotalTransactions
} from "./signals";
import { CurrentDay } from "../../sharedSignals";
import LinePlot from "../utils/plots/PlotToday";
import "./statistics.scss"
import TopRightContainer from "../utils/TopRightContainer/TopRightContainer";
import OpenFolder from "../utils/openFolder/OpenFolder";
import MakeExcel from "../utils/makeExcel/MakeExcel";

const Statistics: Component = () => {
  const dias = ["Domingo", "Lunes", "Martes", "Miércoles", "Jueves", "Viernes", "Sábado"]

  onMount(() => {
    const current_date_day = new Date(CurrentDay().toDateString());

    invoke('get_statistics_by_date', { date: current_date_day.getTime() })
      .then((response) => {
        const data = response as Asistencia[];
        const [transaction, entrada, salida] = countTransactionEntradaSalida(data);
        const entradahelper = groupDatesByHour(data.map(d => d.entrada));
        const salidahelper = groupDatesByHour(data.map(d => d.salida));
        setAsistenciaEntrada(Array.from(entradahelper.values()));
        setAsistenciaSalida(Array.from(salidahelper.values()));
        setTotalEntradas(entrada);
        setTotalSalidas(salida);
        setTotalTransactions(transaction);
      })
      .catch(e => {
        console.error(e);
      })
  })

  const año = new Date();
  año.setMonth(0); // January
  año.setDate(1);   // 1st day of the month
  año.setHours(0, 0, 0, 0); // Set hours, minutes, seconds, and milliseconds to 0

  const dia = new Date(CurrentDay().getTime())

  return (
    <div class="statistics-container">
      <h1 class="current-day">{CurrentDay().toDateString()}</h1>
      <div class="statistics">
        <TopRightContainer>
          <OpenFolder />
          <MakeExcel
            label="año"
            cantidad={año.getTime()}
            name={año.getFullYear().toString()}
          />
          <MakeExcel
            label="dia"
            cantidad={new Date(CurrentDay().getTime()).getTime()}
            name={dias[dia.getDay()] + `-${dia.getDate()}` + `-${dia.getMonth() + 1}`}
          />
        </TopRightContainer>
        <Show
          when={AsistenciaEntrada().length > 0}
          fallback={<h1 class="fallback">No hay datos hasta la fecha</h1>}>
          <LinePlot
            total={TotalEntradas()!}
            variant="Entrada"
            data={AsistenciaEntrada()} />
        </Show>
        <Show
          when={AsistenciaSalida().length > 0}
          fallback={<h1 class="fallback">No hay datos hasta la fecha</h1>}>
          <LinePlot
            total={TotalSalidas()!}
            variant="Salida"
            data={AsistenciaSalida()} />
        </Show>
      </div>
    </div>
  );
};

export default Statistics;
