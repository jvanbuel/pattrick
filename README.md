![1673695104691](image/README/1673695104691.png)

![crates](https://img.shields.io/crates/v/pattrick)
![build](https://img.shields.io/github/actions/workflow/status/jvanbuel/pattrick/test.yml)
![docs](https://img.shields.io/docsrs/pattrick)
![license](https://img.shields.io/crates/l/pattrick)

Pattrick is a command line tool for managing Personal Access Tokens (PAT) in Azure DevOps.

It allows you to:

- 🐣 create
- 📖 list
- 🔎 show
- ⚰️ delete

PATs without having to go to the web interface.

## Installation

On MacOs, you can install Pattrick with [Homebrew](https://brew.sh/):

```bash
brew tap jvanbuel/pattrick
brew install pattrick
```

On Linux, you can install Pattrick by executing the following commands:

```bash
curl -L https://github.com/jvanbuel/pattrick/releases/latest/download/pattrick-x86_64-unknown-linux-gnu.tar.gz | tar xvz
chmod +x pattrick
sudo mv pattrick /usr/local/bin/pattrick
```

## Usage

Pattrick looks for Azure CLI credentials to fetch an access token for authentication with Azure DevOps. You can get one locally by logging in to Azure with:

```bash
az login
```

If `pattrick` cannot find a valid access token, it will try to log you in automatically (by using the `az login` command under the hood). It will also try to automatically figure out the DevOps organization to connect to. If you have access to multiple DevOps organization, you can specify the organization you want to manage PAT tokens for via the environment variable `DEVOPS_ORGANIZATION`.

You can then start using `pattrick` to manage your PAT tokens:

```bash
pattrick create --lifetime 100 --scope packaging
```

By default, `pattrick` writes newly created token to stdout. However, you can also tell `pattrick` to write the token to your `.netrc` file (useful for e.g. installing Python packages from Azure DevOps Artifacts), or to a local `.env` file:

```bash
pattrick create --out std-out (default) / dot-netrc / dot-env
```

To get an overview of the other commands and options available, run:

```bash
pattrick --help
```

## Usage as standalone library

You can also use Pattrick as a standalone library. This is useful if you want to manage PATS programmatically in your own codebase.

```rust
use pattrick::{PatTokenManager, PatTokenListRequest, DisplayFilterOption};
use pattrick::azure::get_ad_token_for_devops;

let pat_manager = PatTokenManager::new(get_ad_token_for_devops(1).await?);

let pat_tokens = pat_manager.list_pat_tokens(
     PatTokenListRequest {
        display_filter_option: DisplayFilterOption::All
     }
 ).await?;
```

For more information, check out the `pattrick` documentation at [docs.rs](https://docs.rs/pattrick/latest/pattrick/)
