import { Column } from "../types";

type QueryResult = { [key: string]: string }[];

export interface Service {
  query(query: string): Promise<[Column[], QueryResult]>;
  find_select_statement(query: string): Promise<string[]>;
}

