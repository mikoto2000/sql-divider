use sqlparser::{
    ast::{Expr, JoinConstraint, JoinOperator, Query, SetExpr, Statement, TableFactor, TableWithJoins},
    dialect::PostgreSqlDialect,
    parser::{Parser, ParserError},
};

pub async fn find_select_statement(sql: &String) -> Result<Vec<String>, ParserError> {
    let dialect = PostgreSqlDialect {};

    let ast = Parser::parse_sql(&dialect, &sql)?;

    let mut select_statements: Vec<String> = vec![];
    for statement in ast.iter() {
        select_statements.extend(walk_statement(statement));
    }
    return Ok(select_statements);
}

fn walk_statement(statement: &Statement) -> Vec<String> {
    match statement {
        Statement::Query(query) => {
            return walk_query(query);
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_setexpr(setexpr: &SetExpr) -> Vec<String> {
    match setexpr {
        SetExpr::Select(select) => {
            //println!("Select: {:?}", select);
            println!("Select: {:?}", select.to_string());
            let mut select_statement = vec![select.to_string()];
            select_statement.extend(walk_table_with_joins(&select.from));

            if let Some(prewhere) = &select.prewhere {
                select_statement.extend(walk_expr(&prewhere));
            };

            if let Some(selection) = &select.selection {
                select_statement.extend(walk_expr(&selection));
            };
            return select_statement;
        }
        SetExpr::Query(query) => {
            return walk_query(query);
        }
        SetExpr::SetOperation { left, right, .. } => {
            let mut select_statement = vec![];
            select_statement.extend(walk_setexpr(left));
            select_statement.extend(walk_setexpr(right));
            return select_statement;
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_query(query: &Query) -> Vec<String> {
    let body = &query.body;
    return walk_setexpr(&body);
}

fn walk_expr(expr: &Expr) -> Vec<String> {
    //println!("{:?}", expr);
    match &expr {
        Expr::InSubquery { subquery, .. } => {
            return walk_query(subquery);
        }
        Expr::Subquery(subquery) => {
            return walk_query(subquery);
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_table_with_joins(twjs: &Vec<TableWithJoins>) -> Vec<String> {
    //println!("{:?}", twjs);
    let mut select_statements = vec![];
    for twj in twjs {
        match &twj.relation {
            TableFactor::Derived { subquery, .. } => {
                select_statements.extend(walk_query(subquery));
            }
            _ => {
                select_statements.extend(vec![]);
            }
        }

        for join in &twj.joins {
            match &join.relation {
                TableFactor::Derived { subquery, .. } => {
                    select_statements.extend(walk_query(subquery));
                }
                _ => {}
            }

            match &join.join_operator {
                JoinOperator::CrossJoin => {}
                JoinOperator::CrossApply => {}
                JoinOperator::OuterApply => {}
                JoinOperator::AsOf {
                    match_condition,
                    constraint,
                } => {
                    select_statements.extend(walk_expr(match_condition));
                    if let JoinConstraint::On(on) = constraint {
                        select_statements.extend(walk_expr(on));
                    }
                }
                JoinOperator::LeftOuter(join_constraint) => {
                    if let JoinConstraint::On(on) = join_constraint {
                        select_statements.extend(walk_expr(on));
                    }
                }
                JoinOperator::RightOuter(join_constraint) => {
                    if let JoinConstraint::On(on) = join_constraint {
                        select_statements.extend(walk_expr(on));
                    }
                }
                JoinOperator::FullOuter(join_constraint) => {
                    if let JoinConstraint::On(on) = join_constraint {
                        select_statements.extend(walk_expr(on));
                    }
                }
                JoinOperator::LeftSemi(join_constraint) => {
                    if let JoinConstraint::On(on) = join_constraint {
                        select_statements.extend(walk_expr(on));
                    }
                }
                JoinOperator::RightSemi(join_constraint) => {
                    if let JoinConstraint::On(on) = join_constraint {
                        select_statements.extend(walk_expr(on));
                    }
                }
                JoinOperator::LeftAnti(join_constraint) => {
                    if let JoinConstraint::On(on) = join_constraint {
                        select_statements.extend(walk_expr(on));
                    }
                }
                JoinOperator::RightAnti(join_constraint) => {
                    if let JoinConstraint::On(on) = join_constraint {
                        select_statements.extend(walk_expr(on));
                    }
                }

                _ => {}
            }
        }
    }
    return select_statements;
}
