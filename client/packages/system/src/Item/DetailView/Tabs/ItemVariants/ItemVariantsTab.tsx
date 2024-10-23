import React, { useState } from 'react';

import {
  AppBarButtonsPortal,
  BasicTextInput,
  ButtonWithIcon,
  InputWithLabelRow,
  Typography,
} from '@common/components';
import {
  Box,
  useTranslation,
  PlusCircleIcon,
} from '@openmsupply-client/common';
import { ItemPackagingVariantsTable } from './ItemPackagingVariantsTable';
import { ItemVariantFragment, PackagingVariantFragment } from '../../../api';

const packVariants: PackagingVariantFragment[] = [
  {
    __typename: 'PackagingVariantNode',
    id: '1',
    packagingLevel: 1,
    name: 'Primary',
    packSize: 1,
    volumePerUnit: 1,
  },
  {
    __typename: 'PackagingVariantNode',
    id: '2',
    packagingLevel: 2,
    name: 'Secondary',
    packSize: 2,
    volumePerUnit: 2,
  },
  {
    __typename: 'PackagingVariantNode',
    id: '3',
    packagingLevel: 3,
    name: 'Tertiary',
    packSize: 3,
    volumePerUnit: 3,
  },
];

export const ItemVariantsTab = ({
  itemVariants,
}: {
  itemVariants: ItemVariantFragment[];
}) => {
  const t = useTranslation();
  const [variants, setVariants] = useState<ItemVariantFragment[]>(itemVariants);

  const setVariant = (variant: ItemVariantFragment) => {
    const idx = variants.findIndex(v => v.id === variant.id);

    if (idx !== -1) {
      variants[idx] = variant;
      setVariants([...variants]);
    }
  };

  return (
    <>
      <AppBarButtonsPortal>
        <ButtonWithIcon
          Icon={<PlusCircleIcon />}
          onClick={() => {
            setVariants([
              ...variants,
              {
                __typename: 'ItemVariantNode',
                id: Date.now().toString(),
                name: '',
                packagingVariants: [...packVariants],
              },
            ]);
          }}
          label={t('label.add-variant')}
        />
      </AppBarButtonsPortal>
      <Box flex={1} marginX={2}>
        {variants.map((v, idx) => (
          <ItemVariant variant={v} setVariant={setVariant} index={idx} />
        ))}
      </Box>
      {/* // TODO footer */}
    </>
  );
};

const ItemVariant = ({
  variant,
  index,
  setVariant,
}: {
  variant: ItemVariantFragment;
  index: number;
  setVariant: (variant: ItemVariantFragment) => void;
}) => {
  const t = useTranslation();

  return (
    <Box maxWidth="1000px" margin="25px auto">
      <Typography variant="h5" fontWeight="bold">
        {variant.name || `${t('label.variant')} ${index + 1}`}
      </Typography>

      <Box justifyContent="center" display="flex" gap={2}>
        <Box
          marginTop={5}
          display="flex"
          flexDirection="column"
          gap={1}
          flex={1}
        >
          <InputWithLabelRow
            label={t('label.name')}
            labelWidth="200"
            Input={
              <BasicTextInput
                value={variant.name}
                onChange={event => {
                  setVariant({ ...variant, name: event.target.value });
                }}
                fullWidth
              />
            }
          />

          <InputWithLabelRow
            label={t('label.cold-storage-type')}
            labelWidth="200"
            Input={
              // TODO: temp range dropdown
              <BasicTextInput
                value={variant.coldStorageTypeId}
                onChange={event => {
                  setVariant({
                    ...variant,
                    coldStorageTypeId: event.target.value,
                  });
                }}
                fullWidth
              />
            }
          />
          <InputWithLabelRow
            label={t('label.manufacturer')}
            labelWidth="200"
            Input={
              // TODO ManufacturerSearch
              <BasicTextInput
                value={variant.manufacturerId}
                onChange={event => {
                  setVariant({
                    ...variant,
                    manufacturerId: event.target.value,
                  });
                }}
                fullWidth
              />
            }
          />
        </Box>
        <Box flex={1}>
          <Typography fontWeight="bold">{t('title.packaging')}</Typography>
          <ItemPackagingVariantsTable data={variant.packagingVariants} />
        </Box>
      </Box>
    </Box>
  );
};
