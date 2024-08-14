import "./App.css";

import { AppBar, Box, Button, Divider, Link, Stack, TextField, Typography } from "@mui/material";
import Grid from '@mui/material/Unstable_Grid2';
import { DataGrid } from '@mui/x-data-grid';
import { useState } from "react";
import { Parameter } from "./types";

function App() {

  const [sql, setSql] = useState<string>("");
  const [parameters, setParameters] = useState<Parameter[]>([{ name: "", value: "" }]);

  const createParameterRow = (index: number, parameter: Parameter) => {

    return (
      <>
        <Grid
          key={index + "_name"}
          xs={5}
        >
          <TextField
            placeholder={parameter.name}
            sx={{ width: "100%" }}
            onChange={(e) => {
              const newName = e.currentTarget.value;
              const targetParameter = parameters[index];
              targetParameter.name = newName;
              setParameters([...parameters]);
            }}
          >
          </TextField>
        </Grid>
        <Grid
          key={index + "_value"}
          xs={5}
        >
          <TextField
            placeholder={parameter.value}
            sx={{ width: "100%" }}
          >
          </TextField>
        </Grid>
        <Grid
          key={index + "_button"}
          xs={2}
        >
          <Button
            variant="contained"
            color="error"
            sx={{ width: "100%", height: "100%" }}
            onClick={() => {
              const newParameters = parameters.filter((_, i) => i !== index);
              setParameters([...newParameters]);
            }}
          >
            削除
          </Button>
        </Grid>
      </>
    );
  }


  return (
    <>
      <AppBar position="static">SQL Divider</AppBar>
      <Box className="sql" sx={{ marginTop: "1em" }}>
        <TextField
          fullWidth
          label="SQL"
          placeholder="select * from user;"
          multiline
          value={sql}
          onChange={(e) => {
            setSql(e.target.value);
          }}
        >
        </TextField>
        <Box className="controls">
          <Button variant="outlined" >SQL 発行</Button>
          <Button variant="outlined" >SELECT 文抽出</Button>
        </Box>
      </Box>
      <Divider sx={{ marginTop: "1em" }} />
      <Typography>Parameters:</Typography>
      <Grid container className="parameter-header">
        <Grid xs={5}>name</Grid>
        <Grid xs={5}>value</Grid>
        <Grid xs={2}/>
      </Grid>
      <Grid container className="parameter">
        {parameters.map((e, i) => <Grid container key={i} xs={12}>{createParameterRow(i, e)}</Grid>)}
      </Grid>
      <Grid container className="parameter-control">
        <Grid xs={10} />
        <Grid xs={2}>
          <Button
            variant="contained"
            sx={{ width: "100%", height: "4em" }}
            onClick={() => setParameters([...parameters, { name: "", value: "" }])}
          >
            追加
          </Button>
        </Grid>
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
        columns={[{ field: "id" }, { field: "name" }, { field: "age" }]}
        rows={[
          { id: 1, name: "mikoto2000", age: 11 },
          { id: 2, name: "mikoto2001", age: 12 }
        ]}

      />
    </>
  );
}

export default App;
