import type { GraphDistributionValues } from './interfacesGraphDistribution.js';

export interface ResponseData {
  estadistico: { [index: number]: number };
  frecuencia_esperada: { [index: number]: number };
  frecuencia_observada: { [index: number]: number };
  m: number;
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
