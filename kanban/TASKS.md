# Kanban Board – Getting Started Tasks

Stack: C# / ASP.NET Core backend, server-rendered HTML with htmx for frontend, Alpine.js for client-side interactivity, PostgreSQL for storage (run in Docker locally, and as a separate service when deployed).

## Phase 0 – Environment & Project Setup
- [x] Install the .NET SDK (latest LTS) and verify with `dotnet --version`
- [x] Install an IDE (Rider or VS Code with the C# Dev Kit)
- [ ] Install Docker Desktop (needed for the local Postgres container and for Testcontainers in tests)
- [x] Create a solution: `dotnet new sln -n Kanban`
- [x] Create the web project: `dotnet new web -n Kanban.Web` (or `webapi`/`mvc`), add it to the solution
- [x] Create a test project: `dotnet new xunit -n Kanban.Tests` and reference the web project
- [x] Add a `.gitignore` for .NET (`dotnet new gitignore`)
- [x] Add a `README.md` with run instructions
- [x] Confirm "Hello World" runs at `https://localhost:<port>` with `dotnet run`

## Phase 1 – Domain Model
- [ ] Define `Board`, `Column`, and `Card` (or `Task`) models with `Id`, `Title`, `Description`, `Position`, and timestamps
- [ ] Decide relationships: a Board has many Columns, a Column has many Cards
- [ ] Write unit tests for domain rules, e.g. a card can't move to a column that doesn't exist
- [ ] Decide default columns (To Do, In Progress, Done)

## Phase 2 – Persistence (PostgreSQL)
- [ ] Run PostgreSQL locally in Docker: `docker run --name kanban-pg -e POSTGRES_PASSWORD=<password> -p 5432:5432 -d postgres:17`
- [ ] Add EF Core with the PostgreSQL provider (`Npgsql.EntityFrameworkCore.PostgreSQL`)
- [ ] Store the connection string and database password in user secrets (`dotnet user-secrets`), not in `appsettings.json`
- [ ] Create `AppDbContext` and configure the entities, including an index on `Cards (ColumnId, Position)`
- [ ] Create the first migration (`dotnet ef migrations add InitialCreate`) and apply it to Postgres
- [ ] Turn on EF Core SQL logging and read the SQL generated for a card move, to see what Postgres is actually running
- [ ] Seed a default board with three columns on first run
- [ ] Add `Testcontainers.PostgreSql` to `Kanban.Tests` so tests start a throwaway Postgres container (requires Docker running)
- [ ] Write an integration test that proves a card round-trips through the database

## Phase 3 – REST API (JSON)
- [ ] `GET /api/boards/{id}` returns the board with its columns and cards
- [ ] `POST /api/cards` creates a card in a column
- [ ] `PUT /api/cards/{id}` updates title or description
- [ ] `POST /api/cards/{id}/move` moves a card to another column and position
- [ ] `DELETE /api/cards/{id}` deletes a card
- [ ] Validate input and return proper status codes (400, 404)
- [ ] Test the endpoints with `.http` files or Swagger (`Microsoft.AspNetCore.OpenApi`)
- [ ] Write integration tests with `WebApplicationFactory<Program>`, reusing the Postgres Testcontainers setup from Phase 2

## Phase 4 – Server-Rendered UI with htmx
- [ ] Add a base layout with htmx loaded from a CDN (`https://unpkg.com/htmx.org`)
- [ ] Render the board page using Razor views (or Razor components)
- [ ] Render a card partial that can be swapped in by the server
- [ ] Add "Add card" with `hx-post` that returns the new card HTML and swaps it into the column
- [ ] Add inline edit for a card title with `hx-get` / `hx-put`
- [ ] Add delete with `hx-delete` and `hx-confirm`
- [ ] Add a column header partial that shows the card count and updates after changes
- [ ] Add basic CSS for a column/card layout (flexbox or CSS grid)

## Phase 5 – Drag and Drop with Alpine.js
- [ ] Add Alpine.js (`https://unpkg.com/alpinejs`, defer)
- [ ] Use an Alpine component to track which card is being dragged
- [ ] Use HTML5 drag events to highlight drop targets
- [ ] On drop, send `hx-post` to `/cards/{id}/move` with the target column and position
- [ ] Refresh only the affected columns from the server response (out-of-band swaps with `hx-swap-oob`)
- [ ] Handle the error case where the move fails and the card snaps back

## Phase 6 – Polish & Interactivity
- [ ] Add a modal for editing a card, controlled with Alpine `x-data` / `x-show`
- [ ] Add keyboard shortcuts with Alpine (for example, `n` for new card)
- [ ] Add loading indicators with htmx's `htmx-indicator` class
- [ ] Show toast notifications for errors using `HX-Trigger` response headers
- [ ] Add form validation messages returned from the server

## Phase 7 – Real-Time (Stretch)
- [ ] Add SignalR or Server-Sent Events so other browser tabs update when a card moves
- [ ] Use htmx's SSE extension (`hx-ext="sse"`) to listen to the stream

## Phase 8 – Deployment & Quality
- [ ] Add a Dockerfile for the web app (`mcr.microsoft.com/dotnet/aspnet`)
- [ ] Add a `docker-compose.yml` with two services: the web app and `postgres` (a named volume for the data, with the connection string passed as an environment variable rather than committed)
- [ ] Verify the deployed stack by running `docker compose up` and checking that data survives `docker compose down` and `up`
- [ ] Add logging with Serilog or the built-in logger
- [ ] Add GitHub Actions to run `dotnet build` and `dotnet test` on push
- [ ] Add authentication (ASP.NET Core Identity or a simple cookie login) for stretch goals

## Suggested Order of Work
Phases 0 → 1 → 2 → 3 → 4 → 5 → 6. Docker must be running from Phase 2 onward (for the local database and the tests). Finish Phase 3 before starting Phase 4 so the UI has a working API to lean on, but the HTML endpoints can also be written directly against the database if you'd like a shorter path. Treat Phases 7 and 8 as optional stretch goals.
