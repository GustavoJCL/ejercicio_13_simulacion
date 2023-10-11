import type { GraphDistributionValues } from './interfacesGraphDistribution.js';

export interface ResponseData {
  mu: number;
  sigma: number;
  estadistico: number;
  estadistico_tabla: number;
  normal_inv_value: GraphDistributionValues;
  corridas: number;
}

export interface SourceDataItem {
  mu: number;
  sigma: number;
  estadistico: number;
  estadistico_tabla: number;
  corridas: number;
}
