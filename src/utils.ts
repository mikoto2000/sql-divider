import { Parameter } from "./types";

export function replaceParameters(
  query: string,
  parameterPattern: string,
  parameters: Parameter[]
) {
  let replacedQuery = query;
  if (parameterPattern === "log") {
    parameters.forEach((param: Parameter) => {
      let replaceStr = "$" + param.name;
      replacedQuery = replacedQuery.replace(replaceStr, param.value)
    });
    return replacedQuery;
  } else {
    parameters.forEach((param: Parameter) => {
      let replaceStr;
      switch (parameterPattern) {
        case "mybatis":
          replaceStr = "#{" + param.name + "}";
          break;
        case "jpa":
          replaceStr = ":" + param.name;
          break;
        case "dapper":
          replaceStr = "@" + param.name;
          break;
        default: // do nothing
      }

      if (replaceStr) {
        replacedQuery = replacedQuery.replaceAll(replaceStr, param.value)
      }
    });
    return replacedQuery;
  }
}

