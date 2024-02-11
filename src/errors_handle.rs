use postgres::Error;

trait HandleErrorsToJson {
    fn handle_errors_str_to_json(self) -> String;
}

impl HandleErrorsToJson for Result<String, Error> {
    fn handle_errors_str_to_json(self) -> String {
        match self {
            Ok(string) => serde_json::to_string(&string).unwrap_or_else(|err| {
                format!("{{\"error\":\"Failed to serialize string: {}\"}}", err)
            }),
            Err(err) => {
                format!("{{\"error\":\"Err: {}\"}}", err)
            }
        }
    }
}
