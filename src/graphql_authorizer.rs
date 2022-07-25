use apollo_parser::{ast, Parser};

pub fn get_selections_from_field(gql_query: &str, field_name: &str) -> Vec<String> {
    let mut fields_read: Vec<String> = Vec::new();

    let parser = Parser::new(gql_query);
    let ast = parser.parse();
    assert_eq!(0, ast.errors().len());

    let doc = ast.document();
    for def in doc.definitions() {
        if let ast::Definition::OperationDefinition(op_def) = def {
            for selection in op_def.selection_set().unwrap().selections() {
                if let ast::Selection::Field(ref field) = selection {
                    if field.name().unwrap().text() == field_name {
                        for field_selection in field.selection_set().unwrap().selections() {
                            if let ast::Selection::Field(field_selection_field) = field_selection {
                                fields_read.push(
                                    String::from(field_selection_field.name().unwrap().text())
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    fields_read
}

pub fn get_fields_authorized(authorized_fields: Vec<String>, user: &str) -> Vec<String> {
    let mut fields_authorized: Vec<String> = Vec::new();

    for row in &authorized_fields {
        let split: Vec<&str> = row.split(":").collect();
        let cur_user = split[0];
        let fields = split[1].split(",");

        if cur_user == user {
            for field in fields {
                fields_authorized.push(String::from(field));
            }
        }
    }

    fields_authorized
}
