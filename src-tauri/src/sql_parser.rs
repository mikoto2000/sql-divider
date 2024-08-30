use sqlparser::{
    ast::{
        Array, ConnectBy, Cte, DictionaryField, Distinct, Expr, ExprWithAlias, Fetch, Function,
        FunctionArg, FunctionArgExpr, FunctionArgumentClause, FunctionArgumentList,
        FunctionArguments, GroupByExpr, HavingBound, Interpolate, InterpolateExpr, Interval, Join,
        JoinConstraint, JoinOperator, LambdaFunction, LateralView, ListAggOnOverflow, Map,
        MapEntry, Measure, NamedWindowDefinition, NamedWindowExpr, Offset, OrderBy, OrderByExpr,
        PivotValueSource, Query, Select, SelectItem, SetExpr, Statement, SymbolDefinition,
        TableFactor, TableFunctionArgs, TableVersion, TableWithJoins, Top, TopQuantity, WindowSpec,
        WindowType, With, WithFill,
    },
    dialect::PostgreSqlDialect,
    parser::{Parser, ParserError},
};

pub async fn find_select_statement(
    sql: &String,
) -> Result<(Vec<String>, Vec<String>), ParserError> {
    let dialect = PostgreSqlDialect {};

    let ast = Parser::parse_sql(&dialect, &sql)?;

    let mut select_statements: (Vec<String>, Vec<String>) = (vec![], vec![]);
    for statement in ast.iter() {
        let (with, select) = walk_statement(statement);
        select_statements.0.extend(with);
        select_statements.1.extend(select);
    }
    return Ok(select_statements);
}

fn walk_statement(statement: &Statement) -> (Vec<String>, Vec<String>) {
    //println!("{:?}", statement);
    match statement {
        Statement::Query(query) => {
            return walk_query(query);
        }
        _ => {
            return (vec![], vec![]);
        }
    }
}

fn walk_distinct(distinct: &Distinct) -> (Vec<String>, Vec<String>) {
    match distinct {
        Distinct::On(vecexpr) => {
            let mut select_statement = (vec![], vec![]);
            for expr in vecexpr {
                let (w, s) = walk_expr(&expr);
                select_statement.0.extend(w);
                select_statement.1.extend(s);
            }
            select_statement
        }
        _ => (vec![], vec![]),
    }
}

fn walk_quantity(quantity: &TopQuantity) -> (Vec<String>, Vec<String>) {
    match quantity {
        TopQuantity::Expr(expr) => {
            return walk_expr(&expr);
        }
        _ => {
            return (vec![], vec![]);
        }
    }
}

fn walk_top(top: &Top) -> (Vec<String>, Vec<String>) {
    if let Some(quantity) = &top.quantity {
        return walk_quantity(&quantity);
    };

    (vec![], vec![])
}

fn walk_select_item(select_item: &SelectItem) -> (Vec<String>, Vec<String>) {
    match select_item {
        SelectItem::UnnamedExpr(expr) => {
            return walk_expr(expr);
        }
        SelectItem::ExprWithAlias { expr, .. } => {
            return walk_expr(expr);
        }
        _ => {
            return (vec![], vec![]);
        }
    }
}

fn walk_lateral_view(lateral_view: &LateralView) -> (Vec<String>, Vec<String>) {
    return walk_expr(&lateral_view.lateral_view);
}

fn walk_group_by_expr(group_by_expr: &GroupByExpr) -> (Vec<String>, Vec<String>) {
    match group_by_expr {
        GroupByExpr::Expressions(vecexpr, _) => {
            let mut select_statement = (vec![], vec![]);
            for expr in vecexpr {
                let (w, s) = walk_expr(&expr);
                select_statement.0.extend(w);
                select_statement.1.extend(s);
            }
            return select_statement;
        }
        _ => {
            return (vec![], vec![]);
        }
    }
}

