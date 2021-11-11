import React, { FC } from 'react';
import { CircularProgress, Typography } from '@mui/material';
import Box from '@mui/material/Box';
import { useTranslation } from '../../../../intl/intlHelpers';

interface InlineSpinnerProps {
  color?: 'primary' | 'secondary' | 'error' | 'info' | 'success' | 'warning';
  showText?: boolean;
}

export const InlineSpinner: FC<InlineSpinnerProps> = ({
  color = 'primary',
  showText = false,
}) => {
  const t = useTranslation();

  return (
    <Box
      style={{
        display: 'flex',
        alignItems: 'center',
        justifyContent: 'center',
        flexDirection: 'row',
      }}
    >
      <CircularProgress size={20} color={color} />
      <Typography sx={{ margin: theme => theme.spacing(1), fontSize: '12px' }}>
        {showText && t('app.loading')}
      </Typography>
    </Box>
  );
};