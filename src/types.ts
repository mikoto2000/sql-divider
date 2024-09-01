export type DbType = "postgres" | "mysql";

export type ConnectInfo = {
  dbType: DbType,
  url: string,
  db: string,
  user: string,
  password: string,
};

export type Parameter = {
  name: string,
  value: string,
};

export type ParameterPattern = "mybatis" | "jpa" | "dapper";

export type Column = {
  ordinal: number,
  name: string,
};

export type QueryResult = { [key: string]: string }[];

