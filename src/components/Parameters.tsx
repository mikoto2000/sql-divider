import { Button, FormControlLabel, Grid, Radio, RadioGroup, TextField, Typography } from "@mui/material";
import { Parameter, ParameterPattern } from "../types";

type ParametersProps = {
  parameters: Parameter[],
  parameterPattern: ParameterPattern,
  onParametersChange: (newParameters: Parameter[]) => void,
  onParameterPattermChange: (newParameterPattern: ParameterPattern) => void,
};

export const Parameters: React.FC<ParametersProps> = ({
  parameters,
  parameterPattern,
  onParametersChange: setParameters,
  onParameterPattermChange: setParameterPatternChange
}) => {

  const createParameterRow = (index: number, parameter: Parameter) => {

    return (
      <>
        <Grid
          key={index + "_name"}
          xs={5}
        >
          <TextField
            placeholder={parameter.name}
            sx={{ width: "100%" }}
            onChange={(e) => {
              const newName = e.currentTarget.value;
              const targetParameter = parameters[index];
              targetParameter.name = newName;
              setParameters([...parameters]);
            }}
          >
          </TextField>
        </Grid>
        <Grid
          key={index + "_value"}
          xs={5}
        >
          <TextField
            placeholder={parameter.value}
            sx={{ width: "100%" }}
            onChange={(e) => {
              const newValue = e.currentTarget.value;
              const targetParameter = parameters[index];
              targetParameter.value = newValue;
              setParameters([...parameters]);
            }}
          >
          </TextField>
        </Grid>
        <Grid
          key={index + "_button"}
          xs={2}
        >
          <Button
            variant="contained"
            color="error"
            sx={{ width: "100%", height: "100%" }}
            onClick={() => {
              const newParameters = parameters.filter((_, i) => i !== index);
              setParameters([...newParameters]);
            }}
          >
            削除
          </Button>
        </Grid>
      </>
    );
  }


  return (
    <>
      <Typography>Parameters:</Typography>
      <RadioGroup
        row
        defaultValue={parameterPattern}
        onChange={(e) => {
          const newValue = e.currentTarget.value;
          console.log(newValue);
          if (newValue) {
            setParameterPatternChange(newValue as ParameterPattern);
          }
        }}
      >
        {/*<FormControlLabel value="mybatis" control={<Radio disabled />} label="MyBatis(#{name})" />*/}
        <FormControlLabel value="jpa" control={<Radio />} label="JPA(:name)" />
        <FormControlLabel value="dapper" control={<Radio />} label="Dapper(@name)" />
        <FormControlLabel value="log" control={<Radio />} label="log($N)" />
      </RadioGroup>
      <Grid container className="parameter-header">
        <Grid xs={5}>name</Grid>
        <Grid xs={5}>value</Grid>
        <Grid xs={2} />
      </Grid>
      <Grid container className="parameter">
        {parameters.map((e, i) => <Grid container key={i} xs={12}>{createParameterRow(i, e)}</Grid>)}
      </Grid>
      <Grid container className="parameter-control">
        <Grid xs={10} />
        <Grid xs={2}>
          <Button
            variant="contained"
            sx={{ width: "100%", height: "4em" }}
            onClick={() => setParameters([...parameters, { name: "", value: "" }])}
          >
            追加
          </Button>
        </Grid>
      </Grid>
    </>
  )
}
