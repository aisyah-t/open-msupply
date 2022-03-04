import React from 'react';
import {
  DialogButton,
  Grid,
  useDialog,
  InlineSpinner,
  Box,
  useTranslation,
  ModalMode,
  useBufferState,
  useDirtyCheck,
} from '@openmsupply-client/common';
import { ItemRowFragment } from '@openmsupply-client/system';
import { OutboundLineEditTable } from './OutboundLineEditTable';
import { OutboundLineEditForm } from './OutboundLineEditForm';
import {
  useDraftOutboundLines,
  usePackSizeController,
  useNextItem,
} from './hooks';
import {
  allocateQuantities,
  sumAvailableQuantity,
  getAllocatedQuantity,
} from './utils';
import {
  useIsOutboundDisabled,
  useOutboundFields,
  useSaveOutboundLines,
} from '../../api';
interface ItemDetailsModalProps {
  isOpen: boolean;
  onClose: () => void;
  item: ItemRowFragment | null;
  mode: ModalMode | null;
}

export const OutboundLineEdit: React.FC<ItemDetailsModalProps> = ({
  isOpen,
  onClose,
  item,
  mode,
}) => {
  const t = useTranslation(['distribution']);
  const { Modal } = useDialog({ isOpen, onClose });
  const [currentItem, setCurrentItem] = useBufferState(item);

  const { mutate } = useSaveOutboundLines();
  const { status } = useOutboundFields('status');
  const isDisabled = useIsOutboundDisabled();
  const {
    draftOutboundLines,
    updateQuantity,
    setDraftOutboundLines,
    isLoading,
  } = useDraftOutboundLines(currentItem);
  const packSizeController = usePackSizeController(draftOutboundLines);
  const { next, disabled: nextDisabled } = useNextItem(currentItem?.id);
  const { setIsDirty } = useDirtyCheck();

  const onNext = () => {
    setCurrentItem(next);
    return true;
  };

  const onAllocate = (newVal: number, packSize: number | null) => {
    const newAllocateQuantities = allocateQuantities(
      status,
      draftOutboundLines
    )(newVal, packSize);
    setIsDirty(true);
    setDraftOutboundLines(newAllocateQuantities ?? draftOutboundLines);
  };

  return (
    <Modal
      title={t(
        mode === ModalMode.Update ? 'heading.edit-item' : 'heading.add-item'
      )}
      cancelButton={<DialogButton variant="cancel" onClick={onClose} />}
      nextButton={
        <DialogButton
          disabled={mode === ModalMode.Create || nextDisabled}
          variant="next"
          onClick={onNext}
        />
      }
      okButton={
        <DialogButton
          variant="ok"
          onClick={async () => {
            try {
              await mutate(draftOutboundLines);
              onClose();
            } catch (e) {
              console.log(e);
            }
          }}
        />
      }
      height={700}
      width={900}
    >
      <Grid container gap={0.5}>
        <OutboundLineEditForm
          disabled={mode === ModalMode.Update || isDisabled}
          packSizeController={packSizeController}
          onChangeItem={setCurrentItem}
          item={currentItem}
          allocatedQuantity={getAllocatedQuantity(draftOutboundLines)}
          availableQuantity={sumAvailableQuantity(draftOutboundLines)}
          onChangeQuantity={onAllocate}
        />
        {!!currentItem ? (
          !isLoading ? (
            <OutboundLineEditTable
              packSizeController={packSizeController}
              onChange={updateQuantity}
              rows={draftOutboundLines}
            />
          ) : (
            <Box
              display="flex"
              flex={1}
              height={400}
              justifyContent="center"
              alignItems="center"
            >
              <InlineSpinner />
            </Box>
          )
        ) : null}
      </Grid>
    </Modal>
  );
};