import {
  InboundReturnInput,
  InvoiceNodeType,
  InvoiceSortFieldInput,
  SupplierReturnInput,
} from '@common/types';
import { OutboundReturnRowFragment, Sdk } from './operations.generated';
import { FilterByWithBoolean, SortBy } from '@common/hooks';

export type OutboundListParams = {
  first: number;
  offset: number;
  sortBy: SortBy<OutboundReturnRowFragment>;
  filterBy: FilterByWithBoolean | null;
};

const outboundParsers = {
  toSortField: (
    sortBy: SortBy<OutboundReturnRowFragment>
  ): InvoiceSortFieldInput => {
    switch (sortBy.key) {
      case 'createdDatetime': {
        return InvoiceSortFieldInput.CreatedDatetime;
      }
      case 'otherPartyName': {
        return InvoiceSortFieldInput.OtherPartyName;
      }
      case 'invoiceNumber': {
        return InvoiceSortFieldInput.InvoiceNumber;
      }
      case 'status':
      default: {
        return InvoiceSortFieldInput.Status;
      }
    }
  },
};

export const getReturnsQueries = (sdk: Sdk, storeId: string) => ({
  get: {
    listOutbound: async ({
      first,
      offset,
      sortBy,
      filterBy,
    }: OutboundListParams): Promise<{
      nodes: OutboundReturnRowFragment[];
      totalCount: number;
    }> => {
      const filter = {
        ...filterBy,
        type: { equalTo: InvoiceNodeType.SupplierReturn },
      };
      const result = await sdk.outboundReturns({
        first,
        offset,
        key: outboundParsers.toSortField(sortBy),
        desc: !!sortBy.isDesc,
        filter,
        storeId,
      });
      return result?.invoices;
    },
    listAllOutbound: async (
      sortBy: SortBy<OutboundReturnRowFragment>
    ): Promise<{
      nodes: OutboundReturnRowFragment[];
      totalCount: number;
    }> => {
      const filter = {
        type: { equalTo: InvoiceNodeType.SupplierReturn },
      };
      const result = await sdk.outboundReturns({
        key: outboundParsers.toSortField(sortBy),
        desc: !!sortBy.isDesc,
        filter,
        storeId,
      });
      return result?.invoices;
    },
    newSupplierReturnLines: async (inboundShipmentLineIds: string[]) => {
      const result = await sdk.newSupplierReturnLines({
        inboundShipmentLineIds,
        storeId,
      });

      return result?.newSupplierReturn;
    },
    inboundReturnLines: async (outboundShipmentLineIds: string[]) => {
      const result = await sdk.generateInboundReturnLines({
        outboundShipmentLineIds,
        storeId,
      });

      return result?.generateInboundReturnLines;
    },
    invoiceByNumber: async (invoiceNumber: number) => {
      const result = await sdk.invoiceByNumber({
        invoiceNumber,
        storeId,
      });

      return result?.invoiceByNumber;
    },
  },
  insertSupplierReturn: async (input: SupplierReturnInput) => {
    const result = await sdk.insertSupplierReturn({
      input,
    });

    return result.insertSupplierReturn;
  },
  insertInboundReturn: async (input: InboundReturnInput) => {
    const result = await sdk.insertInboundReturn({
      input,
      storeId,
    });

    return result.insertInboundReturn;
  },
  deleteOutbound: async (
    returns: OutboundReturnRowFragment[]
  ): Promise<string[]> => {
    const result = await sdk.deleteOutboundReturns({
      storeId,
      input: {
        ids: returns.map(({ id }) => id),
      },
    });

    const { deleteSupplierReturns } = result;

    if (deleteSupplierReturns.__typename === 'DeletedIdsResponse') {
      return deleteSupplierReturns.deletedIds;
    }

    // TODO: query for and handle error response...
    throw new Error('Could not delete outbound returns');
  },
});