fn walk_with_fill(with_fill: &WithFill) -> (Vec<String>, Vec<String>) {
    let mut select_statement = (vec![], vec![]);

    if let Some(expr) = &with_fill.from {
        let (w, s) = walk_expr(&expr);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    };

    if let Some(expr) = &with_fill.to {
        let (w, s) = walk_expr(&expr);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    };

    if let Some(expr) = &with_fill.step {
        let (w, s) = walk_expr(&expr);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    };

    select_statement
}

fn walk_order_by_expr(order_by_expr: &OrderByExpr) -> (Vec<String>, Vec<String>) {
    let mut select_statement = (vec![], vec![]);

    let (w, s) = walk_expr(&order_by_expr.expr);
    select_statement.0.extend(w);
    select_statement.1.extend(s);

    if let Some(with_fill) = &order_by_expr.with_fill {
        let (w, s) = walk_with_fill(&with_fill);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    };

    select_statement
}

fn walk_window_spec(window_spec: &WindowSpec) -> (Vec<String>, Vec<String>) {
    let mut select_statement = (vec![], vec![]);

    for expr in &window_spec.partition_by {
        let (w, s) = walk_expr(expr);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    }

    for order_by_expr in &window_spec.order_by {
        let (w, s) = walk_order_by_expr(order_by_expr);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    }

    select_statement
}

fn walk_named_window_expr(named_window_expr: &NamedWindowExpr) -> (Vec<String>, Vec<String>) {
    match named_window_expr {
        NamedWindowExpr::WindowSpec(window_spec) => {
            return walk_window_spec(window_spec);
        }
        _ => {
            return (vec![], vec![]);
        }
    }
}

fn walk_named_window_definition(
    named_window_definition: &NamedWindowDefinition,
) -> (Vec<String>, Vec<String>) {
    return walk_named_window_expr(&named_window_definition.1);
}

fn walk_connect_by(connect_by: &ConnectBy) -> (Vec<String>, Vec<String>) {
    let mut select_statement = (vec![], vec![]);

    let (w, s) = walk_expr(&connect_by.condition);
    select_statement.0.extend(w);
    select_statement.1.extend(s);

    for relationship in &connect_by.relationships {
        let (w, s) = walk_expr(&relationship);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    }

    select_statement
}

fn walk_select(select: &Select) -> (Vec<String>, Vec<String>) {
    //println!("Select: {:?}", select);
    //println!("Select: {:?}", select.to_string());
    let mut select_statement = (vec![], vec![select.to_string()]);

    if let Some(distinct) = &select.distinct {
        let (w, s) = walk_distinct(&distinct);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    };

    if let Some(top) = &select.top {
        let (w, s) = walk_top(&top);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    };

    for select_item in &select.projection {
        let (w, s) = walk_select_item(&select_item);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    }

    for table_with_join in &select.from {
        let (w, s) = walk_table_with_joins(table_with_join);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    }

    for lateral_view in &select.lateral_views {
        let (w, s) = walk_lateral_view(&lateral_view);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    }

    if let Some(prewhere) = &select.prewhere {
        let (w, s) = walk_expr(&prewhere);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    };

    if let Some(selection) = &select.selection {
        let (w, s) = walk_expr(&selection);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    };

    let (w, s) = walk_group_by_expr(&select.group_by);
    select_statement.0.extend(w);
    select_statement.1.extend(s);

    for expr in &select.cluster_by {
        let (w, s) = walk_expr(&expr);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    }

    for expr in &select.distribute_by {
        let (w, s) = walk_expr(&expr);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    }

    for expr in &select.sort_by {
        let (w, s) = walk_expr(&expr);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    }

    if let Some(having) = &select.having {
        let (w, s) = walk_expr(&having);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    };

    for named_window_definition in &select.named_window {
        let (w, s) = walk_named_window_definition(&named_window_definition);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    }

    if let Some(expr) = &select.qualify {
        let (w, s) = walk_expr(&expr);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    };

    if let Some(connect_by) = &select.connect_by {
        let (w, s) = walk_connect_by(&connect_by);
        select_statement.0.extend(w);
        select_statement.1.extend(s);
    };

    return select_statement;
}

