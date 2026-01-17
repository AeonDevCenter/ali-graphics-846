# System Architecture

## Overview
The distributor follows a **Redirect-on-Authenticated-Request** pattern.

### 1. Ingestion & Metadata Fetching
When a user requests `/file?name=...`, the server:
1. Validates the filename.
2. Calls the S3 `HeadObject` API. This is a low-latency call that returns only metadata (size, type, last modified).
3. Renders the template server-side to prevent "Flicker of Unstyled Content" (FOUC).

### 2. Secure Redirection Pipeline
The download process is designed for maximum throughput:
- **Handler:** `download_handler.rs`
- **Logic:** Instead of streaming bytes through the Axum runtime (which would block the event loop for large files), we generate an **IAM-scoped Presigned URL**.
- **Result:** The server returns a `307 Temporary Redirect`. The client’s browser then pulls the file directly from AWS's global edge network.



## Memory Management
Since we use `multipart` streaming and `HeadObject` calls, the server's memory footprint remains constant (~15-20MB RAM) regardless of whether the file being shared is 1MB or 10GB.