use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "sql.pest"] // .pest defines grammar of SQL 
struct SQLParser;

fn parse_sql(query: &str) {
    let pairs = SQLParser::parse(Rule::sql, query)
        .expect("Failed to parse SQL")
        .next().unwrap();

    for pair in pairs.into_inner() {
        match pair.as_rule() {
            Rule::select_statement => {
                println!("Parsed a SELECT statement: {:?}", pair);
            },
            Rule::insert_statement => {
                println!("Parsed an INSERT statement: {:?}", pair);
            },
            _ => {}
        }
    }
}

