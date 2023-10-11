import type { GraphDistributionValues } from './interfacesGraphDistribution.js';

export interface Estadistico {
  [index: number]: number;
}
export interface FrecuenciaEsperada {
  [index: number]: number;
}
export interface FrecuenciaObservada {
  [index: number]: number;
}

export interface ResponseData {
  estadistico: Estadistico;
  frecuencia_esperada: FrecuenciaEsperada;
  frecuencia_observada: FrecuenciaObservada;
  h: number;
  total_estadistico: number;
  total_estadistico_tabla: number;
  chi_square_values: GraphDistributionValues;
}
export interface SourceDataItem {
  indice: number;
  estadistico: number;
  frecuencia_esperada: number;
  frecuencia_observada: number;
}
