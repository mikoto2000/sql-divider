import "./App.css";

import { AppBar, Box, Button, Divider, Link, Stack, TextField, Typography } from "@mui/material";
import Grid from '@mui/material/Unstable_Grid2';
import { DataGrid } from '@mui/x-data-grid';
import { useState } from "react";
import { Column, Parameter } from "./types";
import { Service } from "./services/Service";
import { TauriService } from "./services/TauriService";

function App() {

  const service: Service = new TauriService();

  const [showStatements, setShowStatements] = useState<boolean>(false);
  const [showResult, setShowResult] = useState<boolean>(false);

  const [sql, setSql] = useState<string>("");
  const [parameters, setParameters] = useState<Parameter[]>([{ name: "", value: "" }]);
  const [columns, setColumns] = useState<Column[]>([]);
  const [queryResult, setQueryResult] = useState<{ [key: string]: string }[]>([]);

  const [selectStatements, setSelectStatements] = useState<string[]>([]);

  const [error, setError] = useState<string>("");

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
              setError("");
              try {
                const [columns, rows] = await service.query(sql);
                console.log(columns);
                console.log(rows);
                setShowResult(true);
                setColumns(columns);
                setQueryResult(rows);
              } catch (e) {
                console.log(e);
                setError(e as string);
              }
            }}
          >
            SQL 発行
          </Button>
          <Button
            variant="outlined"
            onClick={async () => {
              setError("");
              try {
                const selectStatements = await service.find_select_statement(sql);
                console.log(selectStatements);
                setSelectStatements(selectStatements);
              } catch(e) {
                console.log(e);
                setError(e as string);
              }
              setShowStatements(true)
            }}
          >
            SELECT 文抽出
          </Button>
        </Box>
      </Box >
      <p>{error}</p>
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
              {
                selectStatements.map((e, i) => {
                  return <Link key={i} onClick={() => { }}>{e}</Link>
                })}
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
              columns={columns.sort((a, b) => a.ordinal - b.ordinal).map((c) => { return { field: c.name } })}
              rows={queryResult}
            />
          </>
          :
          <></>
      }
    </>
  );
}

export default App;
