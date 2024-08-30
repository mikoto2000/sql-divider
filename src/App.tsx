import "./App.css";
import notice from "../NOTICE.md?raw";

import { AppBar, Box, Button, CssBaseline, Dialog, DialogContent, Divider, Stack, TextField, Typography } from "@mui/material";
import Tooltip from '@mui/material/Tooltip';
import { useEffect, useState } from "react";
import { Column, ConnectInfo, Parameter, ParameterPattern, QueryResult } from "./types";
import { Service } from "./services/Service";
import { TauriService } from "./services/TauriService";
import { Parameters } from "./components/Parameters";

import Accordion from '@mui/material/Accordion';
import AccordionDetails from '@mui/material/AccordionDetails';
import AccordionSummary from '@mui/material/AccordionSummary';
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
import CircularProgress from '@mui/material/CircularProgress';
import InfoOutlinedIcon from '@mui/icons-material/InfoOutlined';
import { Statements } from "./components/Statements";
import { replaceParameters } from "./utils";
import { QueryResultView } from "./components/QueryResultView";
import { Store } from "@tauri-apps/plugin-store";

import { theme } from "./theme";
import { ThemeProvider } from "@emotion/react";

type ConnectStatus = "disconnect" | "connect" | "connecting";

function App() {

  const store = new Store("store.dat");
  const service: Service = new TauriService();

  const [showConnectInfo, setShowConnectInfo] = useState<boolean>(true);
  const [connectInfo, setConnectInfo] = useState<ConnectInfo>({ url: "", db: "", user: "", password: "" });
  const [connectStatus, setConnectStatus] = useState<ConnectStatus>("disconnect");
  const [connectionError, setConnectionError] = useState<string>("");

  const [showStatements, setShowStatements] = useState<boolean>(false);
  const [showResult, setShowResult] = useState<boolean>(false);

  const [sql, setSql] = useState<string>("");

  const [parameters, setParameters] = useState<Parameter[]>([{ name: "", value: "" }]);

  const [parameterPattern, setParameterPattern] = useState<ParameterPattern>("jpa");


  const [columns, setColumns] = useState<Column[]>([]);
  const [queryResult, setQueryResult] = useState<QueryResult>([]);

  const [selectStatements, setSelectStatements] = useState<string[]>([]);

  const [error, setError] = useState<string>("");

  const [showNoticeDialog, setShowNoticeDialog] = useState<boolean>(false);

  useEffect(() => {
    (async () => {
      const initial_connectInfo = await store.get<ConnectInfo>("connectInfo");
      if (initial_connectInfo) {
        setConnectInfo(initial_connectInfo);
      }
    })()
  }, []);

  return (
    <ThemeProvider theme={theme("light")}>
      <CssBaseline />
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
      <Accordion
        defaultExpanded={true}
        expanded={showConnectInfo}
        onChange={(_event, expanded) => {
          setShowConnectInfo(expanded);
        }}
      >
        <AccordionSummary
          expandIcon={<ArrowDropDownIcon />}
        >
          <Typography>{connectStatus === "connect" ? "接続中: " : "接続情報: "}{`postgres://${connectInfo.user}:****@${connectInfo.url}/${connectInfo.db}`}</Typography>
        </AccordionSummary>
        <AccordionDetails>
          <Stack spacing={2}>
            <TextField
              label="サーバーアドレス"
              placeholder="localhost:5432"
              fullWidth
              disabled={connectStatus === "connect"}
              InputProps={{
                readOnly: connectStatus === "connect",
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
              placeholder="public"
              fullWidth
              disabled={connectStatus === "connect"}
              InputProps={{
                readOnly: connectStatus === "connect",
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
              disabled={connectStatus === "connect"}
              InputProps={{
                readOnly: connectStatus === "connect",
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
              disabled={connectStatus === "connect"}
              InputProps={{
                readOnly: connectStatus === "connect",
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
              connectStatus === "connect"
                ?
                <Button
                  variant="contained"
                  color="error"
                  onClick={() => {
                    service.close();
                    setConnectStatus("disconnect");
                  }}
                >
                  切断
                </Button>
                :
                connectStatus === "disconnect"
                  ?
                  <Button
                    variant="contained"
                    onClick={async () => {
                      setConnectionError("");
                      setConnectStatus("connecting");
                      try {
                        await service.connect(connectInfo);
                        setConnectStatus("connect");
                        setShowConnectInfo(false);
                        store.set("connectInfo", connectInfo);
                      } catch (e) {
                        setConnectionError(e as string);
                        setConnectStatus("disconnect");
                      }
                    }}
                  >
                    接続
                  </Button>
                  :
                  <Button
                    variant="contained"
                    disabled
                  >
                    <CircularProgress size={20} />
                    接続中
                  </Button>
            }
          </Stack>
          <Box>{connectionError ? `Error: ${connectionError}` : <></>}</Box>
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
            disabled={!connectStatus}
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
                const selectStatements = await service.findSelectStatement(sql);
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
        onStatementClick={(_columns, _rows) => {
          setError("");
          //setColumns(columns.sort((a, b) => a.ordinal - b.ordinal));
          //setQueryResult(rows);
        }}
        onError={(e) => {
          setError(e as string);
        }}

      />
      <Divider sx={{ marginTop: "1em" }} />
      <QueryResultView
        show={showResult}
        columns={columns}
        queryResult={queryResult}
      />
      <Dialog
        open={showNoticeDialog}
        onClose={() => { setShowNoticeDialog(false) }} >
        <DialogContent>
          <pre style={{ fontSize: "0.75em" }}>
            {(notice as any)}
          </pre>
        </DialogContent>
      </Dialog>
    </ThemeProvider>
  );

}

export default App;
