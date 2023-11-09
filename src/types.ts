export type Sala = {
  sala_id: number;
  sala_piso: number;
  sala_nom: string;
}

export type RegisterData = {
  sala_id:number,
  visitante_id: number,
  fecha: number
}

export type RegisterResponse = {
  message: 'Success' | 'Failure' | 'Idle',
  register_type: 'Salida' | 'Entrada' | 'Idle',
  visitante_id: number
}

export type Asistencia = {
  sala_id: number,
  visitante_id: number,
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

export type DBConfigInterface = {
  db_name: string,
  db_port: number,
  db_user: string,
  db_pass: string,
  db_host:string
}

export type Evento = {
  id: number,
  sala_id: number,
  nombre: string
}

export type CreateEvento = {
  sala_id: number,
  nombre: string
}
