import {
  useAuthState,
  useOmSupplyApi,
  useQuery,
} from '@openmsupply-client/common';
import { DashboardApi, getDashboardQueries } from './api';
import { getSdk } from './operations.generated';

export const useDashboardApi = (): DashboardApi => {
  const { client } = useOmSupplyApi();
  const { storeId } = useAuthState();
  const queries = getDashboardQueries(getSdk(client), storeId);
  return { ...queries, storeId: storeId };
};

export const useStockCounts = () => {
  const api = useDashboardApi();
  const { storeId } = useAuthState();
  return useQuery(['dashboard', 'stock-counts', storeId], () =>
    api.get.stockCounts()
  );
};

export const useItemStats = () => {
  const api = useDashboardApi();
  const { storeId } = useAuthState();
  return useQuery(['dashboard', 'item-stats', storeId], () =>
    api.get.itemStats()
  );
};