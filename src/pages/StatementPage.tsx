import { useEffect, useState } from "react";
import { Statements } from "../components/Statements";
import { Service } from "../services/Service";
import { TauriService } from "../services/TauriService";
import { emit } from "@tauri-apps/api/event";
import { Column, Parameter, ParameterPattern, QueryResult } from "../types";
import { QueryResultView } from "../components/QueryResultView";
import { Divider } from "@mui/material";

import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

type StatementPageProps = {
};

export const StatementPage: React.FC<StatementPageProps> = ({ }) => {

  const service: Service = new TauriService();
  let initialized = false;

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
  }, []);

  return (
    <>
      <Statements
        service={service}
        show={true}
        parameterPattern={parameterPattern}
        parameters={parameters}
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
    </>
  )
}
