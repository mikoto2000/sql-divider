import { invoke } from "@tauri-apps/api/core";

export class TauriService implements Service {
  async query(query: string): Promise<QueryResult> {
    return await invoke("query_command", { query })
  }
}
