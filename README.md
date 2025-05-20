
# Simple JWT Authentication Server

This is a minimal Actix-Web server in Rust that demonstrates basic JSON Web Token (JWT) authentication. It exposes three endpoints:

- `/public-view`: Publicly accessible.
- `/get-token`: Issues a JWT for a given user.
- `/secret-view`: Requires a valid JWT in the `Authorization` header.

## 📦 Features

- ✅ Issue JWTs with 1-minute expiration.
- ✅ Validate JWTs via a custom request extractor (`Auth`).
- ✅ Simple public/private route structure.

---

## 📜 Endpoints

### 🔓 `GET /public-view`

Returns a public message. No authentication required.

**Request:**
```bash
curl http://localhost:2424/public-view
````

**Response:**

```json
{
  "success": true,
  "data": {
    "message": "This data is visible to all users"
  }
}
```

---

### 🔑 `POST /get-token`

Accepts a JSON body with a user email and returns a JWT.

**Request:**

```bash
curl -X POST http://localhost:2424/get-token \
  -H "Content-Type: application/json" \
  -d '{"email": "me@example.com"}'
```

**Response:**

```json
{
  "success": true,
  "data": {
    "token": "<JWT_TOKEN>"
  }
}
```

---

### 🔐 `GET /secret-view`

Protected endpoint. Requires a JWT in the `Authorization` header using the `Bearer` scheme.

**Request (replace `<JWT_TOKEN>`):**

```bash
curl http://localhost:2424/secret-view \
  -H "Authorization: Bearer <JWT_TOKEN>"
```

**Response:**

```json
{
  "success": true,
  "data": {
    "email": "me@example.com"
  }
}
```

**If missing or invalid token:**

```json
{
  "success": false,
  "data": {
    "message": "No token provided" // or "InvalidToken" etc.
  }
}
```

---

## 🛠 How it works

* JWTs are signed using a hardcoded secret (`"mykey"`) and expire after **1 minute**.
* When a client requests a protected route, a custom `Auth` extractor reads the `Authorization` header, decodes the JWT, and either:

  * Injects the decoded `User` into the handler.
  * Rejects with a `401 Unauthorized` response.

---

## 🚀 Run It

```bash
cargo run
```

Then hit the routes using `curl`, Postman, or your browser.

---

## 📁 Project Structure

```
src/
├── main.rs        # Starts Actix server, configures routes
├── handlers.rs    # Route handler logic
├── auth.rs        # Custom Auth extractor for JWT
├── lib.rs         # JWT encode/decode logic (User, get_jwt, decode_jwt)
```

---

## 🔐 Note

This is a demo and **not suitable for production**:

* 🔑 Secret is hardcoded
* 🔁 No token refresh or revocation
* ⏱ Short expiration for demonstration

---


