import { invoke } from "@tauri-apps/api/core";

export class TauriService implements Service {
  async query(query: string): Promise<QueryResult> {
    return await invoke("query_command", { query })
  }
  async find_select_statement(query: string): Promise<string[]> {
    return await invoke("find_select_statement_command", { query })
  }
}
