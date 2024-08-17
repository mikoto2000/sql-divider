import "./App.css";

import { AppBar, Box, Button, Divider, Stack, TextField, Typography } from "@mui/material";
import { DataGrid } from '@mui/x-data-grid';
import { useState } from "react";
import { Column, ConnectInfo, Parameter, ParameterPattern } from "./types";
import { Service } from "./services/Service";
import { TauriService } from "./services/TauriService";
import { Parameters } from "./components/Parameters";

import Accordion from '@mui/material/Accordion';
import AccordionDetails from '@mui/material/AccordionDetails';
import AccordionSummary from '@mui/material/AccordionSummary';
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
import { Statements } from "./components/Statements";
import { replaceParameters } from "./utils";

function App() {

  const service: Service = new TauriService();

  const [connectInfo, setConnectInfo] = useState<ConnectInfo>({ url: "localhost:5432", db: "postgres", user: "postgres", password: "postgres" });
  const [connecting, setConnecting] = useState<boolean>(false);

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
          <Typography>接続情報: {`postgres://${connectInfo.user}:****@${connectInfo.url}/${connectInfo.db}`}</Typography>
        </AccordionSummary>
        <AccordionDetails>
          <Stack spacing={2}>
            <TextField
              label="サーバーアドレス"
              placeholder="localhost:5432"
              fullWidth
              disabled={connecting}
              InputProps={{
                readOnly: connecting,
              }}
              value={connectInfo.url}
              onChange={(e) => {
                const newValue = e.currentTarget.value
                if (newValue !== undefined) {
                  setConnectInfo({ ...connectInfo, url: newValue });
                }
              }}
            />
            <TextField
              label="データベース名"
              placeholder="postgres"
              fullWidth
              disabled={connecting}
              InputProps={{
                readOnly: connecting,
              }}
              value={connectInfo.db}
              onChange={(e) => {
                const newValue = e.currentTarget.value
                if (newValue !== undefined) {
                  setConnectInfo({ ...connectInfo, db: newValue });
                }
              }}
            />
            <TextField
              label="ユーザー名"
              placeholder="postgres"
              fullWidth
              disabled={connecting}
              InputProps={{
                readOnly: connecting,
              }}
              value={connectInfo.user}
              onChange={(e) => {
                const newValue = e.currentTarget.value
                if (newValue !== undefined) {
                  setConnectInfo({ ...connectInfo, user: newValue });
                }
              }}
            />
            <TextField
              label="パスワード"
              placeholder="postgres"
              type="password"
              fullWidth
              disabled={connecting}
              InputProps={{
                readOnly: connecting,
              }}
              value={connectInfo.password}
              onChange={(e) => {
                const newValue = e.currentTarget.value
                if (newValue !== undefined) {
                  setConnectInfo({ ...connectInfo, password: newValue });
                }
              }}
            />
            {
              connecting
                ?
                <Button
                  variant="contained"
                  color="error"
                  onClick={() => {
                    service.close();
                    setConnecting(false);
                  }}
                >
                  切断
                </Button>
                :
                <Button
                  variant="contained"
                  onClick={() => {
                    setError("");
                    try {
                      service.connect(connectInfo);
                      setConnecting(true);
                    } catch (e) {
                      setError(e as string);
                    }
                  }}
                >
                  接続
                </Button>
            }
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
            disabled={!connecting}
            variant="outlined"
            onClick={async () => {
              setError("");
              try {
                const [columns, rows] = await service.query(
                  replaceParameters(sql, parameterPattern, parameters));
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
            disabled={!connecting}
            variant="outlined"
            onClick={async () => {
              setError("");
              try {
                const selectStatements = await service.find_select_statement(sql);
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
      <Statements
        service={service}
        show={showStatements}
        parameterPattern={parameterPattern}
        parameters={parameters}
        selectStatements={selectStatements}
        onStatementClick={(columns, rows) => {
          setError("");
          setColumns(columns);
          setQueryResult(rows);
        }}
        onError={(e) => {
          setError(e as string);
        }}

      />
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