fn walk_setexpr(setexpr: &SetExpr) -> (Vec<String>, Vec<String>) {
    match setexpr {
        SetExpr::Select(select) => {
            return walk_select(select);
        }
        SetExpr::Query(query) => {
            return walk_query(query);
        }
        SetExpr::SetOperation { left, right, .. } => {
            let mut select_statement = (vec![], vec![]);
            let (with, select) = walk_setexpr(left);
            select_statement.0.extend(with);
            select_statement.1.extend(select);

            let (with, select) = walk_setexpr(right);
            select_statement.0.extend(with);
            select_statement.1.extend(select);

            return select_statement;
        }
        _ => {
            return (vec![], vec![]);
        }
    }
}

fn walk_cte(cte: &Cte) -> (Vec<String>, Vec<String>) {
    return walk_query(&cte.query);
}

fn walk_with(with: &With) -> (Vec<String>, Vec<String>) {
    let mut select_statements = (vec![with.to_string()], vec![]);

    for cte in &with.cte_tables {
        let (with, select) = walk_cte(&cte);
        select_statements.0.extend(with);
        select_statements.1.extend(select);
    }

    select_statements
}

fn walk_interpolate_expr(interpolate_expr: &InterpolateExpr) -> (Vec<String>, Vec<String>) {
    if let Some(expr) = &interpolate_expr.expr {
        return walk_expr(&expr);
    } else {
        return (vec![], vec![]);
    }
}

fn walk_interpolate(interpolate: &Interpolate) -> (Vec<String>, Vec<String>) {
    let mut select_statements = (vec![], vec![]);

    if let Some(exprs) = &interpolate.exprs {
        for expr in exprs {
            let (w, s) = walk_interpolate_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);
        }
    }

    select_statements
}

