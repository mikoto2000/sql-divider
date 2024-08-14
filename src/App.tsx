import { useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

import { AppBar, Box, Button, Label, TextField, Typography } from "@mui/material";
import Grid from '@mui/material/Unstable_Grid2';

function App() {

  return (
    <>
    <AppBar position="static">SQL Divider</AppBar>
    <Typography>SQL</Typography>
    <Box className="sql">
    <TextField
      fullWidth
      label="SQL"
      placeholder="select * from user;"
      multiline
    >
    </TextField>
    <Box className="controls">
    <Button variant="outlined" >SQL 発行</Button>
    <Button variant="outlined" >SELECT 文抽出</Button>
    </Box>
    </Box>
    <Typography>Parameter</Typography>
    <Grid container className="parameter">
    <Grid xs={6}>name</Grid>
    <Grid xs={6}>value</Grid>
    <Grid xs={5}><TextField xs={2}></TextField></Grid>
    <Grid xs={5}><TextField xs={2}></TextField></Grid>
    <Grid xs={2}></Grid>
    <Grid xs={5}><TextField xs={2}></TextField></Grid>
    <Grid xs={5}><TextField xs={2}></TextField></Grid>
    <Grid xs={2}><Button variant="contained">追加</Button></Grid>
    </Grid>
    </>
  );
}

export default App;
