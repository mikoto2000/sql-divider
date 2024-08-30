import { invoke } from "@tauri-apps/api/core";
import { Column, ConnectInfo, Parameter, ParameterPattern, QueryResult } from "../types";
import { Service } from "./Service";

export class TauriService implements Service {
  async connect(connectInfo: ConnectInfo): Promise<void> {
    await invoke("connect_command", connectInfo);
  }
  async close(): Promise<void> {
    return await invoke("close_command", {})
  }
  async query(query: string): Promise<[Column[], QueryResult]> {
    return await invoke("query_command", { query })
  }
  async findSelectStatement(query: string): Promise<[string[], string[]]> {
    return await invoke("find_select_statement_command", { query })
  }
  async openNewStatementWindow(parameterPattern: ParameterPattern, parameters: Parameter[], selectStatements: string[], columns: Column[], queryResult: QueryResult): Promise<void> {
    return await invoke("open_new_statement_window_command", { parameterPattern, parameters, selectStatements, columns, queryResult })
  }
}
