import React, { FC, ReactNode, useState, useEffect } from 'react';
import TabContext from '@mui/lab/TabContext';
import { Box } from '@mui/material';
import {
  useConfirmOnLeaving,
  UrlQueryObject,
  UrlQuerySort,
  useDetailPanelStore,
  useDrawer,
} from '@common/hooks';
import { LocaleKey, useTranslation } from '@common/intl';
import { debounce } from '@common/utils';
import { AppBarTabsPortal } from '../../portals';
import { DetailTab } from './DetailTab';
import { ShortTabList, Tab } from './Tabs';
import { useUrlQuery } from '@common/hooks';

export type TabDefinition = {
  Component: ReactNode;
  value: string;
  confirmOnLeaving?: boolean;
  sort?: UrlQuerySort;
};
interface DetailTabsProps {
  tabs: TabDefinition[];
  requiresConfirmation?: (tab: string) => boolean;
}
export const DetailTabs: FC<DetailTabsProps> = ({
  tabs,
  requiresConfirmation = () => false,
}) => {
  const { urlQuery, updateQuery } = useUrlQuery();
  const [currentTab, setCurrentTab] = useState<string>(tabs[0]?.value ?? '');
  const t = useTranslation('common');

  // Inelegant hack to force the "Underline" indicator for the currently active
  // tab to re-render in the correct position when one of the side "drawers" is
  // expanded. See issue #777 for more detail.
  const { isOpen: detailPanelOpen } = useDetailPanelStore();
  const { isOpen: drawerOpen } = useDrawer();
  const { showConfirmation } = useConfirmOnLeaving(false);
  const handleResize = debounce(
    () => window.dispatchEvent(new Event('resize')),
    100
  );
  useEffect(() => {
    handleResize();
  }, [detailPanelOpen, drawerOpen]);

  const onChange = (_: React.SyntheticEvent, tab: string) => {
    const tabConfirm = tabs.find(({ value }) => value === currentTab);
    const tabDefinition = tabs.find(({ value }) => value === tab);
    const sort = tabDefinition?.sort;
    const query: UrlQueryObject = sort
      ? { tab, sort: sort.key, dir: sort.dir }
      : { tab };

    if (!!tabConfirm?.confirmOnLeaving && requiresConfirmation(currentTab)) {
      showConfirmation(() => updateQuery(query));
    } else {
      updateQuery(query);
    }
  };

  const isValidTab = (tab?: string) =>
    !!tab && tabs.some(({ value }) => value === tab);

  useEffect(() => {
    const tab = urlQuery['tab'];
    if (isValidTab(tab)) {
      setCurrentTab(tab);
    }
  }, [urlQuery]);

  return (
    <TabContext value={currentTab}>
      <AppBarTabsPortal
        sx={{
          display: 'flex',
          flex: 1,
          justifyContent: 'center',
        }}
      >
        <Box flex={1}>
          <ShortTabList value={currentTab} centered onChange={onChange}>
            {tabs.map(({ value }, index) => (
              <Tab
                key={value}
                value={value}
                label={t(`label.${value.toLowerCase()}` as LocaleKey, {
                  defaultValue: value,
                })}
                tabIndex={index === 0 ? -1 : undefined}
              ></Tab>
            ))}
          </ShortTabList>
        </Box>
      </AppBarTabsPortal>
      {tabs.map(({ Component, value }) => (
        <DetailTab value={value} key={value}>
          {Component}
        </DetailTab>
      ))}
    </TabContext>
  );
};
