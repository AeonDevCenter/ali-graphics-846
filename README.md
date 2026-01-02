# Ali Graphics Download Server

A lightweight, high‑performance **file metadata & download server** built with **Rust, Axum, Askama, and AWS S3**.

This project is designed to serve **beautiful file preview pages** and **fast, direct downloads** using **S3 presigned
URLs**, while keeping the backend minimal, secure, and scalable.

---

## ✨ What This Server Does

Ali Graphics Download Server provides **two core paths** for working with files stored in AWS S3:

### 1️⃣ File Metadata Page (`/file`)

* Accepts a **query parameter** with the file name
* Fetches file metadata from S3
* Renders a **modern HTML card UI** showing:

    * File name
    * File type
    * File size
* Includes a **Download button**
* Fully responsive
* Automatic **light / dark mode**
* Trust‑focused, clean visual design

📌 Example:

```
/file?name=design-assets.zip
```

This page is rendered using **Askama templates** and custom CSS.

---

### 2️⃣ File Download (`/download`)

* Triggered by the **Download button** on the file page
* Accepts the same file name via query
* Generates a **presigned AWS S3 URL**
* Redirects the user to S3 for download

This approach:

* Avoids proxying files through the server
* Enables **maximum download speed**
* Works perfectly with browsers and download managers (IDM, etc.)

📌 Example:

```
/download?name=design-assets.zip
```

---

## 🚀 Key Features

* ⚡ **Blazing fast downloads** (direct S3)
* 🎨 **Modern UI** with glassmorphism & trust indicators
* 🔐 No AWS credentials exposed to users
* 🧠 Clean Axum routing & shared application state
* 🧩 Modular project structure
* 🛡 Safe error handling with friendly HTML error pages

---

## 🏗 Project Structure

```
ali-graphics-server/
├── src/
│   ├── app/            # App router
│   ├── handlers/       # Request handlers
│   │   ├── aws_file_handler.rs
│   │   └── download_handler.rs
│   ├── routes/         # Route grouping
│   ├── types/          # AppState, queries, templates
│   └── main.rs
├── templates/          # Askama HTML templates
│   ├── meta.html
│   └── error.html
├── static/             # CSS & assets
│   └── meta.css
├── Cargo.toml
├── Cargo.lock
└── .gitignore
```

---

## 🧠 How It Works (High Level)

1. Client opens `/file?name=...`
2. Server:

    * Calls `HeadObject` on S3
    * Builds a `FileMeta` template
    * Renders a beautiful HTML card
3. User clicks **Download**
4. Browser hits `/download?name=...`
5. Server:

    * Creates a presigned S3 URL
    * Responds with a redirect
6. Browser downloads directly from S3

---

## 🔧 Environment Variables

Create a `.env` file (not committed):

```
AWS_REGION=your-region
AWS_ACCESS_KEY_ID=your-key
AWS_SECRET_ACCESS_KEY=your-secret
S3_BUCKET=your-bucket-name
```

You may also create `.env.example` for sharing.

---

## 🛠 Tech Stack

* **Rust**
* **Axum** – web framework
* **Askama** – HTML templates
* **AWS SDK for Rust (S3)**
* **Tokio** – async runtime
* **HTML + CSS** (no JS framework)

---

## 🎯 Design Goals

* Keep the backend **simple and fast**
* Avoid heavy frontend frameworks
* Let AWS handle file delivery
* Focus on **trust, clarity, and user experience**

---

## 📌 Status

This project is actively evolving and designed to be extended with:

* Auth checks
* Expiring links
* File permissions
* Analytics / logging

---

## 🧑‍💻 Author

**Aeon Roamer**
Built with care, performance, and clean design in mind.

---

If you’re reading this README, you’re already looking at a project designed like a real production service — not a toy
demo.This Service is actually is in use.
