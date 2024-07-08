import React, { ReactElement } from 'react';
import {
  RecordWithId,
  CellProps,
  BasicCellLayout,
  Tooltip,
  Typography,
  NumUtils,
} from '@openmsupply-client/common';
import { usePackVariant } from '../../context';

// Adjust quantity to reflect selected pack variant
export const PackVariantQuantityCell =
  <T extends RecordWithId>({
    getItemId,
    getQuantity,
  }: {
    getItemId: (row: T) => string;
    getQuantity: (row: T) => number;
  }) =>
  ({ isError, rowData }: CellProps<T>): ReactElement => {
    const { numberOfPacksFromQuantity } = usePackVariant(
      getItemId(rowData),
      null
    );

    const quantity = getQuantity(rowData);
    const packQuantity = numberOfPacksFromQuantity(quantity);

    return (
      <BasicCellLayout isError={isError}>
        <Tooltip title={String(quantity)}>
          <Typography>{NumUtils.round(packQuantity, 2)}</Typography>
        </Tooltip>
      </BasicCellLayout>
    );
  };
