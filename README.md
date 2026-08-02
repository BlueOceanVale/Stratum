# Stratum

Stratum is a collaborative business operations platform built for startups and small businesses. It helps teams manage projects, employees, clients, and workflows through a single intuitive workspace, reducing the need for multiple disconnected tools.

## 1.0

Version 1.0 will deliver a modern, user-friendly platform designed to help businesses manage and streamline their operations. The platform will enable teams to collaborate effectively by managing employees, projects, deadlines, and basic client management from a unified dashboard.

This version focuses on reliability, performance, and intuitive user experience, the platform aims to integrate seamlessly with the tools and technologies businesses already use, helping organizations improve productivity, coordination, and growth.

## Stack

**Frontend** — Svelte 5, SvelteKit, TypeScript, Tailwind CSS, Bun

**Backend** — Rust, Axum, SQLx, PostgreSQL, JWT (jsonwebtoken), Argon2

## Features

- **Auth** — register, login, JWT-based sessions
- **Workspaces** — create and switch between workspaces
- **Members** — invite by user ID, assign roles (member/admin), remove access
- **Projects** — create, rename, delete
- **Tasks** — Kanban board per project (To Do / In Progress / Done), drag-and-drop status updates, priority levels
- **Comments** — per-task discussion thread
- **Clients** — track client contacts and status per workspace

## Getting Started

### Prerequisites

- [Bun](https://bun.sh)
- [Rust](https://www.rust-lang.org/tools/install) (stable toolchain)
- PostgreSQL running locally

### Backend

```bash
cd backend

# create backend/.env
cat <<EOF > .env
DATABASE_URL=postgres://<user>:<password>@localhost:5432/stratum_db
JWT_SECRET=<your-secret>
EOF

# create the database, then run migrations
sqlx migrate run

cargo run
```

The server starts on `http://localhost:3000`, with all routes nested under `/api`.

### Frontend

```bash
cd frontend

# create frontend/.env
echo "VITE_API_URL=http://localhost:3000/api" > .env

bun install
bun run dev
```

The app runs on `http://localhost:5173` by default. Register an account, create a workspace, and you're in.

## License

This project is under Apache License 2.0
See the [LICENSE](LICENSE) file for the full text.


Built by [Ocean](https://github.com/BlueOceanVale)