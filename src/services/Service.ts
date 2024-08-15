type QueryResult = { [key: string]: string };

export interface Service {
  query(query: string): Promise<QueryResult>;
}

