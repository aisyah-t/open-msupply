import React, { FC, useEffect, useRef } from 'react';
import {
  useNotification,
  DropdownMenu,
  DropdownMenuItem,
  useTranslation,
  DeleteIcon,
  useTableStore,
  AppBarContentPortal,
  SearchBar,
  FilterController,
} from '@openmsupply-client/common';
import { StocktakeRow } from '../../types';
import { canDeleteStocktake } from '../../utils';

export const Toolbar: FC<{
  onDelete: (toDelete: StocktakeRow[]) => void;
  filter: FilterController;
  data?: StocktakeRow[];
}> = ({ onDelete, data, filter }) => {
  const t = useTranslation('inventory');

  const { success, info } = useNotification();

  const { selectedRows } = useTableStore(state => ({
    selectedRows: Object.keys(state.rowState)
      .filter(id => state.rowState[id]?.isSelected)
      .map(selectedId => data?.find(({ id }) => selectedId === id))
      .filter(Boolean) as StocktakeRow[],
  }));

  const deleteAction = () => {
    const numberSelected = selectedRows.length;
    if (selectedRows && numberSelected > 0) {
      const canDeleteRows = selectedRows.every(canDeleteStocktake);
      if (!canDeleteRows) {
        const cannotDeleteSnack = info(t('message.cant-delete-stocktakes'));
        cannotDeleteSnack();
      } else {
        onDelete(selectedRows);
        const deletedMessage = t('messages.deleted-stocktakes', {
          number: numberSelected,
        });
        const successSnack = success(deletedMessage);
        successSnack();
      }
    } else {
      const selectRowsSnack = info(t('message.select-rows-to-delete'));
      selectRowsSnack();
    }
  };

  const ref = useRef(deleteAction);

  useEffect(() => {
    ref.current = deleteAction;
  }, [selectedRows]);

  const key = 'comment' as keyof StocktakeRow;
  const filterString = filter.filterBy?.[key]?.like as string;

  return (
    <AppBarContentPortal
      sx={{
        paddingBottom: '16px',
        flex: 1,
        justifyContent: 'space-between',
        display: 'flex',
      }}
    >
      <SearchBar
        placeholder="Search by comment..."
        value={filterString}
        onChange={newValue => {
          filter.onChangeStringFilterRule('comment', 'like', newValue);
        }}
      />

      <DropdownMenu label="Select">
        <DropdownMenuItem IconComponent={DeleteIcon} onClick={deleteAction}>
          {t('button.delete-lines')}
        </DropdownMenuItem>
      </DropdownMenu>
    </AppBarContentPortal>
  );
};