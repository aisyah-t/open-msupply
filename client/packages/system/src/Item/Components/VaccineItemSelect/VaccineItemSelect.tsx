import React from 'react';
import {
  AutocompleteOptionRenderer,
  FnUtils,
  AutocompleteMulti,
} from '@openmsupply-client/common';
import { useDemographicData } from '@openmsupply-client/system';
import {
  DraftVaccineCourse,
  DraftVaccineCourseItem,
} from '@openmsupply-client/system/src/Immunisation';

interface VaccineItemSelectProps {
  extraFilter?: (item: DraftVaccineCourseItem) => boolean;
  onChange: (newData: Partial<DraftVaccineCourse>) => void;
  draft: DraftVaccineCourse;
}

const renderOption: AutocompleteOptionRenderer<DraftVaccineCourseItem> = (
  props,
  option
): JSX.Element => {
  return (
    <li {...props}>
      <span
        style={{
          fontWeight: 700,
          whiteSpace: 'nowrap',
          width: 100,
        }}
      >
        {option.item.name ?? ''}
      </span>
    </li>
  );
};

export const VaccineItemSelect = ({
  // extraFilter,
  onChange,
  draft,
}: VaccineItemSelectProps) => {
  const { data } = useDemographicData.vaccineItems.get();

  const onChangeSelectedItems = (selectedItems: DraftVaccineCourseItem[]) => {
    onChange({ vaccineCourseItems: selectedItems });
  };

  const options: DraftVaccineCourseItem[] =
    data?.nodes?.map(item => {
      const vaccineItem: DraftVaccineCourseItem = {
        id: FnUtils.generateUUID(),
        item: {
          id: item.id,
          name: item.name,
        },
      };
      return vaccineItem;
    }) ?? [];

  return (
    <AutocompleteMulti
      isOptionEqualToValue={(option, value) => option.item.id === value.item.id}
      getOptionLabel={option => `${option.item.name}`}
      value={draft?.vaccineCourseItems ?? []}
      filterSelectedOptions
      onChange={(_event, newSelectedItems) =>
        onChangeSelectedItems(newSelectedItems)
      }
      options={options}
      renderOption={renderOption}
      inputProps={{ fullWidth: true }}
    />
  );
};
