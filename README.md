# 🦀 s3_asset_distributor

A high-performance gateway for secure, private AWS S3 asset delivery. Built with **Rust**, **Axum**, and **Askama**.



## 🚀 The Mission
Serving large assets directly from a server is slow and expensive. `s3_asset_distributor` solves this by acting as a secure metadata layer. It provides beautiful previews and offloads the heavy lifting (bandwidth) to AWS S3 using time-limited presigned URLs.

## ✨ Core Features
- **Zero-Proxy Downloads:** Users download directly from S3, saving your server's CPU and RAM.
- **Smart Metadata:** Uses S3 `HeadObject` to peek at file stats without downloading the file.
- **Type-Safe Templates:** Powered by Askama for compile-time HTML safety.
- **Modern UI:** Responsive, glassmorphic design with native light/dark mode support.

## 🛠 Tech Stack
- **Language:** Rust 1.75+
- **Framework:** Axum (Tokio-based)
- **Cloud:** AWS SDK for Rust (S3)
- **Templating:** Askama (Jinja-style)

## 📖 Documentation
- [Architecture & Design Decisions](docs/ARCHITECTURE.md)
- [Deployment & Environment Setup](docs/DEPLOYMENT.md)
- [API Endpoints Reference](docs/API_SPEC.md)

---
Built by **Aeon Roamer** | 19yo Backend Engineer focused on performance.