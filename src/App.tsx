import "./App.css";

import { AppBar, Box, Button, Divider, Link, Stack, TextField, Typography } from "@mui/material";
import Grid from '@mui/material/Unstable_Grid2';
import { DataGrid } from '@mui/x-data-grid';

function App() {

  return (
    <>
      <AppBar position="static">SQL Divider</AppBar>
      <Box className="sql" sx={{ marginTop: "1em" }}>
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
      <Divider sx={{ marginTop: "1em" }} />
      <Typography>Parameters:</Typography>
      <Grid container className="parameter">
        <Grid xs={6}>name</Grid>
        <Grid xs={6}>value</Grid>
        <Grid xs={5}><TextField placeholder="name" sx={{ width: "100%" }} ></TextField></Grid>
        <Grid xs={5}><TextField placeholder="value" sx={{ width: "100%" }} ></TextField></Grid>
        <Grid xs={2}></Grid>
        <Grid xs={5}><TextField placeholder="name" sx={{ width: "100%" }} ></TextField></Grid>
        <Grid xs={5}><TextField placeholder="value" sx={{ width: "100%" }} ></TextField></Grid>
        <Grid xs={2}><Button variant="contained" sx={{ width: "100%", height: "100%" }} >追加</Button></Grid>
      </Grid>
      <Divider sx={{ marginTop: "1em" }} />
      <Typography>Select statements:</Typography>
      <Stack>
        <Link>{"select * from (select * from account where age >= #{age}) as a"}</Link>
        <Link>{"select * from account where age >= #{age}"}</Link>
      </Stack>
      <Divider />
      <Typography>Result:</Typography>
      <DataGrid
        columns={[{field: "id"},{field: "name"},{field: "age"}]}
        rows={[
          {id: 1, name: "mikoto2000", age: 11},
          {id: 2, name: "mikoto2001", age: 12}
        ]}

      />
    </>
  );
}

export default App;
