import React from 'react';
import { Box, Typography, Table, TableBody, TableCell, TableContainer, TableHead, TableRow, Paper } from '@mui/material';
import { Column, QueryResult } from '../types';

type SqlResultProps = {
  columns: Column[];
  queryResult: QueryResult;
  show: boolean;
};

const SqlResult: React.FC<SqlResultProps> = ({ columns, queryResult, show }) => {
  return (
    <Box>
      <Typography>Result:</Typography>
      {show ? (
        <TableContainer component={Paper}>
          <Table>
            <TableHead>
              <TableRow>
                {columns.map((column) => (
                  <TableCell key={column.name}>{column.name}</TableCell>
                ))}
              </TableRow>
            </TableHead>
            <TableBody>
              {queryResult.map((row, rowIndex) => (
                <TableRow key={rowIndex}>
                  {columns.map((column) => (
                    <TableCell key={column.name}>{row[column.name]}</TableCell>
                  ))}
                </TableRow>
              ))}
            </TableBody>
          </Table>
        </TableContainer>
      ) : (
        <Typography>結果無し</Typography>
      )}
    </Box>
  );
};

export default SqlResult;
