import React, { FC, useState } from 'react';
import {
  useTranslation,
  DataTable,
  Box,
  BaseButton,
  DialogButton,
  Typography,
  useDialog,
  TableProvider,
  createTableStore,
  Grid,
  useNotification,
  getErrorMessage,
  noOtherVariants,
  ReportContext,
  LoadingButton,
  PrinterIcon,
} from '@openmsupply-client/common';
import { RepackEditForm } from './RepackEditForm';
import {
  Repack,
  ReportRowFragment,
  ReportSelector,
  useReport,
  useStock,
} from '@openmsupply-client/system';
import { RepackFragment, StockLineRowFragment } from '../../api';
import { useRepackColumns } from './column';

interface RepackModalControlProps {
  isOpen: boolean;
  onClose: () => void;
  stockLine: StockLineRowFragment | null;
}

const useDraftRepack = (seed: Repack) => {
  const [repack, setRepack] = useState<Repack>(() => ({ ...seed }));
  const { mutateAsync, isLoading, isError } = useStock.repack.insert(
    seed.stockLineId ?? ''
  );

  const onChange = (patch: Partial<Repack>) => {
    setRepack({ ...repack, ...patch });
  };

  const onInsert = async () => mutateAsync(repack);

  return {
    onChange,
    onInsert,
    isLoading,
    draft: repack,
    isError,
  };
};

export const RepackModal: FC<RepackModalControlProps> = ({
  isOpen,
  onClose,
  stockLine,
}) => {
  const t = useTranslation('inventory');
  const { error, success } = useNotification();
  const { Modal } = useDialog({ isOpen, onClose });

  const [invoiceId, setInvoiceId] = useState<string | undefined>(undefined);
  const [isNew, setIsNew] = useState<boolean>(false);
  const defaultRepack = {
    stockLineId: stockLine?.id,
    newPackSize: 0,
    numberOfPacks: 0,
  };

  const { data, isError, isLoading } = useStock.repack.list(
    stockLine?.id ?? ''
  );
  const { draft, onChange, onInsert } = useDraftRepack(defaultRepack);
  const { columns } = useRepackColumns();
  const displayMessage = invoiceId == undefined && !isNew;
  const showRepackDetail = invoiceId || isNew;

  const { print, isPrinting } = useReport.utils.print();

  const printReport = (report: ReportRowFragment) => {
    if (!data) return;
    print({ reportId: report.id, dataId: invoiceId || '' });
  };

  const onRowClick = (rowData: RepackFragment) => {
    onChange(defaultRepack);
    setInvoiceId(rowData.id);
    setIsNew(false);
  };

  const onNewClick = () => {
    onChange(defaultRepack);
    setInvoiceId(undefined);
    setIsNew(true);
  };

  const mapStructuredErrors = (
    result: Awaited<ReturnType<typeof onInsert>>
  ): string | undefined => {
    if (result.__typename === 'InvoiceNode') {
      return undefined;
    }

    const { error: repackError } = result;

    switch (repackError.__typename) {
      case 'StockLineReducedBelowZero':
        return t('error.repack-has-stock-reduced-below-zero');
      case 'CannotHaveFractionalPack':
        return t('error.repack-cannot-be-fractional');
      default:
        noOtherVariants(repackError);
    }
  };

  return (
    <Modal
      width={900}
      height={700}
      slideAnimation={false}
      title={t('title.repack-details')}
      okButton={
        <DialogButton
          variant="save"
          disabled={draft?.newPackSize === 0 || draft?.numberOfPacks === 0}
          onClick={async () => {
            try {
              const result = await onInsert();
              const errorMessage = mapStructuredErrors(result);

              if (errorMessage) {
                error(errorMessage)();
              } else {
                onChange(defaultRepack);
                if (stockLine?.totalNumberOfPacks === draft.numberOfPacks) {
                  onClose();
                  success(t('messages.all-packs-repacked'))();
                } else {
                  success(t('messages.saved'))();
                }
              }
            } catch (e) {
              error(getErrorMessage(e))();
            }
          }}
        />
      }
      cancelButton={<DialogButton variant="cancel" onClick={onClose} />}
      reportSelector={
        <ReportSelector
          context={ReportContext.Repack}
          onPrint={printReport}
          disabled={!invoiceId}
        >
          <LoadingButton
            sx={{ marginLeft: 1 }}
            variant="outlined"
            startIcon={<PrinterIcon />}
            isLoading={isPrinting}
            disabled={!invoiceId}
          >
            {t('button.print')}
          </LoadingButton>
        </ReportSelector>
      }
    >
      <Box>
        <Grid
          container
          paddingBottom={1}
          alignItems="center"
          flexDirection="column"
        >
          <Typography sx={{ fontWeight: 'bold' }} variant="h6">
            {stockLine?.item.name}
          </Typography>
          <Typography sx={{ fontWeight: 'bold', marginBottom: 3 }}>
            {`${t('label.code')} : ${stockLine?.item.code}`}
          </Typography>
        </Grid>
        <Box display={'flex'}>
          <Box display={'flex'} flexDirection={'column'} width={'500px'}>
            <Box paddingBottom={2}>
              <TableProvider createStore={createTableStore}>
                <DataTable
                  id="repack-list"
                  columns={columns}
                  data={data?.nodes}
                  isLoading={isLoading}
                  isError={isError}
                  noDataMessage={t('messages.no-repacks')}
                  overflowX="auto"
                  onRowClick={onRowClick}
                />
              </TableProvider>
            </Box>
            <Box display="flex" justifyContent="center">
              <BaseButton onClick={onNewClick}>{t('label.new')}</BaseButton>
            </Box>
          </Box>
          <Box paddingLeft={3} width={'400px'}>
            {displayMessage && (
              <Typography>{t('messages.no-repack-detail')}</Typography>
            )}
            {showRepackDetail && (
              <RepackEditForm
                invoiceId={invoiceId}
                onChange={onChange}
                stockLine={stockLine}
                draft={draft}
              />
            )}
          </Box>
        </Box>
      </Box>
    </Modal>
  );
};
