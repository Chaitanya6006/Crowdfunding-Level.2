# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:

```text
.
├── contracts
│   └── hello_world
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. I've created a premium Next.js frontend in the [frontend](file:///c:/Users/Admin/Desktop/crowdfunding-contract.2.fr/frontend) directory.
- You can run the frontend from this root directory using `npm run dev`.

## Frontend Setup

1.  **Install dependencies** (if not done): `npm install --prefix frontend`
2.  **Start development server**: `npm run dev`
3.  **Visit**: `http://localhost:3000`
