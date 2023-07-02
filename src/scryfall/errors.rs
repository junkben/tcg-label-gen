/// An [`Error`](https://scryfall.com/docs/api/errors) object represents a
/// failure to find information or understand the input you provided to the API.
/// `Error` objects are always transmitted with the appropriate `4XX` or `5XX`
/// HTTP status code.
#[derive(Debug, Deserialize, Serialize)]
pub struct ScryfallError {
    /// An integer HTTP status code for this error.
    status:     u32,
    /// A computer-friendly string representing the appropriate HTTP status
    /// code.
    code:       String,
    /// A human-readable string explaining the error.
    details:    String,
    /// A computer-friendly string that provides additional context for the
    /// main error. For example, an endpoint many generate `HTTP 404` errors
    /// for different kinds of input. This field will provide a label for the
    /// specific kind of 404 failure, such as `ambiguous`.
    #[serde(rename = "type")]
    error_type: Option<String>,
    /// If your input also generated non-failure warnings, they will be
    /// provided as human-readable strings in this array.
    warnings:   Option<Vec<String>>
}
