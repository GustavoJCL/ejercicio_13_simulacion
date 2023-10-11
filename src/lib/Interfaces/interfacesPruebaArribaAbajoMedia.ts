import type { GraphDistributionValues } from './interfacesGraphDistribution.js';

export interface ResponseData {
  mu: number;
  sigma: number;
  estadistico: number;
  estadistico_tabla_s: number;
  estadistico_tabla_i: number;
  normal_inv_value_i: GraphDistributionValues;
  normal_inv_value_s: GraphDistributionValues;
  corridas: number;
}

export interface SourceDataItem {
  mu: number;
  sigma: number;
  estadistico: number;
  estadistico_tabla_s: number;
  estadistico_tabla_i: number;
  corridas: number;
}
