import { Paper, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, Typography } from "@mui/material";
import { Column, QueryResult } from "../types";

import ArrowCircleUpIcon from '@mui/icons-material/ArrowCircleUp';

type QueryResultProps = {
  show: boolean,
  columns: Column[],
  queryResult: QueryResult,
};

export const QueryResultView: React.FC<QueryResultProps> = ({ show, columns, queryResult }) => {


  return (
    <>
      <Typography>Result:</Typography>
      {
        show ?
          <>
            <TableContainer component={Paper}>
              <Table>
                <TableHead>
                  <TableRow>
                    {columns.map((c) => <TableCell>{c.name}</TableCell>)}
                  </TableRow>
                </TableHead>
                <TableBody>
                  {queryResult.map((e => {
                    return (<TableRow>
                      {columns.map((c) => <TableCell>{e[c.name]}</TableCell>)}
                    </TableRow>)
                  }))}
                </TableBody>
              </Table>
            </TableContainer>
          </>
          :
          <>結果無し</>
      }

      {/* トップに戻るボタン */}
      <ArrowCircleUpIcon
        color="secondary"
        sx={{
          position: "fixed",
          fontSize: "2.5em",
          right: "0.5em",
          bottom: "0.5em",
          cursor: "pointer"
        }}
        onClick={() => {
          window.scroll({
            top: 0,
          });
        }}
      />

    </>
  )
}
