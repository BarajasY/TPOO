export type Sala = {
  sala_id: number;
  sala_piso: number;
  sala_nom: string;
  biblio_id: string;
}

export type RegisterData = {
  sala_id:number,
  visitante_mat: number,
  fecha: number
}

export type RegisterResponse = {
  message: 'Success' | 'Failure' | 'Idle',
  register_type: 'Salida' | 'Entrada' | 'Idle',
  visitant_mat: number
}

export type Asistencia = {
  sala_id: number,
  visitante_mat: number,
  biblio_id: number,
  asistencia_id: number,
  entrada: number,
  salida: number
}

export type TodayPlotXHelper = {
  name: string,
  count: number,
  exact: string[],
  dates: Date[]
}

export type TodayPlotXTest = {
  date: Date,
  count: number
}
