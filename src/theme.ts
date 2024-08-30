import { createTheme } from '@mui/material/styles';

export const theme = (mode: 'dark' | 'light') => {
  const defaultLightTheme = createTheme();
  const defaultDarkTheme = createTheme({
    palette: {
      mode: 'dark'
    }
  });

  return createTheme({
    ...(mode === 'light' ? defaultLightTheme : defaultDarkTheme),
    palette: {
      ...(mode === 'light' ? defaultLightTheme.palette : defaultDarkTheme.palette),
      mode: mode,
    },
  });
}

