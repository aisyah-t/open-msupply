import {
  useColumns,
  GenericColumnKey,
  SortBy,
  Column,
  useCurrency,
  TooltipTextCell,
  useColumnUtils,
  NumberCell,
  getRowExpandColumn,
  ArrayUtils,
  ColumnAlign,
  ColumnDescription,
} from '@openmsupply-client/common';
import { SupplierReturnLineFragment } from '../api';
import { SupplierReturnItem } from '../../types';

interface UseSupplierColumnOptions {
  sortBy: SortBy<SupplierReturnLineFragment | SupplierReturnItem>;
  onChangeSortBy: (sort: string, dir: 'desc' | 'asc') => void;
}

const expansionColumn = getRowExpandColumn<
  SupplierReturnLineFragment | SupplierReturnItem
>();
// const notePopoverColumn = getNotePopoverColumn<
//   StockOutLineFragment | StockOutItem
// >();

const getUnitQuantity = (row: SupplierReturnLineFragment) =>
  row.packSize * row.numberOfPacks;

export const useSupplierReturnColumns = ({
  sortBy,
  onChangeSortBy,
}: UseSupplierColumnOptions): Column<
  SupplierReturnLineFragment | SupplierReturnItem
>[] => {
  const { c } = useCurrency();
  const { getColumnProperty, getColumnPropertyAsString } = useColumnUtils();

  const columns: ColumnDescription<
    SupplierReturnLineFragment | SupplierReturnItem
  >[] = [
    //   [
    //     notePopoverColumn,
    //     {
    //       accessor: ({ rowData }) => {
    //         if ('lines' in rowData) {
    //           const { lines } = rowData;
    //           const noteSections = lines
    //             .map(({ batch, note }) => ({
    //               header: batch ?? '',
    //               body: note ?? '',
    //             }))
    //             .filter(({ body }) => !!body);
    //           return noteSections.length ? noteSections : null;
    //         } else {
    //           return rowData.batch && rowData.note
    //             ? { header: rowData.batch, body: rowData.note }
    //             : null;
    //         }
    //       },
    //     },
    //   ],
    [
      'itemCode',
      {
        getSortValue: row =>
          getColumnPropertyAsString(row, [
            { path: ['lines', 'itemCode'] },
            { path: ['itemCode'], default: '' },
          ]),
        accessor: ({ rowData }) =>
          getColumnProperty(rowData, [
            { path: ['lines', 'itemCode'] },
            { path: ['itemCode'], default: '' },
          ]),
      },
    ],
    [
      'itemName',
      {
        Cell: TooltipTextCell,
        getSortValue: row =>
          getColumnPropertyAsString(row, [
            { path: ['lines', 'itemName'] },
            { path: ['itemName'], default: '' },
          ]),
        accessor: ({ rowData }) =>
          getColumnProperty(rowData, [
            { path: ['lines', 'itemName'] },
            { path: ['itemName'], default: '' },
          ]),
      },
    ],
    //   [
    //     'itemUnit',
    //     {
    //       getSortValue: row =>
    //         getColumnPropertyAsString(row, [
    //           { path: ['lines', 'item', 'unitName'] },
    //           { path: ['item', 'unitName'], default: '' },
    //         ]),
    //       accessor: ({ rowData }) =>
    //         getColumnProperty(rowData, [
    //           { path: ['lines', 'item', 'unitName'] },
    //           { path: ['item', 'unitName'], default: '' },
    //         ]),
    //     },
    //   ],
    [
      'batch',
      {
        getSortValue: row =>
          getColumnPropertyAsString(row, [{ path: ['batch'] }]),
        accessor: ({ rowData }) =>
          getColumnProperty(rowData, [{ path: ['batch'] }]),
      },
    ],
    [
      'expiryDate',
      {
        getSortValue: row =>
          getColumnPropertyAsString(row, [{ path: ['expiryDate'] }]),
        accessor: ({ rowData }) =>
          getColumnProperty(rowData, [{ path: ['expiryDate'] }]),
      },
    ],
    //   [
    //     'location',
    //     {
    //       getSortValue: row =>
    //         getColumnPropertyAsString(row, [
    //           { path: ['lines', 'location', 'code'] },
    //           { path: ['location', 'code'], default: '' },
    //         ]),
    //       accessor: ({ rowData }) =>
    //         getColumnProperty(rowData, [
    //           { path: ['lines', 'location', 'code'] },
    //           { path: ['location', 'code'], default: '' },
    //         ]),
    //     },
    //   ],
  ];

  columns.push(
    [
      'itemUnit',
      {
        getSortValue: row =>
          getColumnPropertyAsString(row, [
            { path: ['lines', 'item', 'unitName'] },
            { path: ['item', 'unitName'], default: '' },
          ]),
        accessor: ({ rowData }) =>
          getColumnProperty(rowData, [
            { path: ['lines', 'item', 'unitName'] },
            { path: ['item', 'unitName'], default: '' },
          ]),
      },
    ],
    [
      'packSize',
      {
        getSortValue: row => {
          if ('lines' in row) {
            const { lines } = row;
            return ArrayUtils.ifTheSameElseDefault(lines, 'packSize', '') ?? '';
          } else {
            return row.packSize ?? '';
          }
        },
        accessor: ({ rowData }) => {
          if ('lines' in rowData) {
            const { lines } = rowData;
            return ArrayUtils.ifTheSameElseDefault(lines, 'packSize', '');
          } else {
            return rowData.packSize;
          }
        },
      },
    ]
  );

  columns.push(
    [
      'numberOfPacks',
      {
        Cell: NumberCell,
        accessor: ({ rowData }) => {
          if ('lines' in rowData) {
            const { lines } = rowData;
            return ArrayUtils.getSum(lines, 'numberOfPacks');
          } else {
            return rowData.numberOfPacks;
          }
        },
        getSortValue: rowData => {
          if ('lines' in rowData) {
            const { lines } = rowData;
            return ArrayUtils.getSum(lines, 'numberOfPacks');
          } else {
            return rowData.numberOfPacks;
          }
        },
      },
    ],
    [
      'unitQuantity',
      {
        accessor: ({ rowData }) => {
          if ('lines' in rowData) {
            const { lines } = rowData;
            return ArrayUtils.getUnitQuantity(lines);
          } else {
            return getUnitQuantity(rowData);
          }
        },
        getSortValue: rowData => {
          if ('lines' in rowData) {
            const { lines } = rowData;
            return ArrayUtils.getUnitQuantity(lines);
          } else {
            return getUnitQuantity(rowData);
          }
        },
      },
    ],
    {
      label: 'label.unit-price',
      key: 'sellPricePerUnit',
      align: ColumnAlign.Right,
      accessor: ({ rowData }) => {
        if ('lines' in rowData) {
          return c(
            Object.values(rowData.lines).reduce(
              (sum, batch) =>
                sum + (batch.sellPricePerPack ?? 0) / batch.packSize,
              0
            )
          ).format();
        } else {
          return c((rowData.sellPricePerPack ?? 0) / rowData.packSize).format();
        }
      },
      getSortValue: rowData => {
        if ('lines' in rowData) {
          return c(
            Object.values(rowData.lines).reduce(
              (sum, batch) =>
                sum + (batch.sellPricePerPack ?? 0) / batch.packSize,
              0
            )
          ).format();
        } else {
          return c((rowData.sellPricePerPack ?? 0) / rowData.packSize).format();
        }
      },
    },
    {
      label: 'label.line-total',
      key: 'lineTotal',
      align: ColumnAlign.Right,
      accessor: ({ rowData }) => {
        if ('lines' in rowData) {
          return c(
            Object.values(rowData.lines).reduce(
              (sum, batch) =>
                sum + batch.sellPricePerPack * batch.numberOfPacks,
              0
            )
          ).format();
        } else {
          const x = c(
            rowData.sellPricePerPack * rowData.numberOfPacks
          ).format();
          return x;
        }
      },
      getSortValue: row => {
        if ('lines' in row) {
          return c(
            Object.values(row.lines).reduce(
              (sum, batch) =>
                sum + batch.sellPricePerPack * batch.numberOfPacks,
              0
            )
          ).format();
        } else {
          const x = c(row.sellPricePerPack * row.numberOfPacks).format();
          return x;
        }
      },
    },
    expansionColumn,
    GenericColumnKey.Selection
  );

  return useColumns(columns, { onChangeSortBy, sortBy }, [sortBy]);
};

export const useExpansionColumns = (): Column<SupplierReturnLineFragment>[] =>
  useColumns(['batch', 'expiryDate', 'numberOfPacks', 'packSize']);
