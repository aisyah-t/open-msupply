import {
  BasicSpinner,
  Checkbox,
  createTableStore,
  DataTable,
  DialogButton,
  TableProvider,
  useColumns,
  useDialog,
  useTranslation,
} from '@openmsupply-client/common';
import React, { useState } from 'react';
import { StockLineFragment, useStockLines } from '../../Item';
import { VaccinationStockLine } from '../api';

interface SelectBatchModalProps {
  itemId: string;
  stockLine: VaccinationStockLine | null;
  isOpen: boolean;
  onClose: () => void;
  setStockLine: (stockLine: VaccinationStockLine) => void;
}

export const SelectBatchModal = ({
  itemId,
  isOpen,
  stockLine,
  onClose,
  setStockLine,
}: SelectBatchModalProps) => {
  const t = useTranslation();
  const { data, isLoading } = useStockLines(itemId);

  const [selectedBatch, setSelectedBatch] =
    useState<VaccinationStockLine | null>(stockLine);

  const { Modal } = useDialog({ isOpen, onClose, disableBackdrop: true });

  const onOk = () => {
    if (selectedBatch) {
      setStockLine(selectedBatch);
      onClose();
    }
  };

  const columns = useColumns<StockLineFragment>(
    [
      {
        key: 'select',
        Cell: ({ rowData }) => (
          <Checkbox
            checked={rowData.id === selectedBatch?.id}
            onClick={() => setSelectedBatch(rowData)}
          />
        ),
      },
      'batch',
      'expiryDate',
      ['code', { accessor: ({ rowData }) => rowData.item.code }],

      {
        key: 'availableNumberOfPacks',
        label: 'label.available-packs',
        accessor: ({ rowData }) => rowData.availableNumberOfPacks,
      },
      {
        key: 'doses',
        label: 'label.doses',
        accessor: ({ rowData }) =>
          rowData.item.doses *
          rowData.availableNumberOfPacks *
          rowData.packSize,
      },
    ],
    {},
    [itemId, selectedBatch]
  );

  return (
    <TableProvider createStore={createTableStore}>
      <Modal
        title={t('label.available-batches')}
        cancelButton={<DialogButton variant="cancel" onClick={onClose} />}
        okButton={
          <DialogButton disabled={!selectedBatch} variant="ok" onClick={onOk} />
        }
        height={500}
        width={750}
        slideAnimation={false}
      >
        {isLoading ? (
          <BasicSpinner />
        ) : (
          <DataTable
            id="vaccination-batches"
            columns={columns}
            data={data?.nodes ?? []}
            noDataMessage={t('messages.no-stock-available')}
            onRowClick={row => setSelectedBatch(row)}
            dense
          />
        )}
      </Modal>
    </TableProvider>
  );
};
