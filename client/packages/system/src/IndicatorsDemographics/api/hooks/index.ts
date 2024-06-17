import { Utils } from './utils';
import { Document } from './document';

export const useDemographicData = {
  utils: {
    demographics: Utils.useDemographicsApi,
  },
  indicator: {
    get: Document.useDemographicIndicator,
    list: Document.useDemographicIndicators,
    insert: Document.useDemographicIndicatorInsert,
    update: Document.useDemographicIndicatorUpdate,
  },
  projection: {
    get: Document.useDemographicProjection,
    list: Document.useDemographicProjections,
    upsert: Document.useDemographicProjectionUpsert,
  },
  vaccineItems: {
    get: Document.useVaccineItems,
  },
};
