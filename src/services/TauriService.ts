import { invoke } from "@tauri-apps/api/core";
import { Column, ConnectInfo, QueryResult } from "../types";
import { Service } from "./Service";

export class TauriService implements Service {
  async connect(connectInfo: ConnectInfo): Promise<void> {
    return await invoke("connect_command", connectInfo)
  }
  async close(): Promise<void> {
    return await invoke("close_command", {})
  }
  async query(query: string): Promise<[Column[], QueryResult]> {
    return await invoke("query_command", { query })
  }
  async find_select_statement(query: string): Promise<string[]> {
    return await invoke("find_select_statement_command", { query })
  }
}
