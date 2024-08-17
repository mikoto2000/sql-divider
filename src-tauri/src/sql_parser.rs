use sqlparser::{
    ast::{
        Distinct, Expr, GroupByExpr, JoinConstraint, JoinOperator, LateralView, Query, Select,
        SelectItem, SetExpr, Statement, TableFactor, TableWithJoins, Top, TopQuantity,
    },
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

fn walk_distinct(distinct: &Distinct) -> Vec<String> {
    match distinct {
        Distinct::On(vecexpr) => {
            let mut select_statement = vec![];
            for expr in vecexpr {
                select_statement.extend(walk_expr(&expr));
            }
            select_statement
        }
        _ => {
            vec![]
        }
    }
}

fn walk_quantity(quantity: &TopQuantity) -> Vec<String> {
    match quantity {
        TopQuantity::Expr(expr) => {
            return walk_expr(&expr);
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_top(top: &Top) -> Vec<String> {
    if let Some(quantity) = &top.quantity {
        return walk_quantity(&quantity);
    };

    vec![]
}

fn walk_select_item(select_item: &SelectItem) -> Vec<String> {
    match select_item {
        SelectItem::UnnamedExpr(expr) => {
            return walk_expr(expr);
        }
        SelectItem::ExprWithAlias { expr, .. } => {
            return walk_expr(expr);
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_lateral_view(lateral_view: &LateralView) -> Vec<String> {
    return walk_expr(&lateral_view.lateral_view);
}

fn walk_group_by_expr(group_by_expr: &GroupByExpr) -> Vec<String> {
    match group_by_expr {
        GroupByExpr::Expressions(vecexpr, _) => {
            let mut select_statement = vec![];
            for expr in vecexpr {
                select_statement.extend(walk_expr(&expr));
            }
            return select_statement;
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_select(select: &Select) -> Vec<String> {
    //println!("Select: {:?}", select);
    //println!("Select: {:?}", select.to_string());
    let mut select_statement = vec![select.to_string()];

    if let Some(distinct) = &select.distinct {
        select_statement.extend(walk_distinct(&distinct));
    };

    if let Some(top) = &select.top {
        select_statement.extend(walk_top(&top));
    };

    for select_item in &select.projection {
        select_statement.extend(walk_select_item(&select_item));
    }

    select_statement.extend(walk_table_with_joins(&select.from));

    for lateral_view in &select.lateral_views {
        select_statement.extend(walk_lateral_view(&lateral_view));
    }

    if let Some(prewhere) = &select.prewhere {
        select_statement.extend(walk_expr(&prewhere));
    };

    if let Some(selection) = &select.selection {
        select_statement.extend(walk_expr(&selection));
    };

    select_statement.extend(walk_group_by_expr(&select.group_by));

    for expr in &select.cluster_by {
        select_statement.extend(walk_expr(&expr));
    }

    for expr in &select.distribute_by {
        select_statement.extend(walk_expr(&expr));
    }

    for expr in &select.sort_by {
        select_statement.extend(walk_expr(&expr));
    }

    if let Some(having) = &select.having {
        select_statement.extend(walk_expr(&having));
    };
    return select_statement;
}

fn walk_setexpr(setexpr: &SetExpr) -> Vec<String> {
    match setexpr {
        SetExpr::Select(select) => {
            return walk_select(select);
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
