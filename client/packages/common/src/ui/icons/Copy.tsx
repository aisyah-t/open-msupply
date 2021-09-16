import React from 'react';
import SvgIcon, { SvgIconProps } from '@material-ui/core/SvgIcon';

export const Copy = (props: SvgIconProps): JSX.Element => {
  const combinedProps: SvgIconProps = {
    color: 'primary',
    style: {
      fill: 'none',
    },
    stroke: 'currentColor',
    ...props,
  };

  return (
    <SvgIcon {...combinedProps} viewBox="0 0 24 24" strokeWidth="2">
      <rect x="9" y="9" width="13" height="13" rx="2" ry="2"></rect>
      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1"></path>
    </SvgIcon>
  );
};
