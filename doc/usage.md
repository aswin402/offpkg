# Usage Guide

## Full CLI Reference

### Bun commands
```bash
offpkg bun install <pkg>       # download & cache from npm (needs internet)
offpkg bun add <pkg>           # add from cache to project (offline)
offpkg bun remove <pkg>        # remove from offpkg cache
```

### Python uv commands
```bash
offpkg uv install <pkg>        # download & cache from PyPI (needs internet)
offpkg uv add <pkg>            # add from cache to .venv (offline)
offpkg uv install-all          # cache all deps from pyproject.toml
offpkg uv remove <pkg>         # remove from offpkg cache
```

### Flutter commands
```bash
offpkg flutter install <pkg>   # download & cache from pub.dev (needs internet)
offpkg flutter add <pkg>       # add from cache to project (offline)
offpkg flutter install-all     # cache all deps from pubspec.yaml
offpkg flutter remove <pkg>    # remove from offpkg cache
```

### Stack commands
```bash
offpkg stack list                         # list all available stacks
offpkg stack show <name>                  # show what a stack contains
offpkg stack install <name>               # cache all stack packages (needs internet)
offpkg stack add <name>                   # add stack to current project (offline)
offpkg stack new <name> --runtime bun     # create custom stack template
```

### Docs commands
```bash
offpkg docs edit <pkg> --runtime bun      # open global doc in $EDITOR
offpkg docs show <pkg> --runtime bun      # print global doc to terminal
offpkg docs reset <pkg> --runtime bun     # regenerate from registry
offpkg docs list                          # list all packages with cached docs
offpkg docs list --runtime flutter        # filter by runtime
```

### Global commands
```bash
offpkg list                    # show all cached packages
offpkg list --runtime bun      # filter by runtime
offpkg doctor                  # environment health check
```

---

## Key Behaviors

**`install`** — needs internet, touches nothing in your project
- Downloads package to `~/.offpkg/cache/<runtime>/`
- Fetches README and saves to `~/.offpkg/docs/<runtime>/<pkg>.md`
- Records in `offpkg.db` with checksum

**`add`** — fully offline, modifies your project
- Reads from cache, verifies checksum
- Extracts/installs into project (node_modules / .venv / pub cache)
- Auto-updates `package.json` / `pyproject.toml` / `pubspec.yaml`
- Copies your edited doc to `offpkg_docs/offpkg_<pkg>.md`

**`remove`** — only touches offpkg cache, never your project
- Deletes `.tgz` / `.whl` / `.tar.gz` from cache
- Removes DB record
- Reports freed space

**`docs edit`** — edits the global master copy
- Opens `~/.offpkg/docs/<runtime>/<pkg>.md` in `$EDITOR`
- Every future `offpkg add` copies YOUR edited version to the project

---

## Configuration

```toml
# ~/.offpkg/config.toml

[cache]
path = "~/.offpkg/cache"
max_size_gb = 50.0

[network]
timeout_secs = 30
retries = 3

[runtimes]
bun = "auto"       # auto = detect from PATH
uv = "auto"
flutter = "auto"
```

Override cache location:
```bash
OFFPKG_CACHE_DIR=/external/drive offpkg bun install react
```

---

## Custom Stacks

Create a stack template:
```bash
offpkg stack new my-stack --runtime bun
# opens ~/.offpkg/stacks/my-stack.toml
```

Edit the TOML:
```toml
name = "my-stack"
runtime = "bun"
description = "My personal bun setup"

packages = ["hono", "zod", "dotenv"]
dev_packages = ["typescript", "@types/node"]

[[files]]
path = "src/index.ts"
content = """
import { Hono } from 'hono'
const app = new Hono()
app.get('/', (c) => c.json({ ok: true }))
export default app
"""

[[files]]
path = ".env"
content = "PORT=3000"
```

Then:
```bash
offpkg stack install my-stack   # cache packages
offpkg stack add my-stack       # use in project
```

---

## Troubleshooting

| Problem | Fix |
|---|---|
| `not in offpkg cache` | Run `offpkg <runtime> install <pkg>` first |
| `cache file missing` | Re-run install — file was deleted manually |
| `checksum mismatch` | Re-run install — cache file is corrupt |
| `uv add failed` | Run `offpkg uv install-all` to cache all project deps |
| `flutter pub get failed` | Run `offpkg flutter install-all` to cache all deps |
| `docs not showing` | Run `offpkg docs reset <pkg> --runtime <rt>` |
| DB corrupt | `rm ~/.offpkg/cache/offpkg.db` — auto-recreates on next run |
| No docs dir | Run `offpkg <runtime> install <pkg>` — docs fetched during install |