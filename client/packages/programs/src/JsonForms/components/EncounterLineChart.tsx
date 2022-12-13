import React, { useEffect, useState } from 'react';
import { rankWith, uiTypeIs, ControlProps } from '@jsonforms/core';
import { withJsonFormsControlProps } from '@jsonforms/react';
import { Box, FormLabel } from '@mui/material';
import {
  Line,
  LineChart,
  useFormatDateTime,
  XAxis,
  YAxis,
} from '@openmsupply-client/common';
import { useZodOptionsValidation } from '../common';
import { CartesianGrid, Tooltip, TooltipProps, Label } from 'recharts';
import { useEncounter } from '../../api';
import { z } from 'zod';

export const encounterLineChartTester = rankWith(
  4,
  uiTypeIs('EncounterLineChart')
);

type Options = {
  values: ValueOption[];
};

type ValueOption = {
  field: string;
  label: string;
  unit: string;
};

const ValueOption: z.ZodType<ValueOption> = z
  .object({
    field: z.string(),
    label: z.string(),
    unit: z.string(),
  })
  .strict();

const Options: z.ZodType<Options> = z
  .object({
    values: z.array(ValueOption),
  })
  .strict();

type DateTimeTooltipProps = TooltipProps<string, string> & {
  name: string;
  unit: string;
};

const DateTimeTooltip = (props: DateTimeTooltipProps) => {
  const { localisedDateTime } = useFormatDateTime();

  if (!props.active || props.payload?.[0] === undefined) {
    return null;
  }
  return (
    <Box padding={0.5} border={1} borderColor={'lightgray'}>
      <p>{`${localisedDateTime(props.label)}`}</p>
      <p>{`${props.name}: ${props.payload[0].value} ${props.unit}`}</p>
    </Box>
  );
};

type DataType = { time: number; y: number };

const UIComponent = (props: ControlProps) => {
  const { visible, uischema } = props;
  const { dayMonthShort } = useFormatDateTime();

  const id = useEncounter.utils.idFromUrl();
  const { data: encounter } = useEncounter.document.byId(id);

  const { errors, options } = useZodOptionsValidation(
    Options,
    uischema.options
  );
  const option = options?.values[0];

  const [data, setData] = useState([] as DataType[]);
  const { data: encounterFields } = useEncounter.encounterFields(
    encounter?.patient.id ?? '',
    [option?.field ?? ''],
    !!option
  );

  useEffect(() => {
    const data =
      encounterFields?.map(d => {
        return {
          time: new Date(d.encounter.startDatetime).getTime() / 1000,
          y: d.fields[0],
        };
      }) ?? [];
    setData(data);
  }, [encounterFields]);

  if (errors) {
    return <FormLabel>EncounterLineChart: {errors}</FormLabel>;
  }
  if (!visible || !option) {
    return null;
  }
  return (
    <Box
      display="flex"
      alignItems="center"
      gap={2}
      justifyContent="space-around"
      style={{ minWidth: 300 }}
      marginTop={2}
    >
      <LineChart
        width={500}
        height={250}
        data={data}
        margin={{ top: 5, right: 30, left: 20, bottom: 5 }}
      >
        <CartesianGrid strokeDasharray="3 3" />
        <XAxis
          dataKey="time"
          scale="time"
          type="number"
          tickFormatter={dayMonthShort}
          domain={['auto', 'auto']}
        />
        <YAxis>
          <Label
            value={`${option.label ?? '-'} [${option.unit ?? '?'}]`}
            angle={-90}
            position={{ x: 0, y: 10 }}
          />
        </YAxis>
        <Tooltip
          content={
            <DateTimeTooltip
              name={option.label ?? '-'}
              unit={option.unit ?? '?'}
            />
          }
        />
        <Line
          type="monotone"
          dataKey="y"
          stroke="#8884d8"
          isAnimationActive={false}
        />
      </LineChart>
    </Box>
  );
};

export const EncounterLineChart = withJsonFormsControlProps(UIComponent);
