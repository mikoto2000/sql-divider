import "./App.css";
import notice from "../NOTICE.md?raw";

import { AppBar, Box, Button, CssBaseline, Dialog, DialogContent, Divider, FormControlLabel, Radio, RadioGroup, Stack, TextField, Typography } from "@mui/material";
import Tooltip from '@mui/material/Tooltip';
import { useEffect, useState } from "react";
import { Column, ConnectInfo, DbType, Parameter, ParameterPattern, QueryResult } from "./types";
import { Service } from "./services/Service";
import { TauriService } from "./services/TauriService";
import { Parameters } from "./components/Parameters";

import Accordion from '@mui/material/Accordion';
import AccordionDetails from '@mui/material/AccordionDetails';
import AccordionSummary from '@mui/material/AccordionSummary';
import ArrowDropDownIcon from '@mui/icons-material/ArrowDropDown';
import CircularProgress from '@mui/material/CircularProgress';
import InfoOutlinedIcon from '@mui/icons-material/InfoOutlined';
import { MaterialUISwitch } from "./components/MaterialUISwitch";

import { Statements } from "./components/Statements";
import { replaceParameters } from "./utils";
import { QueryResultView } from "./components/QueryResultView";
import { createStore, Store } from "@tauri-apps/plugin-store";

import { theme } from "./theme";
import { ThemeProvider } from "@emotion/react";

type ConnectStatus = "disconnect" | "connect" | "connecting";

function App() {

  let store: Store | null = null;
  const service: Service = new TauriService();

  const [currentDisplayMode, setCurrentDisplayMode] = useState<"light" | "dark">("light");

  const [showConnectInfo, setShowConnectInfo] = useState<boolean>(true);
  const [connectInfo, setConnectInfo] = useState<ConnectInfo>({ dbType: "postgres", url: "", db: "", user: "", password: "" });
  const [connectStatus, setConnectStatus] = useState<ConnectStatus>("disconnect");
  const [connectionError, setConnectionError] = useState<string>("");

  const [showStatements, setShowStatements] = useState<boolean>(false);
  const [showResult, setShowResult] = useState<boolean>(false);

  const [sql, setSql] = useState<string>("");

  const [parameters, setParameters] = useState<Parameter[]>([{ name: "", value: "" }]);

  const [parameterPattern, setParameterPattern] = useState<ParameterPattern>("jpa");


  const [columns, setColumns] = useState<Column[]>([]);
  const [queryResult, setQueryResult] = useState<QueryResult>([]);

  const [withStatements, setWithStatements] = useState<string[]>([]);
  const [selectStatements, setSelectStatements] = useState<string[]>([]);

  const [error, setError] = useState<string>("");

  const [showNoticeDialog, setShowNoticeDialog] = useState<boolean>(false);

  useEffect(() => {
    (async () => {
      store = await createStore("store.dat");
      const initial_connectInfo = await store.get<ConnectInfo>("connectInfo");
      if (initial_connectInfo) {
        setConnectInfo(initial_connectInfo);
      }

      const initial_displayMode = await store.get<"light" | "dark">("displayMode");
      if (initial_displayMode) {
        setCurrentDisplayMode(initial_displayMode);
      }
    })()
  }, []);

  return (
    <ThemeProvider theme={theme(currentDisplayMode)}>
      <CssBaseline />
      <AppBar position="static">
        <div style={{ display: "flex", flexDirection: "row" }}>
          <div style={{ flexGrow: "1" }}>SQL Divider</div>
          <MaterialUISwitch
            style={{ flexGrow: "0" }}
            checked={currentDisplayMode === 'light' ? false : true}
            onChange={(event) => {
              console.log(event);
              const mode = event.currentTarget.checked ? 'dark' : 'light';
              console.log(mode);
              setCurrentDisplayMode(mode);
              if (store) {
                store.set("displayMode", mode);
              }
            }}
          />
          <Tooltip title="ライセンス情報" style={{ flexGrow: "0" }}>
            <InfoOutlinedIcon fontSize="large" onClick={() => setShowNoticeDialog(true)} style={{ cursor: 'pointer' }} />
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
          <Typography>{connectStatus === "connect" ? "接続中: " : "接続情報: "}{`${connectInfo.dbType}://${connectInfo.user}:****@${connectInfo.url}/${connectInfo.db}`}</Typography>
        </AccordionSummary>
        <AccordionDetails>
          <Stack spacing={2}>
            <RadioGroup
              row
              value={connectInfo.dbType}
              onChange={(event) => {
                setConnectInfo({ ...connectInfo, dbType: event.currentTarget.value as DbType });
              }}
            >
              <FormControlLabel value="postgres" control={<Radio />} label="postgres" />
              <FormControlLabel value="mysql" control={<Radio />} label="mysql(alpha)" />
            </RadioGroup>
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
                        if (store) {
                          store.set("connectInfo", connectInfo);
                        }
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
                const [withStatement, selectStatements] = await service.findSelectStatement(sql);
                setWithStatements(withStatement);
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
        <Typography>Replaced SQL:</Typography>
        {replaceParameters(sql, parameterPattern, parameters)}
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
        withStatements={withStatements}
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
