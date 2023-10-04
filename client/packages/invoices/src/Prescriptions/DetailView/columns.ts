import {
  Formatter,
  useColumns,
  getRowExpandColumn,
  getNotePopoverColumn,
  ColumnAlign,
  GenericColumnKey,
  SortBy,
  Column,
  ArrayUtils,
  useCurrency,
  PositiveNumberCell,
  useTranslation,
} from '@openmsupply-client/common';
import { StockOutLineFragment } from '../../StockOut';
import { StockOutItem } from '../../types';

interface UsePrescriptionColumnOptions {
  sortBy: SortBy<StockOutLineFragment | StockOutItem>;
  onChangeSortBy: (column: Column<StockOutLineFragment | StockOutItem>) => void;
}

const expansionColumn = getRowExpandColumn<
  StockOutLineFragment | StockOutItem
>();

export const usePrescriptionColumn = ({
  sortBy,
  onChangeSortBy,
}: UsePrescriptionColumnOptions): Column<
  StockOutLineFragment | StockOutItem
>[] => {
  const { c } = useCurrency();
  const t = useTranslation('dispensary');

  return useColumns(
    [
      [
        getNotePopoverColumn(t('label.directions')),
        {
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              const noteSections = lines
                .map(({ batch, note }) => ({
                  header: batch ?? '',
                  body: note ?? '',
                }))
                .filter(({ body }) => !!body);
              return noteSections.length ? noteSections : null;
            } else {
              return rowData.batch && rowData.note
                ? { header: rowData.batch, body: rowData.note }
                : null;
            }
          },
        },
      ],
      [
        'itemCode',
        {
          getSortValue: row => {
            if ('lines' in row) {
              const { lines } = row;
              const items = lines.map(({ item }) => item);
              return ArrayUtils.ifTheSameElseDefault(items, 'code', '');
            } else {
              return row.item.code;
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              const items = lines.map(({ item }) => item);
              return ArrayUtils.ifTheSameElseDefault(items, 'code', '');
            } else {
              return rowData.item.code;
            }
          },
        },
      ],
      [
        'itemName',
        {
          getSortValue: row => {
            if ('lines' in row) {
              const { lines } = row;
              const items = lines.map(({ item }) => item);
              return ArrayUtils.ifTheSameElseDefault(items, 'name', '');
            } else {
              return row.item.name;
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              const items = lines.map(({ item }) => item);
              return ArrayUtils.ifTheSameElseDefault(items, 'name', '');
            } else {
              return rowData.item.name;
            }
          },
        },
      ],
      [
        'itemUnit',
        {
          getSortValue: row => {
            if ('lines' in row) {
              return row.lines[0]?.item.unitName ?? '';
            } else {
              return row.item.unitName ?? '';
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const items = rowData.lines.map(({ item }) => item);
              return (
                ArrayUtils.ifTheSameElseDefault(items, 'unitName', '') ?? ''
              );
            } else {
              return rowData.item.unitName ?? '';
            }
          },
        },
      ],
      [
        'batch',
        {
          getSortValue: row => {
            if ('lines' in row) {
              const { lines } = row;
              return (
                ArrayUtils.ifTheSameElseDefault(
                  lines,
                  'batch',
                  `${t('multiple')}`
                ) ?? ''
              );
            } else {
              return row.batch ?? '';
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              return ArrayUtils.ifTheSameElseDefault(
                lines,
                'batch',
                `${t('multiple')}`
              );
            } else {
              return rowData.batch;
            }
          },
        },
      ],
      [
        'expiryDate',
        {
          getSortValue: row => {
            if ('lines' in row) {
              const { lines } = row;
              const expiryDate =
                ArrayUtils.ifTheSameElseDefault(lines, 'expiryDate', null) ??
                '';
              return Formatter.expiryDateString(expiryDate);
            } else {
              return Formatter.expiryDateString(row.expiryDate);
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              const expiryDate = ArrayUtils.ifTheSameElseDefault(
                lines,
                'expiryDate',
                null
              );
              return expiryDate;
            } else {
              return rowData.expiryDate;
            }
          },
        },
      ],
      [
        'locationName',
        {
          getSortValue: row => {
            if ('lines' in row) {
              const locations = row.lines.flatMap(({ location }) =>
                !!location ? [location] : []
              );
              if (locations.length !== 0) {
                return ArrayUtils.ifTheSameElseDefault(
                  locations,
                  'name',
                  `${t('multiple')}`
                );
              } else {
                return '';
              }
            } else {
              return row.location?.name ?? '';
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const locations = rowData.lines.flatMap(({ location }) =>
                !!location ? [location] : []
              );

              if (locations.length !== 0) {
                return ArrayUtils.ifTheSameElseDefault(
                  locations,
                  'name',
                  `${t('multiple')}`
                );
              }
            } else {
              return rowData.location?.name ?? '';
            }
          },
        },
      ],
      [
        'numberOfPacks',
        {
          Cell: PositiveNumberCell,
          getSortValue: row => {
            if ('lines' in row) {
              const { lines } = row;
              const packSize = ArrayUtils.ifTheSameElseDefault(
                lines,
                'packSize',
                ''
              );
              if (packSize) {
                return lines.reduce(
                  (acc, value) => acc + value.numberOfPacks,
                  0
                );
              } else {
                return '';
              }
            } else {
              return row.numberOfPacks;
            }
          },
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              const packSize = ArrayUtils.ifTheSameElseDefault(
                lines,
                'packSize',
                ''
              );
              if (packSize) {
                return lines.reduce(
                  (acc, value) => acc + value.numberOfPacks,
                  0
                );
              } else {
                return '';
              }
            } else {
              return rowData.numberOfPacks;
            }
          },
        },
      ],
      [
        'packSize',
        {
          getSortValue: row => {
            if ('lines' in row) {
              const { lines } = row;
              return (
                ArrayUtils.ifTheSameElseDefault(lines, 'packSize', '') ?? ''
              );
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
      ],
      [
        'unitQuantity',
        {
          accessor: ({ rowData }) => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              return ArrayUtils.getUnitQuantity(lines);
            } else {
              return rowData.packSize * rowData.numberOfPacks;
            }
          },
          getSortValue: rowData => {
            if ('lines' in rowData) {
              const { lines } = rowData;
              return ArrayUtils.getUnitQuantity(lines);
            } else {
              return rowData.packSize * rowData.numberOfPacks;
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
            return c(
              (rowData.sellPricePerPack ?? 0) / rowData.packSize
            ).format();
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
            return c(
              (rowData.sellPricePerPack ?? 0) / rowData.packSize
            ).format();
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
      GenericColumnKey.Selection,
    ],
    { onChangeSortBy, sortBy },
    [sortBy]
  );
};
