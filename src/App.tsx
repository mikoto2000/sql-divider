import "./App.css";

import { AppBar, Box, Button, Divider, Link, Stack, TextField, Typography } from "@mui/material";
import Grid from '@mui/material/Unstable_Grid2';
import { DataGrid } from '@mui/x-data-grid';
import { useState } from "react";
import { Parameter } from "./types";
import { invoke } from "@tauri-apps/api/core";

function App() {

  const [showStatements, setShowStatements] = useState<boolean>(false);
  const [showResult, setShowResult] = useState<boolean>(false);
  const [selectedStatement, setSelectedStatement] = useState<"zero" | "one" | "two" | null>(null);

  const rows_z_t = [
    { id: 1, name: "mikoto2000", age: 11 },
    { id: 2, name: "mikoto2001", age: 12 }
  ];

  const rows_o = [
    { id: 1, name: "mikoto", age: 10 },
    { id: 2, name: "mikoto2000", age: 11 },
    { id: 3, name: "mikoto2001", age: 12 }
  ];


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
          <Button
            variant="outlined"
            onClick={async () => {
              const result = await invoke("query_command", { query: sql });
              console.log(result);
              setShowResult(true);
              setSelectedStatement("zero");
            }}
          >
            SQL 発行
          </Button>
          <Button
            variant="outlined"
            onClick={async () => {
              setShowStatements(true)
            }
            }
          >
            SELECT 文抽出
          </Button>
        </Box>
      </Box >
      <Divider sx={{ marginTop: "1em" }} />
      <Typography>Parameters:</Typography>
      <Grid container className="parameter-header">
        <Grid xs={5}>name</Grid>
        <Grid xs={5}>value</Grid>
        <Grid xs={2} />
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
      {
        showStatements ?
          <>
            <Typography>Select statements:</Typography>
            <Stack>
              <Link onClick={() => setSelectedStatement("two")}>{"select * from (select * from account) as a  where age >= #{age};"}</Link>
              <Link onClick={() => setSelectedStatement("one")}>{"select * from account"}</Link>
            </Stack>
            <Divider />
          </>
          :
          <></>
      }
      {
        showResult ?
          <>
            <Typography>Result:</Typography>
            <DataGrid
              columns={[{ field: "id" }, { field: "name" }, { field: "age" }]}
              rows={selectedStatement ? (selectedStatement === "one" ? rows_o : rows_z_t) : []}
            />
          </>
          :
          <></>
      }
    </>
  );
}

export default App;
