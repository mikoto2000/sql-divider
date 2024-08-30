import { Divider, Link, Stack, Typography } from "@mui/material";
import { Service } from "../services/Service";
import { Column, Parameter, ParameterPattern, QueryResult } from "../types";
import { replaceParameters } from "../utils";

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
  onStatementClick,
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
                  return <Link key={i} sx={{ cursor: "pointer" }} onClick={async () => {
                    onError("");
                    try {
                      const [columns, row] = await service.query(replaceParameters(sql, parameterPattern, parameters));
                      onStatementClick(columns, row);
                      service.openNewStatementWindow(parameterPattern, parameters, [sql], columns, row)
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
