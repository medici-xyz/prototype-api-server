// #[derive(Error, Debug)]
// pub enum ExchangeError {
//     #[error("Postgres Error { source: ?}")]
//     PostgresError {
//         #[from]
//     }
// }

pub fn throw_json_error(
    library: &str,
    function: &Vec<&str>,
    src_file: &str,
    line_num: &str,
    err_type: &str,
) -> String {
    serde_json::json!({
        "error_library": library,
        "error_function": function,
        "error_src_file": src_file,
        "error_line_number": line_num,
        "error_type": err_type
    })
    .to_string()
}
