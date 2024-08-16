import "./App.css";

import { AppBar, Box, Button, Divider, Link, Stack, TextField, Typography } from "@mui/material";
import { DataGrid } from '@mui/x-data-grid';
import { useState } from "react";
import { Column, Parameter } from "./types";
import { Service } from "./services/Service";
import { TauriService } from "./services/TauriService";
import { Parameters } from "./components/Parameters";

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
              } catch (e) {
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
      <Parameters
        parameters={parameters}
        setParameters={(newParameter: Parameter[]) => {
          setParameters(newParameter);
        }}
      />
      <Divider sx={{ marginTop: "1em" }} />
      <Typography>Select statements:</Typography>
      {
        showStatements ?
          <>
            <Stack spacing={2}>
              {
                selectStatements.map((e, i) => {
                  return <Link key={i} onClick={async () => {
                    setError("");
                    try {
                      const [columns, rows] = await service.query(e);
                      console.log(columns);
                      console.log(rows);
                      setShowResult(true);
                      setColumns(columns);
                      setQueryResult(rows);
                    } catch (e) {
                      console.log(e);
                      setError(e as string);
                    }
                  }}>{e}</Link>
                })}
            </Stack>
            <Divider />
          </>
          :
          <>結果無し</>
      }
      <Divider sx={{ marginTop: "1em" }} />
      <Typography>Result:</Typography>
      {
        showResult ?
          <>
            <DataGrid
              columns={columns.sort((a, b) => a.ordinal - b.ordinal).map((c) => { return { field: c.name } })}
              rows={queryResult}
            />
          </>
          :
          <>結果無し</>
      }
    </>
  );
}

export default App;
