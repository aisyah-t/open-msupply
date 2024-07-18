import React from 'react';
import {
  TableProvider,
  DataTable,
  useColumns,
  useUrlQueryParams,
  useNavigate,
  NothingHere,
  useTranslation,
  createTableStore,
  createQueryParamsStore,
} from '@openmsupply-client/common';
import { Toolbar } from './Toolbar';
import { AppBarButtons } from './AppBarButtons';
import { useRnRFormList } from '../../api';
import { RnRFormFragment } from '../../api/operations.generated';

const RnRFormListComponent = () => {
  const {
    updateSortQuery,
    updatePaginationQuery,
    queryParams: { sortBy, page, first, offset, filterBy },
  } = useUrlQueryParams({ filters: [{ key: 'name' }] });
  const pagination = { page, first, offset };
  const navigate = useNavigate();
  const t = useTranslation('programs');

  const queryParams = {
    filterBy,
    offset,
    sortBy,
    first,
  };
  const { data, isLoading, isError } = useRnRFormList(queryParams);

  const columns = useColumns<RnRFormFragment>(
    [
      {
        key: 'periodName',
        width: 350,
        label: 'label.period',
      },
      ['createdDatetime', { accessor: ({ rowData }) => rowData.createdDate }],
      {
        key: 'programName',
        label: 'label.program-name',
        sortable: false,
      },
    ],
    {
      onChangeSortBy: updateSortQuery,
      sortBy,
    },
    [updateSortQuery, sortBy]
  );

  return (
    <>
      <Toolbar />
      <AppBarButtons
        onCreate={() => {
          /* TODO */
        }}
      />
      <DataTable
        id={'rnr-form-list'}
        pagination={{ ...pagination }}
        onChangePage={updatePaginationQuery}
        columns={columns}
        data={data?.nodes ?? []}
        isLoading={isLoading}
        isError={isError}
        onRowClick={row => navigate(row.id)}
        noDataElement={<NothingHere body={t('error.no-rnr-forms')} />}
      />
    </>
  );
};

export const RnRFormListView = () => (
  <TableProvider
    createStore={createTableStore}
    queryParamsStore={createQueryParamsStore({
      initialSortBy: { key: 'name' },
    })}
  >
    <RnRFormListComponent />
  </TableProvider>
);