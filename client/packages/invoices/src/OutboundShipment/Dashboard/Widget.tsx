import React from 'react';
import { CustomerSearchModal } from '@openmsupply-client/system';
import {
  ButtonWithIcon,
  Grid,
  PlusCircleIcon,
  useNotification,
  StatsPanel,
  Widget,
  FnUtils,
  useToggle,
  ApiException,
} from '@openmsupply-client/common';
import { useFormatNumber, useTranslation } from '@common/intl';
import { useOutbound } from '../api';

export const OutboundShipmentWidget: React.FC = () => {
  const modalControl = useToggle(false);
  const { error: errorNotification } = useNotification();
  const t = useTranslation(['app', 'dashboard']);
  const formatNumber = useFormatNumber();
  const {
    data: outboundCount,
    isLoading: isOutboundCountLoading,
    isError: isOutboundCountError,
    error: outboundCountError,
  } = useOutbound.utils.count();
  const {
    data: responseCount,
    isLoading: isResponseCountLoading,
    isError: isResponseCountError,
    error: responseCountError,
  } = useOutbound.utils.responseCount();

  const { mutateAsync: onCreate } = useOutbound.document.insert();
  const onError = (e: unknown) => {
    const message = (e as Error).message ?? '';
    const errorSnack = errorNotification(
      `Failed to create invoice! ${message}`
    );
    errorSnack();
  };

  return (
    <>
      {modalControl.isOn ? (
        <CustomerSearchModal
          open={true}
          onClose={modalControl.toggleOff}
          onChange={async ({ id: otherPartyId }) => {
            modalControl.toggleOff();
            await onCreate(
              {
                id: FnUtils.generateUUID(),
                otherPartyId,
              },
              { onError }
            );
          }}
        />
      ) : null}
      <Widget title={t('distribution')}>
        <Grid
          container
          justifyContent="flex-start"
          flex={1}
          flexDirection="column"
        >
          <Grid item>
            <StatsPanel
              error={outboundCountError as ApiException}
              isError={isOutboundCountError}
              isLoading={isOutboundCountLoading}
              title={t('heading.shipments-not-shipped')}
              stats={[
                {
                  label: t('label.today', { ns: 'dashboard' }),
                  value: formatNumber.round(outboundCount?.notShipped),
                },
              ]}
            />
          </Grid>
          <Grid item>
            <StatsPanel
              error={responseCountError as ApiException}
              isError={isResponseCountError}
              isLoading={isResponseCountLoading}
              title={t('heading.new-requisitions', { ns: 'dashboard' })}
              stats={[
                {
                  label: t('label.today', { ns: 'dashboard' }),
                  value: formatNumber.round(
                    responseCount?.newResponseRequisitionCount
                  ),
                },
              ]}
            />
          </Grid>
          <Grid
            item
            flex={1}
            container
            justifyContent="flex-end"
            alignItems="flex-end"
          >
            <ButtonWithIcon
              variant="contained"
              color="secondary"
              Icon={<PlusCircleIcon />}
              label={t('button.new-outbound-shipment')}
              onClick={modalControl.toggleOn}
            />
          </Grid>
        </Grid>
      </Widget>
    </>
  );
};
