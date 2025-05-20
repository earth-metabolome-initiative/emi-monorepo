# Generic backend request errors

Crate defining high-level errors that may occur in the backend when the user performs a request to it.

These errors do not provide detailed information about the error, but rather a high-level descriptions such as whether
the request was `Unauthorized`, or `ServerError`, etc.