fn walk_order_by(order_by: &OrderBy) -> (Vec<String>, Vec<String>) {
    let mut select_statements = (vec![], vec![]);

    for expr in &order_by.exprs {
        let (w, s) = walk_order_by_expr(expr);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    if let Some(interpolate) = &order_by.interpolate {
        let (w, s) = walk_interpolate(&interpolate);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    select_statements
}

fn walk_offset(offset: &Offset) -> (Vec<String>, Vec<String>) {
    return walk_expr(&offset.value);
}

fn walk_fetch(fetch: &Fetch) -> (Vec<String>, Vec<String>) {
    if let Some(expr) = &fetch.quantity {
        return walk_expr(&expr);
    } else {
        return (vec![], vec![]);
    }
}

fn walk_query(query: &Query) -> (Vec<String>, Vec<String>) {
    let mut select_statements = (vec![], vec![]);

    if let Some(with) = &query.with {
        let (w, s) = walk_with(&with);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    let (w, s) = walk_setexpr(&query.body);
    select_statements.0.extend(w);
    select_statements.1.extend(s);

    if let Some(order_by) = &query.order_by {
        let (w, s) = walk_order_by(&order_by);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    if let Some(limit) = &query.limit {
        let (w, s) = walk_expr(&limit);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    for limit_by_elem in &query.limit_by {
        let (w, s) = walk_expr(&limit_by_elem);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    if let Some(offset) = &query.offset {
        let (w, s) = walk_offset(&offset);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    if let Some(fetch) = &query.fetch {
        let (w, s) = walk_fetch(&fetch);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    select_statements
}

fn walk_list_agg_on_overflow(list_agg_on_overflow: &ListAggOnOverflow) -> (Vec<String>, Vec<String>) {
    match list_agg_on_overflow {
        ListAggOnOverflow::Truncate { filler, .. } => {
            if let Some(filler) = &filler {
                return walk_expr(&filler);
            } else {
                return (vec![], vec![]);
            }
        }
        _ => {
            return (vec![], vec![]);
        }
    }
}

fn walk_having_bound(having_bound: &HavingBound) -> (Vec<String>, Vec<String>) {
    return walk_expr(&having_bound.1);
}

fn walk_function_argument_clause(function_argument_clause: &FunctionArgumentClause) -> (Vec<String>, Vec<String>) {
    match function_argument_clause {
        FunctionArgumentClause::OrderBy(order_by) => {
            let mut select_statements = (vec![], vec![]);

            for order_by_expr in order_by {
                let (w, s) = walk_order_by_expr(&order_by_expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
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
            return (vec![], vec![]);
        }
    }
}

fn walk_function_argument_list(function_argument_list: &FunctionArgumentList) -> (Vec<String>, Vec<String>) {
    let mut select_statements = (vec![], vec![]);
    for function_arg in &function_argument_list.args {
        let (w, s) = walk_function_arg(&function_arg);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    for function_argument_clause in &function_argument_list.clauses {
        let (w, s) = walk_function_argument_clause(&function_argument_clause);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    select_statements
}

fn walk_function_arguments(function_arguments: &FunctionArguments) -> (Vec<String>, Vec<String>) {
    match function_arguments {
        FunctionArguments::Subquery(subquery) => {
            return walk_query(&subquery);
        }
        FunctionArguments::List(list) => {
            return walk_function_argument_list(&list);
        }
        _ => {
            return (vec![], vec![]);
        }
    }
}

fn walk_window_type(window_type: &WindowType) -> (Vec<String>, Vec<String>) {
    match window_type {
        WindowType::WindowSpec(window_spec) => {
            return walk_window_spec(&window_spec);
        }
        _ => {
            return (vec![], vec![]);
        }
    }
}

fn walk_function(function: &Function) -> (Vec<String>, Vec<String>) {
    let mut select_statements = (vec![], vec![]);

    let (w, s) = walk_function_arguments(&function.parameters);
    select_statements.0.extend(w);
    select_statements.1.extend(s);

    let (w, s) = walk_function_arguments(&function.args);
    select_statements.0.extend(w);
    select_statements.1.extend(s);

    if let Some(filter) = &function.filter {
        let (w, s) = walk_expr(&filter);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    };

    if let Some(window_type) = &function.over {
        let (w, s) = walk_window_type(&window_type);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    };

    for order_by_expr in &function.within_group {
        let (w, s) = walk_order_by_expr(&order_by_expr);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    select_statements
}

fn walk_dictionary_field(dictionary_field: &DictionaryField) -> (Vec<String>, Vec<String>) {
    return walk_expr(&dictionary_field.value);
}

fn walk_map_entry(map_entry: &MapEntry) -> (Vec<String>, Vec<String>) {
    let mut select_statements = (vec![], vec![]);

    let (w, s) = walk_expr(&map_entry.key);
    select_statements.0.extend(w);
    select_statements.1.extend(s);

    let (w, s) = walk_expr(&map_entry.value);
    select_statements.0.extend(w);
    select_statements.1.extend(s);

    select_statements
}

fn walk_map(map: &Map) -> (Vec<String>, Vec<String>) {
    let mut select_statements = (vec![], vec![]);

    for entry in &map.entries {
        let (w, s) = walk_map_entry(&entry);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    select_statements
}

fn walk_array(array: &Array) -> (Vec<String>, Vec<String>) {
    let mut select_statements = (vec![], vec![]);

    for e in &array.elem {
        let (w, s) = walk_expr(&e);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    select_statements
}

fn walk_interval(interval: &Interval) -> (Vec<String>, Vec<String>) {
    return walk_expr(&interval.value);
}

fn walk_lambda_function(lambda_function: &LambdaFunction) -> (Vec<String>, Vec<String>) {
    return walk_expr(&lambda_function.body);
}

fn walk_expr(expr: &Expr) -> (Vec<String>, Vec<String>) {
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
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr1);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&expr2);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::IsNotDistinctFrom(expr1, expr2) => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr1);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&expr2);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::InList { expr, list, .. } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            for expr in list {
                let (w, s) = walk_expr(&expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            select_statements
        }
        Expr::InSubquery { expr, subquery, .. } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_query(&subquery);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::InUnnest {
            expr, array_expr, ..
        } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&array_expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::Between {
            expr, low, high, ..
        } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&low);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&high);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::BinaryOp { left, right, .. } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&left);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&right);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::Like { expr, pattern, .. } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&pattern);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::ILike { expr, pattern, .. } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&pattern);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::SimilarTo { expr, pattern, .. } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&pattern);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::RLike { expr, pattern, .. } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&pattern);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::AnyOp { left, right, .. } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&left);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&right);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::AllOp { left, right, .. } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&left);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&right);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::UnaryOp { expr, .. } => {
            return walk_expr(&expr);
        }
        Expr::Convert { expr, styles, .. } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            for style in styles {
                let (w, s) = walk_expr(&style);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
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
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&timestamp);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&time_zone);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

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
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&r#in);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        Expr::Substring {
            expr,
            substring_from,
            substring_for,
            ..
        } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            if let Some(substring_from) = &substring_from {
                let (w, s) = walk_expr(&substring_from);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            };

            if let Some(substring_for) = &substring_for {
                let (w, s) = walk_expr(&substring_for);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            };

            select_statements
        }
        Expr::Trim {
            expr,
            trim_what,
            trim_characters,
            ..
        } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            if let Some(trim_what) = &trim_what {
                let (w, s) = walk_expr(&trim_what);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            };

            if let Some(trim_characters) = &trim_characters {
                for trim_character in trim_characters {
                    let (w, s) = walk_expr(&trim_character);
                    select_statements.0.extend(w);
                    select_statements.1.extend(s);
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
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(&expr);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&overlay_what);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_expr(&overlay_from);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            if let Some(overlay_for) = &overlay_for {
                let (w, s) = walk_expr(&overlay_for);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
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
        Expr::Case {
            operand,
            conditions,
            results,
            else_result,
        } => {
            let mut select_statements = (vec![], vec![]);

            if let Some(operand) = &operand {
                let (w, s) = walk_expr(&operand);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            for expr in conditions {
                let (w, s) = walk_expr(&expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            for expr in results {
                let (w, s) = walk_expr(&expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            if let Some(else_result) = &else_result {
                let (w, s) = walk_expr(&else_result);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            select_statements
        }
        Expr::Exists { subquery, .. } => {
            return walk_query(&subquery);
        }
        Expr::Subquery(subquery) => {
            return walk_query(subquery);
        }
        Expr::GroupingSets(grouping_sets) => {
            let mut select_statements = (vec![], vec![]);

            for exprs in grouping_sets {
                for expr in exprs {
                    let (w, s) = walk_expr(&expr);
                    select_statements.0.extend(w);
                    select_statements.1.extend(s);
                }
            }

            select_statements
        }
        Expr::Cube(cube) => {
            let mut select_statements = (vec![], vec![]);

            for exprs in cube {
                for expr in exprs {
                    let (w, s) = walk_expr(&expr);
                    select_statements.0.extend(w);
                    select_statements.1.extend(s);
                }
            }

            select_statements
        }
        Expr::Rollup(rollup) => {
            let mut select_statements = (vec![], vec![]);

            for exprs in rollup {
                for expr in exprs {
                    let (w, s) = walk_expr(&expr);
                    select_statements.0.extend(w);
                    select_statements.1.extend(s);
                }
            }

            select_statements
        }
        Expr::Tuple(tuple) => {
            let mut select_statements = (vec![], vec![]);

            for expr in tuple {
                let (w, s) = walk_expr(&expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            select_statements
        }
        Expr::Struct { values, .. } => {
            let mut select_statements = (vec![], vec![]);

            for expr in values {
                let (w, s) = walk_expr(&expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            select_statements
        }
        Expr::Named { expr, .. } => {
            return walk_expr(&expr);
        }
        Expr::Dictionary(vec_dictionary_field) => {
            let mut select_statements = (vec![], vec![]);

            for dictionary_field in vec_dictionary_field {
                let (w, s) = walk_dictionary_field(&dictionary_field);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            select_statements
        }
        Expr::Map(map) => {
            return walk_map(&map);
        }
        Expr::Array(array) => {
            return walk_array(&array);
        }
        Expr::Interval(interval) => {
            return walk_interval(&interval);
        }
        Expr::OuterJoin(outer_join) => {
            return walk_expr(&outer_join);
        }
        Expr::Prior(prior) => {
            return walk_expr(&prior);
        }
        Expr::Lambda(lambda) => {
            return walk_lambda_function(&lambda);
        }
        _ => {
            return (vec![], vec![]);
        }
    }
}

fn walk_function_arg_expr(function_arg_expr: &FunctionArgExpr) -> (Vec<String>, Vec<String>) {
    match function_arg_expr {
        FunctionArgExpr::Expr(expr) => {
            return walk_expr(&expr);
        }
        _ => {
            return (vec![], vec![]);
        }
    }
}

fn walk_function_arg(function_arg: &FunctionArg) -> (Vec<String>, Vec<String>) {
    match function_arg {
        FunctionArg::Named { arg, .. } => {
            return walk_function_arg_expr(&arg);
        }
        FunctionArg::Unnamed(function_arg_expr) => {
            return walk_function_arg_expr(&function_arg_expr);
        }
    }
}

fn walk_table_function_args(table_function_args: &TableFunctionArgs) -> (Vec<String>, Vec<String>) {
    let mut select_statements = (vec![], vec![]);

    for arg in &table_function_args.args {
        let (w, s) = walk_function_arg(&arg);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    select_statements
}

fn walk_table_version(table_version: &TableVersion) -> (Vec<String>, Vec<String>) {
    match table_version {
        TableVersion::ForSystemTimeAsOf(expr) => {
            return walk_expr(&expr);
        }
    }
}

fn walk_expr_with_alias(expr_with_alias: &ExprWithAlias) -> (Vec<String>, Vec<String>) {
    return walk_expr(&expr_with_alias.expr);
}

fn walk_pivot_value_source(pivot_value_source: &PivotValueSource) -> (Vec<String>, Vec<String>) {
    match pivot_value_source {
        PivotValueSource::List(vecexpr) => {
            let mut select_statements = (vec![], vec![]);

            for expr in vecexpr {
                let (w, s) = walk_expr_with_alias(&expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            select_statements
        }
        PivotValueSource::Any(vecexpr) => {
            let mut select_statements = (vec![], vec![]);

            for expr in vecexpr {
                let (w, s) = walk_order_by_expr(&expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            select_statements
        }
        PivotValueSource::Subquery(query) => {
            return walk_query(&query);
        }
    }
}

fn walk_measure(measure: &Measure) -> (Vec<String>, Vec<String>) {
    return walk_expr(&measure.expr);
}

fn walk_symbol_definition(symbol_definition: &SymbolDefinition) -> (Vec<String>, Vec<String>) {
    return walk_expr(&symbol_definition.definition);
}

fn walk_table_factor(table_factor: &TableFactor) -> (Vec<String>, Vec<String>) {
    match table_factor {
        TableFactor::Table {
            args,
            with_hints,
            version,
            ..
        } => {
            let mut select_statements = (vec![], vec![]);

            if let Some(args) = args {
                let (w, s) = walk_table_function_args(args);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            for expr in with_hints {
                let (w, s) = walk_expr(expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            if let Some(version) = version {
                let (w, s) = walk_table_version(version);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
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
            let mut select_statements = (vec![], vec![]);

            for function_arg in args {
                let (w, s) = walk_function_arg(&function_arg);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            select_statements
        }
        TableFactor::UNNEST { array_exprs, .. } => {
            let mut select_statements = (vec![], vec![]);
            for expr in array_exprs {
                let (w, s) = walk_expr(&expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
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
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_table_factor(&table);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            for expr_with_alias in aggregate_functions {
                let (w, s) = walk_expr_with_alias(&expr_with_alias);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            let (w, s) = walk_pivot_value_source(&value_source);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            if let Some(expr) = default_on_null {
                let (w, s) = walk_expr(expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
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
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_table_factor(&table);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            for expr in partition_by {
                let (w, s) = walk_expr(&expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            for expr in order_by {
                let (w, s) = walk_order_by_expr(&expr);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            for measure in measures {
                let (w, s) = walk_measure(&measure);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            for symbol_definition in symbols {
                let (w, s) = walk_symbol_definition(&symbol_definition);
                select_statements.0.extend(w);
                select_statements.1.extend(s);
            }

            select_statements
        }
    }
}

fn walk_join_operator(join_operator: &JoinOperator) -> (Vec<String>, Vec<String>) {
    match join_operator {
        JoinOperator::Inner(join_constraint) => {
            return walk_join_constraint(join_constraint);
        }
        JoinOperator::LeftOuter(join_constraint) => {
            return walk_join_constraint(join_constraint);
        }
        JoinOperator::RightOuter(join_constraint) => {
            return walk_join_constraint(join_constraint);
        }
        JoinOperator::FullOuter(join_constraint) => {
            return walk_join_constraint(join_constraint);
        }
        JoinOperator::LeftSemi(join_constraint) => {
            return walk_join_constraint(join_constraint);
        }
        JoinOperator::RightSemi(join_constraint) => {
            return walk_join_constraint(join_constraint);
        }
        JoinOperator::LeftAnti(join_constraint) => {
            return walk_join_constraint(join_constraint);
        }
        JoinOperator::RightAnti(join_constraint) => {
            return walk_join_constraint(join_constraint);
        }
        JoinOperator::AsOf {
            match_condition,
            constraint,
        } => {
            let mut select_statements = (vec![], vec![]);

            let (w, s) = walk_expr(match_condition);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            let (w, s) = walk_join_constraint(constraint);
            select_statements.0.extend(w);
            select_statements.1.extend(s);

            select_statements
        }
        _ => {
            return (vec![], vec![]);
        }
    }
}

fn walk_join(join: &Join) -> (Vec<String>, Vec<String>) {
    let mut select_statements = (vec![], vec![]);

    let (w, s) = walk_table_factor(&join.relation);
    select_statements.0.extend(w);
    select_statements.1.extend(s);

    let (w, s) = walk_join_operator(&join.join_operator);
    select_statements.0.extend(w);
    select_statements.1.extend(s);

    select_statements
}

fn walk_join_constraint(join_constraint: &JoinConstraint) -> (Vec<String>, Vec<String>) {
    match join_constraint {
        JoinConstraint::On(on) => walk_expr(&on),
        _ => (vec![], vec![]),
    }
}

fn walk_table_with_joins(twjs: &TableWithJoins) -> (Vec<String>, Vec<String>) {
    //println!("{:?}", twjs);
    let mut select_statements = (vec![], vec![]);

    let (w, s) = walk_table_factor(&twjs.relation);
    select_statements.0.extend(w);
    select_statements.1.extend(s);

    for join in &twjs.joins {
        let (w, s) = walk_join(&join);
        select_statements.0.extend(w);
        select_statements.1.extend(s);
    }

    return select_statements;
}
