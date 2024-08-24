import { Column, ConnectInfo, Parameter, ParameterPattern, QueryResult } from "../types";

export interface Service {
  connect(connectInfo: ConnectInfo): Promise<void>;
  close(): Promise<void>;
  query(query: string): Promise<[Column[], QueryResult]>;
  findSelectStatement(query: string): Promise<string[]>;
  openNewStatementWindow(parameterPattern: ParameterPattern, parameters: Parameter[], selectStatements: string[], columns: Column[], queryResult: QueryResult): Promise<void>;
}

