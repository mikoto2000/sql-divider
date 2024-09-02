import { useEffect, useState } from "react";
import { Statements } from "../components/Statements";
import { Service } from "../services/Service";
import { TauriService } from "../services/TauriService";
import { emit } from "@tauri-apps/api/event";
import { Column, Parameter, ParameterPattern, QueryResult } from "../types";
import { QueryResultView } from "../components/QueryResultView";
import { CssBaseline, Divider, ThemeProvider } from "@mui/material";

import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';
import { Store } from "@tauri-apps/plugin-store";

import { theme } from "../theme";

type StatementPageProps = {
};

export const StatementPage: React.FC<StatementPageProps> = ({ }) => {

  const store = new Store("store.dat");
  const service: Service = new TauriService();
  let initialized = false;

  const [currentDisplayMode, setCurrentDisplayMode] = useState<"light" | "dark">("light");

  const [parameterPattern, setParameterPattern] = useState<ParameterPattern>("jpa");
  const [parameters, setParameters] = useState<Parameter[]>([]);
  const [selectStatements, setSelectStatements] = useState<string[]>([]);
  const [columns, setColumns] = useState<Column[]>([]);
  const [queryResult, setQueryResult] = useState<QueryResult>([]);

  useEffect(() => {
    if (!initialized) {
      getCurrentWebviewWindow().listen("data", (event) => {
        const [parameterPattern, parameters, selectStatements, columns, queryResult] = event.payload as any;
        setParameterPattern(parameterPattern);
        setParameters(parameters);
        setSelectStatements(selectStatements);
        setColumns(columns);
        setQueryResult(queryResult);
      });
      emit("done", {});
      initialized = true;
    }

    (async () => {
      const initial_displayMode = await store.get<"light" | "dark">("displayMode");
      if (initial_displayMode) {
        setCurrentDisplayMode(initial_displayMode);
      }
    })()

  }, []);

  return (
    <ThemeProvider theme={theme(currentDisplayMode)}>
      <CssBaseline />
      <Statements
        service={service}
        show={true}
        parameterPattern={parameterPattern}
        parameters={parameters}
        withStatements={[]}
        selectStatements={selectStatements}
        onStatementClick={() => { }}
        onError={() => { }}
      />
      <Divider sx={{ marginTop: "1em" }} />
      <QueryResultView
        show={true}
        columns={columns}
        queryResult={queryResult}
      />
    </ThemeProvider>
  )
}
