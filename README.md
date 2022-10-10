# Pattrick

Pattrick is a command line tool for managing Personal Access Tokens (PAT) in Azure DevOps.

It allows you to:

- 🐣 create
- 📖 list
- 🔎 show
- ⚰️ delete

PATs without having to go to the web interface.

## Installation

You can install Pattrick with [Homebrew](https://brew.sh/):

```bash
brew tap jvanbuel/pattrick
brew install pattrick
```

## Usage

Pattrick looks for Azure CLI credentials to fetch an access token for authentication with Azure DevOps. If you haven't logged in yet, you can do so with:

```bash
az login
```

## Usage as standalone library

You can also use Pattrick as a standalone library. This is useful if you want to manage PATS programmatically in your own codebase.
`

```rust
use pattrick::PatManager;
```
