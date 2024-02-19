import { Document } from './document';
import { Lines } from './line';
import { Utils } from './utils';

export const useReturns = {
  document: {
    listInbound: Document.useInbounds,
    listAllInbound: Document.useInboundsAll,
    listOutbound: Document.useOutbounds,
    listAllOutbound: Document.useOutboundsAll,
    outboundReturn: Document.useOutboundReturn,

    insertOutboundReturn: Document.useInsertOutboundReturn,
    deleteOutboundRows: Document.useOutboundDeleteRows,
    deleteInboundRows: Document.useInboundDeleteRows,
  },
  lines: {
    outboundReturnLines: Lines.useOutboundReturnLines,
    inboundReturnLines: Lines.useInboundReturnLines,
  },
  utils: {
    api: Utils.useReturnsApi,
  },
};
