import "./App.css";

import { AppBar, Box, Button, Divider, Link, Stack, TextField, Typography } from "@mui/material";
import { DataGrid } from '@mui/x-data-grid';
import { useState } from "react";
import { Column, Parameter, ParameterPattern } from "./types";
import { Service } from "./services/Service";
import { TauriService } from "./services/TauriService";
import { Parameters } from "./components/Parameters";

import Accordion from '@mui/material/Accordion';
import AccordionDetails from '@mui/material/AccordionDetails';
import AccordionSummary from '@mui/material/AccordionSummary';
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';

function App() {

  const service: Service = new TauriService();

  const [showStatements, setShowStatements] = useState<boolean>(false);
  const [showResult, setShowResult] = useState<boolean>(false);

  const [sql, setSql] = useState<string>("");

  const [parameters, setParameters] = useState<Parameter[]>([{ name: "", value: "" }]);

  const [parameterPattern, setParameterPattern] = useState<ParameterPattern>("jpa");


  const [columns, setColumns] = useState<Column[]>([]);
  const [queryResult, setQueryResult] = useState<{ [key: string]: string }[]>([]);

  const [selectStatements, setSelectStatements] = useState<string[]>([]);

  const [error, setError] = useState<string>("");


  return (
    <>
      <AppBar position="static">SQL Divider</AppBar>
      <Accordion>
        <AccordionSummary
          expandIcon={<ArrowDropDownIcon />}
        >
          <Typography>接続情報</Typography>
        </AccordionSummary>
        <AccordionDetails>
          <Stack spacing={2}>
            <TextField label="サーバーアドレス" placeholder="localhost" fullWidth />
            <TextField label="データベース名" placeholder="postgres" fullWidth />
            <TextField label="ユーザー名" placeholder="postgres" fullWidth />
            <TextField label="パスワード" placeholder="postgres" fullWidth />
            <Button variant="contained">接続</Button>
          </Stack>
        </AccordionDetails>
      </Accordion>
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
                const [columns, rows] = await service.query(replaceParameters(sql, parameters));
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
        parameterPattern={parameterPattern}
        onParametersChange={(newParameter: Parameter[]) => {
          setParameters(newParameter);
        }}
        onParameterPattermChange={(newParameterPattern) => {
          setParameterPattern(newParameterPattern);
        }}
      />
      <Divider sx={{ marginTop: "1em" }} />
      <Typography>Select statements:</Typography>
      {
        showStatements ?
          <>
            <Stack spacing={2}>
              {
                selectStatements.map((sql, i) => {
                  return <Link key={i} onClick={async () => {
                    setError("");
                    try {
                      const [columns, rows] = await service.query(replaceParameters(sql, parameters));
                      console.log(columns);
                      console.log(rows);
                      setShowResult(true);
                      setColumns(columns);
                      setQueryResult(rows);
                    } catch (e) {
                      console.log(e);
                      setError(e as string);
                    }
                  }}>{sql}</Link>
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

  function replaceParameters(query: string, parameters: Parameter[]) {
    let replacedQuery = query;
    parameters.forEach((param: Parameter) => {
      let replaceStr;
      switch (parameterPattern) {
        case "mybatis":
          replaceStr = "#{" + param.name + "}";
          break;
        case "jpa":
          replaceStr = ":" + param.name;
          break;
        case "dapper":
          replaceStr = "@" + param.name;
          break;
        default: // do nothing
      }
      console.log(replaceStr);
      console.log(param.value);

      if (replaceStr) {
        replacedQuery = replacedQuery.replaceAll(replaceStr, param.value)
      }
    });
    console.log(replacedQuery);
    return replacedQuery;
  }
}

export default App;
