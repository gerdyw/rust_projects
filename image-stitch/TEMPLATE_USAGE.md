# Using This Template

This is a reusable Rust web service template. When creating a new project, follow these steps:

## Quick Start

1. **Copy the template directory:**
   ```bash
   cp -r template-rust-web my-new-project
   cd my-new-project
   ```

2. **Rename all instances of the placeholder:**

   ### Option A: Using VS Code (Recommended)
   - Press `Cmd+Shift+H` (macOS) or `Ctrl+Shift+H` (Windows/Linux)
   - In "Find": `project-name`
   - In "Replace": `your-project-name`
   - Click "Replace All"
   - Repeat for:
     - `project_name` → `your_project_name` (for Rust identifiers)

   ### Option B: Using Command Line
   ```bash
   # On macOS/Linux with GNU sed or gsed
   find . -type f ! -path "*/target/*" ! -path "*/.git/*" ! -name "TEMPLATE_USAGE.md" -exec sed -i '' 's/project-name/your-project-name/g' {} +
   find . -type f ! -path "*/target/*" ! -path "*/.git/*" ! -name "TEMPLATE_USAGE.md" -exec sed -i '' 's/project_name/your_project_name/g' {} +
   ```

   ### Option C: Using Just (Built-in)
   ```bash
   just rename-project your-project-name
   ```

3. **Set up your environment:**
   ```bash
   cp .env.example .env
   # Edit .env with your database settings
   ```

4. **Initialize your project:**
   ```bash
   just db-up
   cargo check  # This will download dependencies and check compilation
   ```

5. **Customize your domain:**
   - Replace/rename `src/data/example/` with your actual domain models
   - Update migrations in `migrations/` for your schema
   - Add your routes in `src/web/router.rs`
   - Update `README.md` with your project description

6. **Test it works:**
   ```bash
   just run
   curl http://localhost:3000/health
   ```

## What Needs to Be Renamed

When using this template, the following placeholders need to be replaced:

| Placeholder | Example Replacement | Where It Appears |
|------------|---------------------|------------------|
| `project-name` | `my-api-service` | Cargo.toml, docker-compose.yml, Dockerfile, README |
| `project_name` | `my_api_service` | Rust code (imports, logs) |
| `exampledb` | `myservicedb` | .env.example, docker-compose.yml |

## Files That Need Updating

After renaming, customize these files for your project:

1. **README.md** - Update project description and features
2. **Cargo.toml** - Add any extra dependencies you need
3. **.env** - Set real database credentials
4. **src/data/example/** - Replace with your domain models
5. **migrations/** - Create your actual database schema
6. **src/web/router.rs** - Add your API routes
7. **src/web/handlers.rs** - Implement your endpoints

## Pro Tips

- Keep this template directory as a clean reference copy
- Update the template when you discover useful patterns
- Consider git-ignoring the `TEMPLATE_USAGE.md` in new projects
- Use `just --list` to see all available commands
