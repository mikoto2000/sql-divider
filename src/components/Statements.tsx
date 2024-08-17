import { Divider, Link, Stack, Typography } from "@mui/material";
import { Service } from "../services/Service";
import { Column, Parameter, ParameterPattern, QueryResult } from "../types";

type StatementsProps = {
  service: Service,
  show: boolean,
  parameterPattern: ParameterPattern,
  parameters: Parameter[],
  selectStatements: string[],
  onStatementClick: (columns: Column[], statement: QueryResult) => void,
  onError: (e: unknown) => void,
};

export const Statements: React.FC<StatementsProps> = ({
  service,
  show,
  parameterPattern,
  parameters,
  selectStatements,
  onError,
}) => {


  return (
    <>
      <Typography>Select statements:</Typography>
      {
        show ?
          <>
            <Stack spacing={2}>
              {
                selectStatements.map((sql, i) => {
                  return <Link key={i} onClick={async () => {
                    onError("");
                    try {
                      return await service.query(replaceParameters(sql, parameters));
                    } catch (e) {
                      console.log(e);
                      onError(e as string);
                    }
                  }}>{sql}</Link>
                })}
            </Stack>
            <Divider />
          </>
          :
          <>結果無し</>
      }
    </>
  )
}
