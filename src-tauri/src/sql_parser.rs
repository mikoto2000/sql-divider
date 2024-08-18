use sqlparser::{
    ast::{
        ConnectBy, Distinct, Expr, ExprWithAlias, Function, FunctionArg, FunctionArgExpr,
        FunctionArgumentClause, FunctionArgumentList, FunctionArguments, GroupByExpr, HavingBound,
        JoinConstraint, JoinOperator, LateralView, ListAggOnOverflow, Measure,
        NamedWindowDefinition, NamedWindowExpr, OrderByExpr, PivotValueSource, Query, Select,
        SelectItem, SetExpr, Statement, SymbolDefinition, TableFactor, TableFunctionArgs,
        TableVersion, TableWithJoins, Top, TopQuantity, WindowSpec, WindowType, WithFill,
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

fn walk_with_fill(with_fill: &WithFill) -> Vec<String> {
    let mut select_statement = vec![];

    if let Some(expr) = &with_fill.from {
        select_statement.extend(walk_expr(&expr));
    };

    if let Some(expr) = &with_fill.to {
        select_statement.extend(walk_expr(&expr));
    };

    if let Some(expr) = &with_fill.step {
        select_statement.extend(walk_expr(&expr));
    };

    select_statement
}

fn walk_order_by_expr(order_by_expr: &OrderByExpr) -> Vec<String> {
    let mut select_statement = vec![];

    select_statement.extend(walk_expr(&order_by_expr.expr));

    if let Some(with_fill) = &order_by_expr.with_fill {
        select_statement.extend(walk_with_fill(&with_fill));
    };

    select_statement
}

fn walk_window_spec(window_spec: &WindowSpec) -> Vec<String> {
    let mut select_statement = vec![];

    for expr in &window_spec.partition_by {
        select_statement.extend(walk_expr(expr));
    }

    for order_by_expr in &window_spec.order_by {
        select_statement.extend(walk_order_by_expr(order_by_expr));
    }

    select_statement
}

fn walk_named_window_expr(named_window_expr: &NamedWindowExpr) -> Vec<String> {
    match named_window_expr {
        NamedWindowExpr::WindowSpec(window_spec) => {
            return walk_window_spec(window_spec);
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_named_window_definition(named_window_definition: &NamedWindowDefinition) -> Vec<String> {
    return walk_named_window_expr(&named_window_definition.1);
}

fn walk_connect_by(connect_by: &ConnectBy) -> Vec<String> {
    let mut select_statement = vec![];

    select_statement.extend(walk_expr(&connect_by.condition));

    for relationship in &connect_by.relationships {
        select_statement.extend(walk_expr(&relationship));
    }

    select_statement
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

    for table_with_join in &select.from {
        select_statement.extend(walk_table_with_joins(table_with_join));
    }

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

    for named_window_definition in &select.named_window {
        select_statement.extend(walk_named_window_definition(&named_window_definition));
    }

    if let Some(expr) = &select.qualify {
        select_statement.extend(walk_expr(&expr));
    };

    if let Some(connect_by) = &select.connect_by {
        select_statement.extend(walk_connect_by(&connect_by));
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

fn walk_list_agg_on_overflow(list_agg_on_overflow: &ListAggOnOverflow) -> Vec<String> {
    match list_agg_on_overflow {
        ListAggOnOverflow::Truncate { filler, .. } => {
            if let Some(filler) = &filler {
                return walk_expr(&filler);
            } else {
                return vec![];
            }
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_having_bound(having_bound: &HavingBound) -> Vec<String> {
    return walk_expr(&having_bound.1);
}

fn walk_function_argument_clause(function_argument_clause: &FunctionArgumentClause) -> Vec<String> {
    match function_argument_clause {
        FunctionArgumentClause::OrderBy(order_by) => {
            let mut select_statements = vec![];

            for order_by_expr in order_by {
                select_statements.extend(walk_order_by_expr(&order_by_expr));
            }

            select_statements
        }
        FunctionArgumentClause::Limit(limit) => {
            return walk_expr(&limit);
        }
        FunctionArgumentClause::OnOverflow(overflow) => {
            return walk_list_agg_on_overflow(&overflow);
        }
        FunctionArgumentClause::Having(having) => {
            return walk_having_bound(having);
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_function_argument_list(function_argument_list: &FunctionArgumentList) -> Vec<String> {
    let mut select_statements = vec![];
    for function_arg in &function_argument_list.args {
        select_statements.extend(walk_function_arg(&function_arg));
    }

    for function_argument_clause in &function_argument_list.clauses {
        select_statements.extend(walk_function_argument_clause(&function_argument_clause));
    }

    select_statements
}

fn walk_function_arguments(function_arguments: &FunctionArguments) -> Vec<String> {
    match function_arguments {
        FunctionArguments::Subquery(subquery) => {
            return walk_query(&subquery);
        }
        FunctionArguments::List(list) => {
            return walk_function_argument_list(&list);
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_window_type(window_type: &WindowType) -> Vec<String> {
    match window_type {
        WindowType::WindowSpec(window_spec) => {
            return walk_window_spec(&window_spec);
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_function(function: &Function) -> Vec<String> {
    let mut select_statements = vec![];

    select_statements.extend(walk_function_arguments(&function.parameters));

    select_statements.extend(walk_function_arguments(&function.args));

    if let Some(filter) = &function.filter {
        select_statements.extend(walk_expr(&filter));
    };

    if let Some(window_type) = &function.over {
        select_statements.extend(walk_window_type(&window_type));
    };

    for order_by_expr in &function.within_group {
        select_statements.extend(walk_order_by_expr(&order_by_expr));
    }

    select_statements
}

fn walk_expr(expr: &Expr) -> Vec<String> {
    //println!("{:?}", expr);
    match &expr {
        Expr::JsonAccess { value, .. } => {
            return walk_expr(&value);
        }
        Expr::CompositeAccess { expr, .. } => {
            return walk_expr(&expr);
        }
        Expr::IsFalse(expr) => {
            return walk_expr(&expr);
        }
        Expr::IsNotFalse(expr) => {
            return walk_expr(&expr);
        }
        Expr::IsTrue(expr) => {
            return walk_expr(&expr);
        }
        Expr::IsNotTrue(expr) => {
            return walk_expr(&expr);
        }
        Expr::IsNull(expr) => {
            return walk_expr(&expr);
        }
        Expr::IsNotNull(expr) => {
            return walk_expr(&expr);
        }
        Expr::IsUnknown(expr) => {
            return walk_expr(&expr);
        }
        Expr::IsNotUnknown(expr) => {
            return walk_expr(&expr);
        }
        Expr::IsDistinctFrom(expr1, expr2) => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr1));

            select_statements.extend(walk_expr(&expr2));

            select_statements
        }
        Expr::IsNotDistinctFrom(expr1, expr2) => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr1));

            select_statements.extend(walk_expr(&expr2));

            select_statements
        }
        Expr::InList { expr, list, .. } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            for expr in list {
                select_statements.extend(walk_expr(&expr));
            }

            select_statements
        }
        Expr::InSubquery { expr, subquery, .. } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            select_statements.extend(walk_query(subquery));

            select_statements
        }
        Expr::InUnnest {
            expr, array_expr, ..
        } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            select_statements.extend(walk_expr(&array_expr));

            select_statements
        }
        Expr::Between {
            expr, low, high, ..
        } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            select_statements.extend(walk_expr(&low));

            select_statements.extend(walk_expr(&high));

            select_statements
        }
        Expr::BinaryOp { left, right, .. } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&left));

            select_statements.extend(walk_expr(&right));

            select_statements
        }
        Expr::Like { expr, pattern, .. } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            select_statements.extend(walk_expr(&pattern));

            select_statements
        }
        Expr::ILike { expr, pattern, .. } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            select_statements.extend(walk_expr(&pattern));

            select_statements
        }
        Expr::SimilarTo { expr, pattern, .. } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            select_statements.extend(walk_expr(&pattern));

            select_statements
        }
        Expr::RLike { expr, pattern, .. } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            select_statements.extend(walk_expr(&pattern));

            select_statements
        }
        Expr::AnyOp { left, right, .. } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&left));

            select_statements.extend(walk_expr(&right));

            select_statements
        }
        Expr::AllOp { left, right, .. } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&left));

            select_statements.extend(walk_expr(&right));

            select_statements
        }
        Expr::UnaryOp { expr, .. } => {
            return walk_expr(&expr);
        }
        Expr::Convert { expr, styles, .. } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            for style in styles {
                select_statements.extend(walk_expr(&style));
            }

            select_statements
        }
        Expr::Cast { expr, .. } => {
            return walk_expr(&expr);
        }
        Expr::AtTimeZone {
            timestamp,
            time_zone,
            ..
        } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&timestamp));

            select_statements.extend(walk_expr(&time_zone));

            select_statements
        }
        Expr::Extract { expr, .. } => {
            return walk_expr(&expr);
        }
        Expr::Ceil { expr, .. } => {
            return walk_expr(&expr);
        }
        Expr::Floor { expr, .. } => {
            return walk_expr(&expr);
        }
        Expr::Position { expr, r#in, .. } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            select_statements.extend(walk_expr(&r#in));

            select_statements
        }
        Expr::Substring {
            expr,
            substring_from,
            substring_for,
            ..
        } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            if let Some(substring_from) = &substring_from {
                select_statements.extend(walk_expr(&substring_from));
            };

            if let Some(substring_for) = &substring_for {
                select_statements.extend(walk_expr(&substring_for));
            };

            select_statements
        }
        Expr::Trim {
            expr,
            trim_what,
            trim_characters,
            ..
        } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            if let Some(trim_what) = &trim_what {
                select_statements.extend(walk_expr(&trim_what));
            };

            if let Some(trim_characters) = &trim_characters {
                for trim_character in trim_characters {
                    select_statements.extend(walk_expr(&trim_character));
                }
            };

            select_statements
        }
        Expr::Overlay {
            expr,
            overlay_what,
            overlay_from,
            overlay_for,
        } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_expr(&expr));

            select_statements.extend(walk_expr(&overlay_what));

            select_statements.extend(walk_expr(&overlay_from));

            if let Some(overlay_for) = &overlay_for {
                select_statements.extend(walk_expr(&overlay_for));
            };

            select_statements
        }
        Expr::Collate { expr, .. } => {
            return walk_expr(&expr);
        }
        Expr::Nested(expr) => {
            return walk_expr(&expr);
        }
        Expr::MapAccess { column, .. } => {
            return walk_expr(&column);
        }
        Expr::Function(function) => {
            return walk_function(&function);
        }
        Expr::Subquery(subquery) => {
            return walk_query(subquery);
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_function_arg_expr(function_arg_expr: &FunctionArgExpr) -> Vec<String> {
    match function_arg_expr {
        FunctionArgExpr::Expr(expr) => {
            return walk_expr(&expr);
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_function_arg(function_arg: &FunctionArg) -> Vec<String> {
    match function_arg {
        FunctionArg::Named { arg, .. } => {
            return walk_function_arg_expr(&arg);
        }
        FunctionArg::Unnamed(function_arg_expr) => {
            return walk_function_arg_expr(&function_arg_expr);
        }
    }
}

fn walk_table_function_args(table_function_args: &TableFunctionArgs) -> Vec<String> {
    let mut select_statements = vec![];

    for arg in &table_function_args.args {
        select_statements.extend(walk_function_arg(&arg));
    }

    select_statements
}

fn walk_table_version(table_version: &TableVersion) -> Vec<String> {
    match table_version {
        TableVersion::ForSystemTimeAsOf(expr) => {
            return walk_expr(&expr);
        }
    }
}

fn walk_expr_with_alias(expr_with_alias: &ExprWithAlias) -> Vec<String> {
    return walk_expr(&expr_with_alias.expr);
}

fn walk_pivot_value_source(pivot_value_source: &PivotValueSource) -> Vec<String> {
    match pivot_value_source {
        PivotValueSource::List(vecexpr) => {
            let mut select_statements = vec![];

            for expr in vecexpr {
                select_statements.extend(walk_expr_with_alias(&expr));
            }

            select_statements
        }
        PivotValueSource::Any(vecexpr) => {
            let mut select_statements = vec![];

            for expr in vecexpr {
                select_statements.extend(walk_order_by_expr(&expr));
            }

            select_statements
        }
        PivotValueSource::Subquery(query) => {
            return walk_query(&query);
        }
    }
}

fn walk_measure(measure: &Measure) -> Vec<String> {
    return walk_expr(&measure.expr);
}

fn walk_symbol_definition(symbol_definition: &SymbolDefinition) -> Vec<String> {
    return walk_expr(&symbol_definition.definition);
}

fn walk_table_factor(table_factor: &TableFactor) -> Vec<String> {
    match table_factor {
        TableFactor::Table {
            args,
            with_hints,
            version,
            ..
        } => {
            let mut select_statements = vec![];

            if let Some(args) = args {
                select_statements.extend(walk_table_function_args(args));
            }

            for expr in with_hints {
                select_statements.extend(walk_expr(expr));
            }

            if let Some(version) = version {
                select_statements.extend(walk_table_version(version));
            }

            select_statements
        }
        TableFactor::Derived { subquery, .. } => {
            return walk_query(subquery);
        }
        TableFactor::TableFunction { expr, .. } => {
            return walk_expr(&expr);
        }
        TableFactor::Function { args, .. } => {
            let mut select_statements = vec![];

            for function_arg in args {
                select_statements.extend(walk_function_arg(&function_arg));
            }

            select_statements
        }
        TableFactor::UNNEST { array_exprs, .. } => {
            let mut select_statements = vec![];
            for expr in array_exprs {
                select_statements.extend(walk_expr(&expr));
            }
            select_statements
        }
        TableFactor::JsonTable { json_expr, .. } => {
            return walk_expr(&json_expr);
        }
        TableFactor::NestedJoin {
            table_with_joins, ..
        } => {
            return walk_table_with_joins(table_with_joins);
        }
        TableFactor::Pivot {
            table,
            aggregate_functions,
            value_source,
            default_on_null,
            ..
        } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_table_factor(&table));

            for expr_with_alias in aggregate_functions {
                select_statements.extend(walk_expr_with_alias(&expr_with_alias));
            }

            select_statements.extend(walk_pivot_value_source(&value_source));

            if let Some(expr) = default_on_null {
                select_statements.extend(walk_expr(expr));
            }

            select_statements
        }
        TableFactor::Unpivot { table, .. } => {
            return walk_table_factor(&table);
        }
        TableFactor::MatchRecognize {
            table,
            partition_by,
            order_by,
            measures,
            symbols,
            ..
        } => {
            let mut select_statements = vec![];

            select_statements.extend(walk_table_factor(&table));

            for expr in partition_by {
                select_statements.extend(walk_expr(&expr));
            }

            for expr in order_by {
                select_statements.extend(walk_order_by_expr(&expr));
            }

            for measure in measures {
                select_statements.extend(walk_measure(&measure));
            }

            for symbol_definition in symbols {
                select_statements.extend(walk_symbol_definition(&symbol_definition));
            }

            select_statements
        }
        _ => {
            return vec![];
        }
    }
}

fn walk_table_with_joins(twjs: &TableWithJoins) -> Vec<String> {
    //println!("{:?}", twjs);
    let mut select_statements = vec![];

    select_statements.extend(walk_table_factor(&twjs.relation));

    for join in &twjs.joins {
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
    return select_statements;
}
