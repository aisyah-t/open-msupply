import React, { ReactElement } from 'react';
import {
  RecordWithId,
  CellProps,
  InnerBasicCell,
  Select,
} from '@openmsupply-client/common';
import { useUnitVariant } from '../../context';

// Drop down selector for unit variant
export const getPackUnitSelectCell =
  <T extends RecordWithId>({
    getItemId,
    getUnitName,
  }: {
    getItemId: (row: T) => string;
    getUnitName: (row: T) => string;
  }) =>
  ({ isError, rowData }: CellProps<T>): ReactElement => {
    const { variantsControll } = useUnitVariant(getItemId(rowData));

    if (!variantsControll) {
      return <InnerBasicCell isError={isError} value={getUnitName(rowData)} />;
    }

    const { variants, activeVariant, setUserSelectedVariant } =
      variantsControll;

    return (
      <Select
        options={variants.map(v => ({ label: v.shortName, value: v.id }))}
        value={activeVariant.id}
        onClick={e => {
          e.stopPropagation();
        }}
        onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
          setUserSelectedVariant(e.target.value);
        }}
      />
    );
  };
