import React from 'react';
import SvgIcon, { SvgIconProps } from '@mui/material/SvgIcon';

export const SnowflakeIcon = (
  props: SvgIconProps & { stroke?: string }
): JSX.Element => {
  const { stroke = 'currentColor', ...rest } = props;

  return (
    <SvgIcon
      {...rest}
      viewBox="0 0 20 20"
      stroke={stroke}
      strokeWidth="1"
      strokeLinecap="round"
      strokeLinejoin="round"
    >
      <path d="M1 4.62699L3.53898 6.07783M3.53898 6.07783L7.5288 8.2541M3.53898 6.07783L1 6.80326M3.53898 6.07783L3.17627 3.53886M14.0576 11.8812L11.5186 10.4304M11.5186 10.4304L7.5288 8.2541M11.5186 10.4304L11.8813 12.9693M11.5186 10.4304L14.0576 9.70495M7.5288 8.2541L3.53898 10.4304M7.5288 8.2541V3.17614M7.5288 8.2541V12.9693M7.5288 8.2541L11.5186 6.07783M3.53898 10.4304L1 11.8812M3.53898 10.4304L1 9.70495M3.53898 10.4304L3.17627 12.9693M7.5288 3.17614V0.999878M7.5288 3.17614L5.71524 2.45072M7.5288 3.17614L9.34236 2.45072M7.5288 12.9693V15.5083M7.5288 12.9693L9.34236 14.0575M7.5288 12.9693L5.71524 14.0575M11.5186 6.07783L14.0576 4.62699M11.5186 6.07783L14.0576 6.80326M11.5186 6.07783L11.8813 3.53886" />
    </SvgIcon>
  );
};
