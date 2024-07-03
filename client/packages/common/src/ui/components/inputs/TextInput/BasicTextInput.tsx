import React, { FC, useEffect, useRef } from 'react';
import {
  Box,
  StandardTextFieldProps,
  TextField,
  Typography,
} from '@mui/material';

export type BasicTextInputProps = StandardTextFieldProps & {
  textAlign?: 'left' | 'center' | 'right';
  focusOnRender?: boolean;
};

/**
 * Very basic TextInput component with some simple styling applied where you can
 * build your input on top.
 */

export const BasicTextInput: FC<BasicTextInputProps> = React.forwardRef(
  (
    {
      sx,
      style,
      InputProps,
      error,
      required,
      textAlign,
      focusOnRender,
      ...props
    },
    ref
  ) => {
    const inputRef = useRef<HTMLInputElement | null>(null);
    useEffect(() => {
      if (focusOnRender) {
        inputRef?.current;
        inputRef?.current?.focus();
      }
    }, []);

    return (
      <Box
        display="flex"
        justifyContent={style?.justifyContent ?? 'flex-end'}
        alignItems="center"
        flexBasis={style?.flexBasis}
        flex={style?.flex}
        width={props.fullWidth ? '100%' : undefined}
        sx={
          props.fullWidth
            ? { '& .MuiTextField-root': { width: '100%' } }
            : undefined
        }
      >
        <TextField
          ref={ref}
          inputRef={inputRef}
          color="secondary"
          sx={{
            '& .MuiInput-underline:before': { borderBottomWidth: 0 },
            '& .MuiInput-input': { color: 'gray.dark', textAlign },
            ...sx,
          }}
          variant="standard"
          size="small"
          InputProps={{
            disableUnderline: error ? true : false,
            ...InputProps,
            sx: {
              border: theme =>
                error ? `2px solid ${theme.palette.error.main}` : 'none',
              backgroundColor: theme =>
                props.disabled
                  ? theme.palette.background.toolbar
                  : theme.palette.background.menu,
              borderRadius: '8px',
              padding: '4px 8px',
              ...InputProps?.sx,
            },
          }}
          inputProps={
            props.disabled
              ? { style: { textOverflow: 'ellipsis' } }
              : { inputMode: props.inputMode }
          }
          {...props}
        />
        <Box width={2}>
          {required && (
            <Typography
              sx={{
                color: 'primary.light',
                fontSize: '17px',
                marginRight: 0.5,
              }}
            >
              *
            </Typography>
          )}
        </Box>
      </Box>
    );
  }
);
