import React, { useState } from 'react';
import { Box, Button, TextField, Typography } from '@mui/material';

type SqlInputProps = {
  onSqlChange: (sql: string) => void;
  onExecuteSql: () => void;
  onExtractSelectStatements: () => void;
};

const SqlInput: React.FC<SqlInputProps> = ({ onSqlChange, onExecuteSql, onExtractSelectStatements }) => {
  const [sql, setSql] = useState<string>('');

  const handleSqlChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const newSql = e.target.value;
    setSql(newSql);
    onSqlChange(newSql);
  };

  return (
    <Box className="sql" sx={{ marginTop: '1em' }}>
      <TextField
        fullWidth
        label="SQL"
        placeholder="select * from user;"
        multiline
        value={sql}
        onChange={handleSqlChange}
      />
      <Box className="controls">
        <Button variant="outlined" onClick={onExecuteSql}>
          SQL 発行
        </Button>
        <Button variant="outlined" onClick={onExtractSelectStatements}>
          SELECT 文抽出
        </Button>
      </Box>
      <Typography>Replaced SQL:</Typography>
      {sql}
    </Box>
  );
};

export default SqlInput;
