# कर्तृ (Kartṛ) / Kartru

A minimal, fast CLI coding agent written in Rust. **Kartṛ** (_the doer / agent_) autonomously plans, inspects, and executes tasks using an LLM-driven tool-use loop.

---

## ✨ Features

- **⚡ Lightweight & Fast:** Built with Rust and Tokio async runtime.
- **🛠️ Built-in Tools:**
  - `read_file` — inspect codebase and read file contents.
  - `write_file` — create or write files.
  - `bash` — execute shell commands.
- **🔌 MCP (Model Context Protocol) Client:** Dynamically discover and execute tools from external MCP servers via stdio transport (powered by `rmcp`).
- **🧠 OpenRouter Powered:** Defaults to `anthropic/claude-haiku-4.5` (via OpenRouter).
- **🔄 Autonomous Loop:** Multi-step tool invocation, fault-tolerant execution, and structured step-by-step logging with `tracing`.

---

## 🚀 Getting Started

### 1. Prerequisites

- [Rust](https://rustup.rs/) (Rust 2024 edition)
- [OpenRouter](https://openrouter.ai/) API key

### 2. Configuration

#### Environment Variables
Create a `.env` file in the project root:

```env
OPENROUTER_API_KEY=your_openrouter_api_key
# Optional:
# OPENROUTER_BASE_URL=https://openrouter.ai/api/v1
```

#### MCP Servers Configuration (Optional)
To connect external MCP tools, create a config file at `configs/mcp_servers.json` (or `src/configs/mcp_servers.json`):

```json
{
  "mcpServers": {
    "fetch": {
      "command": "uvx",
      "args": ["--quiet", "mcp-server-fetch"]
    }
  }
}
```

### 3. Usage

Run Kartṛ with the `-p` / `--prompt` flag:

```bash
cargo run -- -p "Create a python script that calculates fibonacci numbers"
```

You can control log verbosity using `RUST_LOG`:

```bash
RUST_LOG=info cargo run -- -p "What is the content of https://example.com"
```

---

## 🗺️ Roadmap

- [ ] Interactive REPL mode & multi-turn sessions
- [ ] Model switcher & custom provider endpoints
- [ ] Command permissions & execution safeguards
- [x] MCP (Model Context Protocol) client integration
- [ ] LSP integrations
- [ ] Slash commands & web search support

