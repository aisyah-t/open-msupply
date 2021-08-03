import React from 'react';
import SvgIcon, { SvgIconProps } from '@material-ui/core/SvgIcon';

export const Messages = (props: SvgIconProps): JSX.Element => {
  const combinedProps: SvgIconProps = { color: 'primary', ...props };
  return (
    <SvgIcon {...combinedProps} viewBox="0 0 20 20">
      <path d="M15 11.667V7.5c0-2.761-2.239-5-5-5s-5 2.239-5 5v4.167c0 .607-.162 1.176-.446 1.666h10.892c-.284-.49-.446-1.06-.446-1.666zM18.333 15H1.667c-1.111 0-1.111-1.667 0-1.667.92 0 1.666-.746 1.666-1.666V7.5C3.333 3.818 6.318.833 10 .833s6.667 2.985 6.667 6.667v4.167c0 .92.746 1.666 1.666 1.666 1.111 0 1.111 1.667 0 1.667zm-6.17 2.918c-.448.771-1.272 1.246-2.163 1.246-.891 0-1.715-.475-2.162-1.246-.323-.555.078-1.251.72-1.251h2.884c.642 0 1.043.696.72 1.251z" />
    </SvgIcon>
  );
};
