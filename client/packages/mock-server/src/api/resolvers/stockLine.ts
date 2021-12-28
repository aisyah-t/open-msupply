import { locationResolver } from './location';
import { createListResponse } from './utils';
import { ResolvedStockLine, ListResponse } from './../../data/types';
import { db } from '../../data/database';

export const stockLineResolver = {
  byId: (id: string): ResolvedStockLine => {
    const stockLine = db.stockLine.get.byId(id);
    const item = db.item.get.byId(stockLine.itemId);
    const location = stockLine.locationId
      ? locationResolver.byId(stockLine.locationId)
      : null;

    return { ...stockLine, item, __typename: 'StockLineNode', location };
  },
  list: (
    lines = db.stockLine.get.all()
  ): ListResponse<ResolvedStockLine, 'StockLineConnector'> => {
    const resolved = lines.map(line => stockLineResolver.byId(line.id));

    const nodes = resolved.map(stockLine =>
      stockLineResolver.byId(stockLine.id)
    );

    return createListResponse(nodes.length, nodes, 'StockLineConnector');
  },
};
