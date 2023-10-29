import { createSignal } from "solid-js";
import { Asistencia, TodayPlotXHelper, TodayPlotXTest } from "../../types";

export const [AsistenciaResponse, setAsistenciaResponse] = createSignal<Asistencia[]>([])
export const [AsistenciaTest, setAsistenciaTest] = createSignal<TodayPlotXTest[]>([]);
export const [AsistenciaEntrada, setAsistenciaEntrada] = createSignal<TodayPlotXHelper[]>([]);
export const [AsistenciaSalida, setAsistenciaSalida] = createSignal<TodayPlotXHelper[]>([]);
export const [TotalEntradas, setTotalEntradas] = createSignal<number>();
export const [TotalSalidas, setTotalSalidas] = createSignal<number>();
export const [TotalTransactions, setTotalTransactions] = createSignal<number>();

export const groupDatesByHour = (dates: number[]): Map<string, TodayPlotXHelper> => {
  const newDates:Date[] = []
  for(const date of dates) {
    if(date !== null) {
      newDates.push(new Date(date))
    }
  }

  newDates.sort()

  const helper = new Map<string, TodayPlotXHelper>();

  for(const date of newDates) {
    let hour = date.getHours()
    let minutes = date.getMinutes()
    const minutesMargin: '00' | '30' = minutes >= 30 ? '30' : '00'

    const temp:TodayPlotXHelper = {
      name: `${hour}:${minutesMargin}`,
      exact: [`${hour}:${minutes}`],
      count: 1,
      dates: [date]
    }

    const prev = helper.get(temp.name);
    if(prev) {
      helper.set(temp.name, {
        name: temp.name,
        count: prev.count + temp.count,
        exact: [...prev.exact, temp.exact[0]],
        dates: [...prev.dates, temp.dates[0]]
      })
    } else {
      helper.set(temp.name, temp);
    }
  }

  return helper;
}

export const countTransactionEntradaSalida = (data:Asistencia[]):[number, number, number] => {
  console.log(data);
  //Number of rows that have entrada
  let entrada = 0;
  //Number of rows that have salida (which consequently means they also have entrada)
  let salida = 0;
  //Number of rouws that have entrada and salida.
  let transaction = 0;
  for (const d of data) {
    console.log(d);
    if (d.entrada > 0) {
      entrada++
    }
    if(d.salida !== null) {
      salida++
    }
    if(d.entrada !== null && d.salida !== null) {
      transaction++
    }
  }
  return [transaction, entrada, salida]
}
