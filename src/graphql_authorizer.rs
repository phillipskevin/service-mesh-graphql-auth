use apollo_parser::{ast, Parser};

fn get_field_selection(parent_field: &str, gql_query: &str) -> Vec<String> {
    let mut fields_read: Vec<String> = Vec::new();

    let parser = Parser::new(gql_query);
    let ast = parser.parse();
    assert_eq!(0, ast.errors().len());

    let doc = ast.document();
    for def in doc.definitions() {
        if let ast::Definition::OperationDefinition(op_def) = def {
            for selection in op_def.selection_set().unwrap().selections() {
                if let ast::Selection::Field(ref field) = selection {
                    if field.name().unwrap().text() == parent_field {
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

fn get_fields_authorized(authorized_fields: &Vec<String>, user: &str) -> Vec<String> {
    let mut fields_authorized: Vec<String> = Vec::new();

    for row in authorized_fields {
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

pub struct GraphqlAuthorizer {
    pub authorized_fields_config: Vec<String>,
}

impl GraphqlAuthorizer {
    pub fn get_unauthorized_fields(&self, user: &str, gql: &str) -> Vec<String> {
        let person_fields_read = get_field_selection("people", gql);
        let people_fields_authorized = get_fields_authorized(&self.authorized_fields_config, &user[..]);

        let mut disallowed_fields = Vec::new();
        for field in person_fields_read {
            if !people_fields_authorized.contains(&field) {
                disallowed_fields.push(field);
            }
        }

        disallowed_fields
    }
}
