import "./App.css";
import notice from "../NOTICE.md?raw";

import { AppBar, Box, Button, Dialog, DialogContent, Divider, Paper, Stack, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, TextField, Typography } from "@mui/material";
import Tooltip from '@mui/material/Tooltip';
import { useState } from "react";
import { Column, ConnectInfo, Parameter, ParameterPattern } from "./types";
import { Service } from "./services/Service";
import { TauriService } from "./services/TauriService";
import { Parameters } from "./components/Parameters";

import Accordion from '@mui/material/Accordion';
import AccordionDetails from '@mui/material/AccordionDetails';
import AccordionSummary from '@mui/material/AccordionSummary';
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
import InfoOutlinedIcon from '@mui/icons-material/InfoOutlined';
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

  const [showNoticeDialog, setShowNoticeDialog] = useState<boolean>(false);

  return (
    <>
      <AppBar position="static">
        <div style={{ display: "flex", flexDirection: "row" }}>
          <div style={{ flexGrow: "1" }}>SQL Divider</div>
          <Tooltip title="ライセンス情報">
            <div
              style={{ flexGrow: "0" }}
              onClick={() => setShowNoticeDialog(true)}
            >
              <InfoOutlinedIcon style={{ cursor: 'pointer' }} /></div>
          </Tooltip>
        </div>
      </AppBar>
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
                  onClick={async () => {
                    setError("");
                    try {
                      await service.connect(connectInfo);
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
                setColumns(columns.sort((a, b) => a.ordinal - b.ordinal));
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
          setColumns(columns.sort((a, b) => a.ordinal - b.ordinal));
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
            <TableContainer component={Paper}>
              <Table>
                <TableHead>
                  <TableRow>
                    {columns.map((c) => <TableCell>{c.name}</TableCell>)}
                  </TableRow>
                </TableHead>
                <TableBody>
                  {queryResult.map((e => {
                    return (<TableRow>
                      {columns.map((c) => <TableCell>{e[c.name]}</TableCell>)}
                    </TableRow>)
                  }))}
                </TableBody>
              </Table>
            </TableContainer>
          </>
          :
          <>結果無し</>
      }
      <Dialog
        open={showNoticeDialog}
        onClose={() => { setShowNoticeDialog(false) }} >
        <DialogContent>
          <pre style={{ fontSize: "0.75em" }}>
            {(notice as any)}
          </pre>
        </DialogContent>
      </Dialog>
    </>
  );

}

export default App;
