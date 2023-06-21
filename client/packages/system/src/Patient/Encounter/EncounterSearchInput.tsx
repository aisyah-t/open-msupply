import React, { FC } from 'react';
import {
  Autocomplete,
  AutocompleteOptionRenderer,
  Box,
  DefaultAutocompleteItemOption,
  ProgramEnrolmentNodeStatus,
  Typography,
  useBufferState,
} from '@openmsupply-client/common';
import { usePatient } from '../api';
import {
  useProgramEnrolments,
  useDocumentRegistry,
  EncounterRegistryByProgram,
} from '@openmsupply-client/programs';

interface EncounterSearchInputProps {
  onChange: (type: EncounterRegistryByProgram) => void;
  width?: number;
  value: EncounterRegistryByProgram | null;
  disabled?: boolean;
}

export const getEncounterOptionRenderer =
  (): AutocompleteOptionRenderer<EncounterRegistryByProgram> =>
  (props, node) => {
    const name = node.encounter.name ?? '';
    return (
      <DefaultAutocompleteItemOption {...props} key={props.id}>
        <Box display="flex" alignItems="flex-end" gap={1} height={25}>
          <Typography
            sx={{
              color:
                node.program.status === ProgramEnrolmentNodeStatus.Active
                  ? 'black'
                  : 'red',
            }}
          >
            {name}
          </Typography>
        </Box>
      </DefaultAutocompleteItemOption>
    );
  };

export const EncounterSearchInput: FC<EncounterSearchInputProps> = ({
  onChange,
  width = 250,
  value,
  disabled = false,
}) => {
  const patientId = usePatient.utils.id();
  const { data: enrolmentData, isLoading: isEnrolmentDataLoading } =
    useProgramEnrolments.document.list({
      filterBy: {
        patientId: { equalTo: patientId },
      },
    });
  const { data: encounterData, isLoading: isEncounterLoading } =
    useDocumentRegistry.get.encounterRegistriesByPrograms(
      enrolmentData?.nodes ?? []
    );
  const [buffer, setBuffer] = useBufferState(value);
  const EncounterOptionRenderer = getEncounterOptionRenderer();

  return (
    <Autocomplete
      disabled={disabled}
      clearable={false}
      value={
        buffer && {
          ...buffer,
          label: buffer.encounter.name ?? '',
        }
      }
      loading={isEnrolmentDataLoading || isEncounterLoading}
      onChange={(_, name) => {
        setBuffer(name);
        name && onChange(name);
      }}
      options={encounterData ?? []}
      renderOption={EncounterOptionRenderer}
      width={`${width}px`}
      popperMinWidth={width}
      isOptionEqualToValue={(option, value) =>
        option.encounter.id === value.encounter.id
      }
    />
  );
};
