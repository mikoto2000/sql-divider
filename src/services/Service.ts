import { Column, ConnectInfo, QueryResult } from "../types";

export interface Service {
  connect(connectInfo: ConnectInfo): Promise<void>;
  close(): Promise<void>;
  query(query: string): Promise<[Column[], QueryResult]>;
  find_select_statement(query: string): Promise<string[]>;
}

