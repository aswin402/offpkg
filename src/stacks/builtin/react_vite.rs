use crate::stacks::{Stack, StackFile};

pub fn react_vite() -> Stack {
    Stack {
        name: "react-vite".into(),
        runtime: "bun".into(),
        description: "React 19 + Vite 8 + Tailwind 4 + Zustand + TanStack Query + React Router 7 (Complete Modern Template)".into(),
                packages: vec![
            "react".into(),
            "react-dom".into(),
            "react-router-dom".into(),
            "tailwindcss".into(),
            "lucide-react".into(),
        ],
        dev_packages: vec![
            "vite".into(),
            "@types/react".into(),
            "@types/react-dom".into(),
            "@vitejs/plugin-react".into(),
            "eslint".into(),
            "globals".into(),
            "typescript".into(),
        ],
        transitive_packages: vec![
            "@babel/code-frame".into(),
            "@babel/compat-data".into(),
            "@babel/core".into(),
            "@babel/generator".into(),
            "@babel/helper-compilation-targets".into(),
            "@babel/helper-globals".into(),
            "@babel/helper-module-imports".into(),
            "@babel/helper-module-transforms".into(),
            "@babel/helper-string-parser".into(),
            "@babel/helper-validator-identifier".into(),
            "@babel/helper-validator-option".into(),
            "@babel/helpers".into(),
            "@babel/parser".into(),
            "@babel/template".into(),
            "@babel/traverse".into(),
            "@babel/types".into(),
            "@eslint-community/eslint-utils".into(),
            "@eslint-community/regexpp".into(),
            "@eslint/config-array".into(),
            "@eslint/config-helpers".into(),
            "@eslint/core".into(),
            "@eslint/eslintrc".into(),
            "@eslint/js".into(),
            "@eslint/object-schema".into(),
            "@eslint/plugin-kit".into(),
            "@hookform/resolvers".into(),
            "@humanfs/core".into(),
            "@humanfs/node".into(),
            "@humanwhocodes/module-importer".into(),
            "@humanwhocodes/retry".into(),
            "@jridgewell/gen-mapping".into(),
            "@jridgewell/remapping".into(),
            "@jridgewell/resolve-uri".into(),
            "@jridgewell/sourcemap-codec".into(),
            "@jridgewell/trace-mapping".into(),
            "@oxc-project/types".into(),
            "@rolldown/binding-linux-x64-gnu".into(),
            "@rolldown/binding-linux-x64-musl".into(),
            "@rolldown/pluginutils".into(),
            "@standard-schema/utils".into(),
            "@tailwindcss/node".into(),
            "@tailwindcss/oxide".into(),
            "@tailwindcss/oxide-linux-x64-gnu".into(),
            "@tailwindcss/oxide-linux-x64-musl".into(),
            "@tailwindcss/vite".into(),
            "@tanstack/query-core".into(),
            "@tanstack/react-query".into(),
            "@types/estree".into(),
            "@types/json-schema".into(),
            "@types/node".into(),
            "@typescript-eslint/eslint-plugin".into(),
            "@typescript-eslint/parser".into(),
            "@typescript-eslint/project-service".into(),
            "@typescript-eslint/scope-manager".into(),
            "@typescript-eslint/tsconfig-utils".into(),
            "@typescript-eslint/type-utils".into(),
            "@typescript-eslint/types".into(),
            "@typescript-eslint/typescript-estree".into(),
            "@typescript-eslint/utils".into(),
            "@typescript-eslint/visitor-keys".into(),
            "acorn".into(),
            "acorn-jsx".into(),
            "ajv".into(),
            "ansi-styles".into(),
            "argparse".into(),
            "asynckit".into(),
            "axios".into(),
            "balanced-match".into(),
            "baseline-browser-mapping".into(),
            "brace-expansion".into(),
            "browserslist".into(),
            "call-bind-apply-helpers".into(),
            "callsites".into(),
            "caniuse-lite".into(),
            "chalk".into(),
            "color-convert".into(),
            "color-name".into(),
            "combined-stream".into(),
            "concat-map".into(),
            "convert-source-map".into(),
            "cookie".into(),
            "cross-spawn".into(),
            "csstype".into(),
            "debug".into(),
            "deep-is".into(),
            "delayed-stream".into(),
            "detect-libc".into(),
            "dunder-proto".into(),
            "electron-to-chromium".into(),
            "enhanced-resolve".into(),
            "es-define-property".into(),
            "es-errors".into(),
            "es-object-atoms".into(),
            "es-set-tostringtag".into(),
            "escalade".into(),
            "escape-string-regexp".into(),
            "eslint-plugin-react-hooks".into(),
            "eslint-plugin-react-refresh".into(),
            "eslint-scope".into(),
            "eslint-visitor-keys".into(),
            "espree".into(),
            "esquery".into(),
            "esrecurse".into(),
            "estraverse".into(),
            "esutils".into(),
            "fast-deep-equal".into(),
            "fast-json-stable-stringify".into(),
            "fast-levenshtein".into(),
            "fdir".into(),
            "file-entry-cache".into(),
            "find-up".into(),
            "flat-cache".into(),
            "flatted".into(),
            "follow-redirects".into(),
            "form-data".into(),
            "function-bind".into(),
            "gensync".into(),
            "get-intrinsic".into(),
            "get-proto".into(),
            "glob-parent".into(),
            "gopd".into(),
            "graceful-fs".into(),
            "has-flag".into(),
            "has-symbols".into(),
            "has-tostringtag".into(),
            "hasown".into(),
            "hermes-estree".into(),
            "hermes-parser".into(),
            "ignore".into(),
            "import-fresh".into(),
            "imurmurhash".into(),
            "is-extglob".into(),
            "is-glob".into(),
            "isexe".into(),
            "jiti".into(),
            "js-tokens".into(),
            "js-yaml".into(),
            "jsesc".into(),
            "json-buffer".into(),
            "json-schema-traverse".into(),
            "json-stable-stringify-without-jsonify".into(),
            "json5".into(),
            "keyv".into(),
            "levn".into(),
            "lightningcss".into(),
            "lightningcss-linux-x64-gnu".into(),
            "lightningcss-linux-x64-musl".into(),
            "locate-path".into(),
            "lodash.merge".into(),
            "lru-cache".into(),
            "magic-string".into(),
            "math-intrinsics".into(),
            "mime-db".into(),
            "mime-types".into(),
            "minimatch".into(),
            "ms".into(),
            "nanoid".into(),
            "natural-compare".into(),
            "node-releases".into(),
            "optionator".into(),
            "p-limit".into(),
            "p-locate".into(),
            "parent-module".into(),
            "path-exists".into(),
            "path-key".into(),
            "picocolors".into(),
            "picomatch".into(),
            "postcss".into(),
            "prelude-ls".into(),
            "proxy-from-env".into(),
            "punycode".into(),
            "react-hook-form".into(),
            "react-router".into(),
            "resolve-from".into(),
            "rolldown".into(),
            "scheduler".into(),
            "semver".into(),
            "set-cookie-parser".into(),
            "shebang-command".into(),
            "shebang-regex".into(),
            "source-map-js".into(),
            "strip-json-comments".into(),
            "supports-color".into(),
            "tapable".into(),
            "tinyglobby".into(),
            "ts-api-utils".into(),
            "type-check".into(),
            "typescript-eslint".into(),
            "undici-types".into(),
            "update-browserslist-db".into(),
            "uri-js".into(),
            "which".into(),
            "word-wrap".into(),
            "yallist".into(),
            "yocto-queue".into(),
            "zod".into(),
            "zod-validation-error".into(),
            "zustand".into(),
        ],
        files: vec![
            StackFile {
                path: "README.md".into(),
                content: r###"# <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/doc/logo.svg" alt="offpkg Logo" width="36" height="36" align="center"/> Offpkg Vite+React Template 🚀

A premium, highly-opinionated Vite + React starter template designed for scalability, type-safety, and modern developer experience.

## ✨ Features

- **Next.js-like Architecture**: Structured layouts, pages, and routing.
- **Tailwind CSS v4**: Modern styling with CSS variables and OKLCH color spaces.
- **Zustand State Management**: Persistent global stores for theme and application state.
- **Type-Safe API & Validation**: Axios integration with Zod schemas and React Query (TanStack).
- **Premium UI Components**: Custom-built, accessible components inspired by Shadcn UI.
- **Enhanced Logging**: Structured, group-collapsed console output for a cleaner dev experience.

---

## 🛠️ Tech Stack

- **Framework**: [React 19](https://react.dev/)
- **Bundler**: [Vite 8](https://vite.dev/)
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/)
- **Router**: [React Router 7](https://reactrouter.com/)
- **State**: [Zustand](https://docs.pmnd.rs/zustand)
- **Data Fetching**: [TanStack Query v5](https://tanstack.com/query)
- **Validation**: [Zod](https://zod.dev/)
- **Forms**: [React Hook Form](https://react-hook-form.com/)
- **Icons**: [Lucide React](https://lucide.dev/)

---

## 🚀 Getting Started

### 1. Installation

```bash
bun install
```

### 2. Development

```bash
bun run dev
```

### 3. Build

```bash
bun run build
```

---

## 🎨 Theme Setup & Modification

### Theme Store
The theme state is managed by Zustand in `src/store/useThemeStore.ts`. It supports `light`, `dark`, and `system` modes with automatic persistence to `localStorage`.

### Theme Provider
Wrap your application (or specific sections) with `<ThemeProvider />` from `src/components/ThemeProvider.tsx`.

### Customizing Colors
Modify the CSS variables in `src/index.css` within the `@theme` block. We use OKLCH for better color perception.

```css
@theme {
  --color-primary: oklch(0.59 0.201 273.444);
  --color-background: oklch(1 0 0);
  /* ... */
}
```

---

## 📦 Package Usage Guides

### 🌐 API (Axios + React Query)
API calls are centralized in `src/api/axios.ts`. Use React Query for data fetching:

```tsx
const { data, isLoading } = useQuery({
  queryKey: ['users'],
  queryFn: () => axiosInstance.get('/users').then(res => res.data),
});
```

### 🛡️ Validation (Zod)
Define your data shapes in `src/types/schema.ts`:

```typescript
const UserSchema = z.object({
  id: z.string(),
  name: z.string(),
});
```

### 📝 Forms (React Hook Form)
Integrated with `@hookform/resolvers` for Zod support:

```tsx
const form = useForm({
  resolver: zodResolver(UserSchema),
});
```

### 📦 State (Zustand)
Create stores in `src/store/`:

```typescript
export const useAuthStore = create((set) => ({
  user: null,
  login: (user) => set({ user }),
}));
```

---

## 📂 Project Structure

```text
src/
├── api/          # Axios instance and API calls
├── assets/       # Static assets (images, svgs)
├── components/   # Reusable UI components
├── hooks/        # Custom React hooks
├── layouts/      # Page layouts (e.g., RootLayout)
├── lib/          # Utilities (logger, etc.)
├── pages/        # Route-level components
├── providers/    # Context/Query providers
├── store/        # Zustand stores
└── types/        # Zod schemas and TS types
```

---

## 📜 License
MIT
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "tsconfig.json".into(),
                content: r###"{
  "files": [],
  "references": [
    { "path": "./tsconfig.app.json" },
    { "path": "./tsconfig.node.json" }
  ]
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "index.html".into(),
                content: r###"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>offpkg vite+react</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "package.json".into(),
                content: r###"{
  "name": "offpkg-vite-react",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc -b && vite build",
    "lint": "eslint .",
    "preview": "vite preview"
  },
  "dependencies": {
    "@hookform/resolvers": "^5.2.2",
    "@tailwindcss/vite": "^4.2.2",
    "@tanstack/react-query": "^5.95.2",
    "axios": "^1.13.6",
    "lucide-react": "^1.6.0",
    "react": "^19.2.4",
    "react-dom": "^19.2.4",
    "react-hook-form": "^7.72.0",
    "react-router-dom": "^7.13.2",
    "tailwindcss": "^4.2.2",
    "zod": "^4.3.6",
    "zustand": "^5.0.12"
  },
  "devDependencies": {
    "@eslint/js": "^9.39.4",
    "@types/node": "^24.12.0",
    "@types/react": "^19.2.14",
    "@types/react-dom": "^19.2.3",
    "@vitejs/plugin-react": "^6.0.1",
    "eslint": "^9.39.4",
    "eslint-plugin-react-hooks": "^7.0.1",
    "eslint-plugin-react-refresh": "^0.5.2",
    "globals": "^17.4.0",
    "typescript": "~5.9.3",
    "typescript-eslint": "^8.57.0",
    "vite": "^8.0.1"
  }
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: ".gitignore".into(),
                content: r###"# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
lerna-debug.log*

node_modules
dist
dist-ssr
*.local

# Editor directories and files
.vscode/*
!.vscode/extensions.json
.idea
.DS_Store
*.suo
*.ntvs*
*.njsproj
*.sln
*.sw?
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "tsconfig.app.json".into(),
                content: r###"{
  "compilerOptions": {
    "tsBuildInfoFile": "./node_modules/.tmp/tsconfig.app.tsbuildinfo",
    "target": "ES2023",
    "useDefineForClassFields": true,
    "lib": ["ES2023", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "types": ["vite/client"],
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "verbatimModuleSyntax": true,
    "moduleDetection": "force",
    "noEmit": true,
    "jsx": "react-jsx",

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "erasableSyntaxOnly": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedSideEffectImports": true,
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src"]
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "vite.config.ts".into(),
                content: r###"import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import path from 'path'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    tailwindcss(),
  ],
  server: {
    hmr: {
      overlay: true,
    },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
})
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "eslint.config.js".into(),
                content: r###"import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import tseslint from 'typescript-eslint'
import { defineConfig, globalIgnores } from 'eslint/config'

export default defineConfig([
  globalIgnores(['dist']),
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      js.configs.recommended,
      tseslint.configs.recommended,
      reactHooks.configs.flat.recommended,
      reactRefresh.configs.vite,
    ],
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
  },
])
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "tsconfig.node.json".into(),
                content: r###"{
  "compilerOptions": {
    "tsBuildInfoFile": "./node_modules/.tmp/tsconfig.node.tsbuildinfo",
    "target": "ES2023",
    "lib": ["ES2023"],
    "module": "ESNext",
    "types": ["node"],
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "verbatimModuleSyntax": true,
    "moduleDetection": "force",
    "noEmit": true,

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "erasableSyntaxOnly": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedSideEffectImports": true
  },
  "include": ["vite.config.ts"]
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "public/favicon.svg".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/favicon.svg").to_vec()),
            },
            StackFile {
                path: "public/icons.svg".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/icons.svg").to_vec()),
            },
            StackFile {
                path: "src/index.css".into(),
                content: r###"@import url('https://fonts.googleapis.com/css2?family=Manrope:wght@400;500;600;700&family=Geist:wght@400;500;600;700&display=swap');
@import "tailwindcss";

@theme {
  --color-background: oklch(var(--background));
  --color-foreground: oklch(var(--foreground));

  --color-primary: oklch(var(--primary));
  --color-primary-foreground: oklch(var(--primary-foreground));

  --color-secondary: oklch(var(--secondary));
  --color-secondary-foreground: oklch(var(--secondary-foreground));

  --color-muted: oklch(var(--muted));
  --color-muted-foreground: oklch(var(--muted-foreground));

  --color-accent: oklch(var(--accent));
  --color-accent-foreground: oklch(var(--accent-foreground));

  --color-destructive: oklch(var(--destructive));
  --color-destructive-foreground: oklch(var(--destructive-foreground));

  --color-border: oklch(var(--border));
  --color-input: oklch(var(--input));
  --color-ring: oklch(var(--ring));

  --radius-lg: 0.5rem;
  --radius-md: calc(0.5rem - 2px);
  --radius-sm: calc(0.5rem - 4px);

  --font-heading: 'Manrope', sans-serif;
  --font-body: 'Geist', sans-serif;
}

@layer base {
  :root {
    --background: 1 0 0;
    --foreground: 0.141 0.005 285.823;
    --primary: 0.59 0.201 273.444;
    --primary-foreground: 1 0 0;
    --secondary: 0.949 0.029 303.081;
    --secondary-foreground: 0.21 0.006 285.885;
    --muted: 0.963 0.023 308.198;
    --muted-foreground: 0.472 0.002 286.339;
    --accent: 0.949 0.029 303.081;
    --accent-foreground: 0.211 0.006 285.885;
    --destructive: 0.637 0.208 25.331;
    --destructive-foreground: 0.985 0 0;
    --border: 0.92 0.02 285; /* Adjusted for better visibility */
    --input: 0.92 0.02 285;
    --ring: 0.59 0.201 273.444;
    --radius: 0.5rem;
  }

  .dark {
    --background: 0.141 0.005 285.823;
    --foreground: 0.985 0 0;
    --primary: 0.665 0.179 278.961;
    --primary-foreground: 1 0 0;
    --secondary: 0.202 0.107 263.462;
    --secondary-foreground: 0.985 0 0;
    --muted: 0.167 0.112 264.144;
    --muted-foreground: 0.673 0 0;
    --accent: 0.202 0.107 263.462;
    --accent-foreground: 0.985 0 0;
    --destructive: 0.396 0.133 25.723;
    --destructive-foreground: 0.985 0 0;
    --border: 0.25 0.05 264; /* Adjusted for dark mode */
    --input: 0.25 0.05 264;
    --ring: 0.665 0.179 278.961;
  }
}

@layer base {
  * {
    @apply border-border;
  }
  body {
    @apply bg-background text-foreground font-body;
  }
  h1, h2, h3, h4, h5, h6 {
    @apply font-heading font-bold;
  }
}

"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/App.css".into(),
                content: r###".counter {
  font-size: 16px;
  padding: 5px 10px;
  border-radius: 5px;
  color: var(--accent);
  background: var(--accent-bg);
  border: 2px solid transparent;
  transition: border-color 0.3s;
  margin-bottom: 24px;

  &:hover {
    border-color: var(--accent-border);
  }
  &:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
}

.hero {
  position: relative;

  .base,
  .framework,
  .vite {
    inset-inline: 0;
    margin: 0 auto;
  }

  .base {
    width: 170px;
    position: relative;
    z-index: 0;
  }

  .framework,
  .vite {
    position: absolute;
  }

  .framework {
    z-index: 1;
    top: 34px;
    height: 28px;
    transform: perspective(2000px) rotateZ(300deg) rotateX(44deg) rotateY(39deg)
      scale(1.4);
  }

  .vite {
    z-index: 0;
    top: 107px;
    height: 26px;
    width: auto;
    transform: perspective(2000px) rotateZ(300deg) rotateX(40deg) rotateY(39deg)
      scale(0.8);
  }
}

#center {
  display: flex;
  flex-direction: column;
  gap: 25px;
  place-content: center;
  place-items: center;
  flex-grow: 1;

  @media (max-width: 1024px) {
    padding: 32px 20px 24px;
    gap: 18px;
  }
}

#next-steps {
  display: flex;
  border-top: 1px solid var(--border);
  text-align: left;

  & > div {
    flex: 1 1 0;
    padding: 32px;
    @media (max-width: 1024px) {
      padding: 24px 20px;
    }
  }

  .icon {
    margin-bottom: 16px;
    width: 22px;
    height: 22px;
  }

  @media (max-width: 1024px) {
    flex-direction: column;
    text-align: center;
  }
}

#docs {
  border-right: 1px solid var(--border);

  @media (max-width: 1024px) {
    border-right: none;
    border-bottom: 1px solid var(--border);
  }
}

#next-steps ul {
  list-style: none;
  padding: 0;
  display: flex;
  gap: 8px;
  margin: 32px 0 0;

  .logo {
    height: 18px;
  }

  a {
    color: var(--text-h);
    font-size: 16px;
    border-radius: 6px;
    background: var(--social-bg);
    display: flex;
    padding: 6px 12px;
    align-items: center;
    gap: 8px;
    text-decoration: none;
    transition: box-shadow 0.3s;

    &:hover {
      box-shadow: var(--shadow);
    }
    .button-icon {
      height: 18px;
      width: 18px;
    }
  }

  @media (max-width: 1024px) {
    margin-top: 20px;
    flex-wrap: wrap;
    justify-content: center;

    li {
      flex: 1 1 calc(50% - 8px);
    }

    a {
      width: 100%;
      justify-content: center;
      box-sizing: border-box;
    }
  }
}

#spacer {
  height: 88px;
  border-top: 1px solid var(--border);
  @media (max-width: 1024px) {
    height: 48px;
  }
}

.ticks {
  position: relative;
  width: 100%;

  &::before,
  &::after {
    content: '';
    position: absolute;
    top: -4.5px;
    border: 5px solid transparent;
  }

  &::before {
    left: 0;
    border-left-color: var(--border);
  }
  &::after {
    right: 0;
    border-right-color: var(--border);
  }
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/main.tsx".into(),
                content: r###"import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { QueryProvider } from '@/providers/QueryProvider.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <QueryProvider>
      <App />
    </QueryProvider>
  </StrictMode>,
)
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/App.tsx".into(),
                content: r###"import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import { RootLayout } from '@/layouts/RootLayout';
import { HomePage } from '@/pages/HomePage';
import { NotFoundPage } from '@/pages/NotFoundPage';
import './App.css';

const router = createBrowserRouter([
  {
    path: '/',
    element: <RootLayout />,
    children: [
      {
        index: true,
        element: <HomePage />,
      },
      // Add other routes here, e.g.:
      // { path: 'about', element: <AboutPage /> },
      {
        path: '*',
        element: <NotFoundPage />,
      },
    ],
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;



"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/assets/vite.svg".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/vite.svg").to_vec()),
            },
            StackFile {
                path: "src/assets/react.svg".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/react.svg").to_vec()),
            },
            StackFile {
                path: "src/assets/hero.png".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/hero.png").to_vec()),
            },
            StackFile {
                path: "src/providers/QueryProvider.tsx".into(),
                content: r###"import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 5, // 5 minutes
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

export function QueryProvider({ children }: { children: React.ReactNode }) {
  return (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/ThemeProvider.tsx".into(),
                content: r###"import React, { useEffect } from 'react';
import { useThemeStore } from '../store/useThemeStore';

interface ThemeProviderProps {
  children: React.ReactNode;
  inlineTheme?: Record<string, string>;
}

export function ThemeProvider({
  children,
  inlineTheme,
}: ThemeProviderProps) {
  const { theme } = useThemeStore();

  useEffect(() => {
    const root = window.document.documentElement;

    root.classList.remove('light', 'dark');

    if (theme === 'system') {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light';

      root.classList.add(systemTheme);
      return;
    }

    root.classList.add(theme);
  }, [theme]);

  return (
    <div style={inlineTheme as React.CSSProperties}>
      {children}
    </div>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/ThemeToggleButton.tsx".into(),
                content: r###"import { Moon, Sun} from 'lucide-react';
import { useThemeStore } from '../store/useThemeStore';
import Button from './button';


export function ThemeToggleButton() {
  const { theme, setTheme } = useThemeStore();

  const cycleTheme = () => {
    if (theme === 'light') setTheme('dark');
    else setTheme('light');
  };

  return (
    <Button
      variant="outline"
      size="icon"
      onClick={cycleTheme}
      title={`Current: ${theme} • Click to cycle`}
      className="fixed top-4 right-4 h-10 w-10 rounded-full"
    >
      <Sun
        className={`h-[1.2rem] w-[1.2rem] transition-all ${
          theme === 'light'
            ? 'rotate-0 scale-100'
            : 'rotate-90 scale-0'
        }`}
      />
      <Moon
        className={`absolute h-[1.2rem] w-[1.2rem] transition-all ${
          theme === 'dark'
            ? 'rotate-0 scale-100'
            : 'rotate-90 scale-0'
        }`}
      />
      <span className="sr-only">Toggle theme</span>
    </Button>
  );
}"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/button.tsx".into(),
                content: r###"import { forwardRef, type ButtonHTMLAttributes } from 'react';

type ButtonVariant = 'primary' | 'secondary' | 'outline' | 'ghost';
type ButtonSize = 'default' | 'sm' | 'lg' | 'icon';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
}

const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ 
    children, 
    variant = 'primary', 
    size = 'default', 
    className = '', 
    ...props 
  }, ref) => {
    
    const baseClasses = "inline-flex items-center justify-center font-medium font-body rounded-button transition-all active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-primary";

    const variantClasses = {
      primary: "bg-primary text-primary-foreground shadow-sm hover:shadow-md hover:bg-primary/90",
      secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
      outline: "border border-border bg-transparent hover:bg-muted hover:text-foreground",
      ghost: "hover:bg-muted hover:text-foreground",
    };

    const sizeClasses = {
      default: "px-4 py-2 text-sm",
      sm: "px-3 py-1.5 text-xs",
      lg: "px-6 py-3 text-base",
      icon: "h-10 w-10 p-0",
    };

    return (
      <button
        ref={ref}
        className={`${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${className}`}
        {...props}
      >
        {children}
      </button>
    );
  }
);

Button.displayName = "Button";

export default Button;"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/Navbar.tsx".into(),
                content: r###"import { Link } from 'react-router-dom';
import { ThemeToggleButton } from '@/components/ThemeToggleButton';
import { Home, Info, Mail } from 'lucide-react';

export function Navbar() {
  return (
    <nav className="fixed top-0 left-0 right-0 h-16 border-b border-border/40 bg-background/80 backdrop-blur-md z-50 flex items-center justify-between px-6">
      <div className="flex items-center gap-8">
        <Link to="/" className="text-xl font-heading font-bold tracking-tight text-primary transition-opacity hover:opacity-80 flex items-center gap-2">
          <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/doc/logo.svg" alt="Offpkg Logo" className="w-6 h-6" />
          <span>OFFPKG</span>
        </Link>
        <div className="hidden md:flex items-center gap-6">
          <Link to="/" className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors flex items-center gap-2">
            <Home className="w-4 h-4" /> Home
          </Link>
          <Link to="/about" className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors flex items-center gap-2">
            <Info className="w-4 h-4" /> About
          </Link>
          <Link to="/contact" className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors flex items-center gap-2">
            <Mail className="w-4 h-4" /> Contact
          </Link>
        </div>
      </div>
      <ThemeToggleButton />
    </nav>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/lib/logger.ts".into(),
                content: r###"type LogLevel = "info" | "warn" | "error" | "debug"

const isDev = import.meta.env.DEV

const styles: Record<LogLevel, string> = {
  info: "color: #3b82f6; font-weight: 600;",
  warn: "color: #f59e0b; font-weight: 600;",
  error: "color: #ef4444; font-weight: 600;",
  debug: "color: #10b981; font-weight: 600;",
}

const formatMessage = (level: LogLevel, message: string) => {
  const prefix = `[WEB] ${level.toUpperCase()}`
  return [`%c${prefix} %c${message}`, styles[level], "color: inherit; font-weight: normal;"]
}

function log(level: LogLevel, message: string, data?: unknown) {
  if (!isDev && level === "debug") return

  const [prompt, style, reset] = formatMessage(level, message)

  if (data === undefined) {
    if (level === "error") console.error(prompt, style, reset)
    else if (level === "warn") console.warn(prompt, style, reset)
    else console.log(prompt, style, reset)
    return
  }

  // Handle data with grouping for a cleaner console
  console.groupCollapsed(prompt, style, reset)
  
  if (data instanceof Error) {
    console.error(data.message)
    if (data.stack) console.debug(data.stack)
  } else {
    console.dir(data)
  }
  
  console.groupEnd()
}

export const logger = {
  info: (msg: string, data?: unknown) => log("info", msg, data),
  warn: (msg: string, data?: unknown) => log("warn", msg, data),
  error: (msg: string, data?: unknown) => log("error", msg, data),
  debug: (msg: string, data?: unknown) => log("debug", msg, data),
}"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/pages/NotFoundPage.tsx".into(),
                content: r###"import { Link } from 'react-router-dom';
import { MoveLeft } from 'lucide-react';

export function NotFoundPage() {
  return (
    <div className="min-h-[calc(100vh-4rem)] flex flex-col items-center justify-center p-8 text-center animate-in fade-in duration-700">
      <div className="relative mb-8">
        <h1 className="text-[12rem] font-black leading-none tracking-tighter text-muted-foreground/10 select-none">
          404
        </h1>
        <div className="absolute inset-0 flex items-center justify-center">
          <p className="text-4xl font-heading font-black tracking-tight">PAGE NOT FOUND</p>
        </div>
      </div>
      
      <p className="text-xl text-muted-foreground mb-12 max-w-md mx-auto leading-relaxed">
        The page you are looking for doesn't exist or has been moved to another universe.
      </p>

      <Link
        to="/"
        className="flex items-center gap-2 bg-primary text-primary-foreground px-8 py-4 rounded-2xl font-bold transition-all hover:gap-4 hover:pr-10 hover:shadow-xl active:scale-95"
      >
        <MoveLeft className="w-5 h-5" /> Back to Home
      </Link>
    </div>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/pages/HomePage.tsx".into(),
                content: r###"import { useState } from 'react';
import reactLogo from '../assets/react.svg';
import viteLogo from '../assets/vite.svg';
import heroImg from '../assets/hero.png';
import { Zap, ShieldCheck, Package } from 'lucide-react';

export function HomePage() {
  const [count, setCount] = useState(0);

  return (
    <div className="animate-in fade-in slide-in-from-bottom-4 duration-1000">
      <section id="center" className="min-h-[calc(100vh-4rem)] flex flex-col items-center justify-center p-8">
        <div className="hero relative mb-12">
          <div className="absolute -inset-4 bg-primary/20 blur-3xl rounded-full animate-pulse" />
          <img src={heroImg} className="relative base w-48 h-auto drop-shadow-2xl" alt="" />
          <img src={reactLogo} className="framework absolute -top-6 -right-6 w-14 h-14 animate-spin-slow" alt="React logo" />
          <img src={viteLogo} className="vite absolute -bottom-6 -left-6 w-14 h-14" alt="Vite logo" />
        </div>

        <div className="text-center mb-12 max-w-2xl">
          <h1 className="text-6xl font-heading mb-6 tracking-tighter leading-tight bg-gradient-to-r from-foreground to-foreground/70 bg-clip-text text-transparent">
            Get started Offpkg <br /> Vite+React
          </h1>
          <p className="text-muted-foreground text-xl leading-relaxed">
            The ultimate developer setup with
            <span className="text-primary font-semibold"> Tailwind v4, Zustand, Zod, </span> and
            <span className="text-primary font-semibold"> React Query</span>.
          </p>
        </div>

        <div className="flex flex-col items-center gap-4">
          <button
            className="group relative bg-primary text-primary-foreground px-8 py-4 rounded-2xl font-semibold text-lg transition-all hover:scale-105 active:scale-95 shadow-lg shadow-primary/20"
            onClick={() => setCount((c) => c + 1)}
          >
            Count is {count}
            <div className="absolute inset-0 rounded-2xl ring-1 ring-white/20 group-hover:ring-white/40 transition-all" />
          </button>
          <p className="text-sm text-muted-foreground">
            Edit <code className="bg-muted px-1.5 py-0.5 rounded font-mono">src/pages/HomePage.tsx</code> to test HMR
          </p>
        </div>
      </section>

      <div className="h-px bg-gradient-to-r from-transparent via-border to-transparent w-full" />

      <section id="features" className="p-16 max-w-6xl mx-auto grid md:grid-cols-3 gap-8">
        <FeatureCard
          title="Fast Refresh"
          desc="Lightning fast HMR provided by Vite 8 for an ultra-smooth dev experience."
          icon={<Zap className="w-8 h-8 text-yellow-500" />}
        />
        <FeatureCard
          title="Type Safe"
          desc="Zod and TypeScript integration ensures your data is always valid."
          icon={<ShieldCheck className="w-8 h-8 text-blue-500" />}
        />
        <FeatureCard
          title="State Master"
          desc="Global state management simplified with Zustand stores."
          icon={<Package className="w-8 h-8 text-purple-500" />}
        />
      </section>
    </div>
  );
}

function FeatureCard({ title, desc, icon }: { title: string; desc: string; icon: React.ReactNode }) {
  return (
    <div className="p-8 rounded-3xl border bg-card/50 backdrop-blur-sm text-card-foreground hover:border-primary/50 transition-colors group cursor-default">
      <div className="mb-4 group-hover:scale-110 transition-transform">{icon}</div>
      <h3 className="text-2xl font-bold mb-3">{title}</h3>
      <p className="text-muted-foreground leading-relaxed">{desc}</p>
    </div>
  );
}

"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/api/axios.ts".into(),
                content: r###"import axios from 'axios';
import { logger } from '../lib/logger';

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL || 'https://api.example.com',
  headers: {
    'Content-Type': 'application/json',
  },
});

api.interceptors.request.use(
  (config) => {
    logger.info(`Request: ${config.method?.toUpperCase()} ${config.url}`);
    return config;
  },
  (error) => {
    logger.error('Request Error', error);
    return Promise.reject(error);
  }
);

api.interceptors.response.use(
  (response) => {
    logger.info(`Response: ${response.status} ${response.config.url}`);
    return response;
  },
  (error) => {
    logger.error('Response Error', error.response?.data || error.message);
    return Promise.reject(error);
  }
);

export default api;
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/store/useAppStore.ts".into(),
                content: r###"import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface AppState {
  user: { name: string; email: string } | null;
  setUser: (user: { name: string; email: string } | null) => void;
  isLoading: boolean;
  setIsLoading: (loading: boolean) => void;
}

export const useAppStore = create<AppState>()(
  persist(
    (set) => ({
      user: null,
      setUser: (user) => set({ user }),
      isLoading: false,
      setIsLoading: (isLoading) => set({ isLoading }),
    }),
    {
      name: 'app-storage',
    }
  )
);
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/store/useThemeStore.ts".into(),
                content: r###"import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export type Theme = 'light' | 'dark' | 'system';

interface ThemeState {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

export const useThemeStore = create<ThemeState>()(
  persist(
    (set) => ({
      theme: 'system',
      setTheme: (theme) => set({ theme }),
    }),
    {
      name: 'theme-storage',
    }
  )
);
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/hooks/useUser.ts".into(),
                content: r###"import { useQuery } from '@tanstack/react-query';
import api from '../api/axios';
import { UserSchema } from '../types/schema';

export const useUser = (userId: string) => {
  return useQuery({
    queryKey: ['user', userId],
    queryFn: async () => {
      const { data } = await api.get(`/users/${userId}`);
      return UserSchema.parse(data);
    },
    enabled: !!userId,
  });
};
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/types/schema.ts".into(),
                content: r###"import { z } from 'zod';

export const UserSchema = z.object({
  id: z.string(),
  name: z.string().min(2, 'Name must be at least 2 characters'),
  email: z.string().email('Invalid email address'),
  role: z.enum(['admin', 'user', 'guest']),
});

export type User = z.infer<typeof UserSchema>;

export const LoginFormSchema = z.object({
  email: z.string().email(),
  password: z.string().min(6, 'Password must be at least 6 characters'),
});

export type LoginFormValues = z.infer<typeof LoginFormSchema>;
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/layouts/RootLayout.tsx".into(),
                content: r###"import { Outlet } from 'react-router-dom';
import { Navbar } from '@/components/Navbar';
import { ThemeProvider } from '@/components/ThemeProvider';

export function RootLayout() {
  return (
    <ThemeProvider>
      <div className="min-h-screen bg-background text-foreground transition-colors duration-300">
        <Navbar />
        <main className="pt-16">
          <Outlet />
        </main>
      </div>
    </ThemeProvider>
  );
}
"###.into(),
                binary_content: None,
            }
        ],
    }
}

pub fn react_vite_full() -> Stack {
    Stack {
        name: "react-vite-full".into(),
        runtime: "bun".into(),
        description: "React 19 + Vite 8 + Tailwind 4 + Zustand + TanStack Query + React Router 7 (Complete Modern Template)".into(),
                packages: vec![
            "@hookform/resolvers".into(),
            "@tailwindcss/vite".into(),
            "@tanstack/react-query".into(),
            "axios".into(),
            "lucide-react".into(),
            "react".into(),
            "react-dom".into(),
            "react-hook-form".into(),
            "react-router-dom".into(),
            "tailwindcss".into(),
            "zod".into(),
            "zustand".into(),
        ],
        dev_packages: vec![
            "@eslint/js".into(),
            "@types/node".into(),
            "@types/react".into(),
            "@types/react-dom".into(),
            "@vitejs/plugin-react".into(),
            "eslint".into(),
            "eslint-plugin-react-hooks".into(),
            "eslint-plugin-react-refresh".into(),
            "globals".into(),
            "typescript".into(),
            "typescript-eslint".into(),
            "vite".into(),
        ],
        transitive_packages: vec![
            "@babel/code-frame".into(),
            "@babel/compat-data".into(),
            "@babel/core".into(),
            "@babel/generator".into(),
            "@babel/helper-compilation-targets".into(),
            "@babel/helper-globals".into(),
            "@babel/helper-module-imports".into(),
            "@babel/helper-module-transforms".into(),
            "@babel/helper-string-parser".into(),
            "@babel/helper-validator-identifier".into(),
            "@babel/helper-validator-option".into(),
            "@babel/helpers".into(),
            "@babel/parser".into(),
            "@babel/template".into(),
            "@babel/traverse".into(),
            "@babel/types".into(),
            "@eslint-community/eslint-utils".into(),
            "@eslint-community/regexpp".into(),
            "@eslint/config-array".into(),
            "@eslint/config-helpers".into(),
            "@eslint/core".into(),
            "@eslint/eslintrc".into(),
            "@eslint/object-schema".into(),
            "@eslint/plugin-kit".into(),
            "@humanfs/core".into(),
            "@humanfs/node".into(),
            "@humanwhocodes/module-importer".into(),
            "@humanwhocodes/retry".into(),
            "@jridgewell/gen-mapping".into(),
            "@jridgewell/remapping".into(),
            "@jridgewell/resolve-uri".into(),
            "@jridgewell/sourcemap-codec".into(),
            "@jridgewell/trace-mapping".into(),
            "@oxc-project/types".into(),
            "@rolldown/binding-linux-x64-gnu".into(),
            "@rolldown/binding-linux-x64-musl".into(),
            "@rolldown/pluginutils".into(),
            "@standard-schema/utils".into(),
            "@tailwindcss/node".into(),
            "@tailwindcss/oxide".into(),
            "@tailwindcss/oxide-linux-x64-gnu".into(),
            "@tailwindcss/oxide-linux-x64-musl".into(),
            "@tanstack/query-core".into(),
            "@types/estree".into(),
            "@types/json-schema".into(),
            "@typescript-eslint/eslint-plugin".into(),
            "@typescript-eslint/parser".into(),
            "@typescript-eslint/project-service".into(),
            "@typescript-eslint/scope-manager".into(),
            "@typescript-eslint/tsconfig-utils".into(),
            "@typescript-eslint/type-utils".into(),
            "@typescript-eslint/types".into(),
            "@typescript-eslint/typescript-estree".into(),
            "@typescript-eslint/utils".into(),
            "@typescript-eslint/visitor-keys".into(),
            "acorn".into(),
            "acorn-jsx".into(),
            "ajv".into(),
            "ansi-styles".into(),
            "argparse".into(),
            "asynckit".into(),
            "balanced-match".into(),
            "baseline-browser-mapping".into(),
            "brace-expansion".into(),
            "browserslist".into(),
            "call-bind-apply-helpers".into(),
            "callsites".into(),
            "caniuse-lite".into(),
            "chalk".into(),
            "color-convert".into(),
            "color-name".into(),
            "combined-stream".into(),
            "concat-map".into(),
            "convert-source-map".into(),
            "cookie".into(),
            "cross-spawn".into(),
            "csstype".into(),
            "debug".into(),
            "deep-is".into(),
            "delayed-stream".into(),
            "detect-libc".into(),
            "dunder-proto".into(),
            "electron-to-chromium".into(),
            "enhanced-resolve".into(),
            "es-define-property".into(),
            "es-errors".into(),
            "es-object-atoms".into(),
            "es-set-tostringtag".into(),
            "escalade".into(),
            "escape-string-regexp".into(),
            "eslint-scope".into(),
            "eslint-visitor-keys".into(),
            "espree".into(),
            "esquery".into(),
            "esrecurse".into(),
            "estraverse".into(),
            "esutils".into(),
            "fast-deep-equal".into(),
            "fast-json-stable-stringify".into(),
            "fast-levenshtein".into(),
            "fdir".into(),
            "file-entry-cache".into(),
            "find-up".into(),
            "flat-cache".into(),
            "flatted".into(),
            "follow-redirects".into(),
            "form-data".into(),
            "function-bind".into(),
            "gensync".into(),
            "get-intrinsic".into(),
            "get-proto".into(),
            "glob-parent".into(),
            "gopd".into(),
            "graceful-fs".into(),
            "has-flag".into(),
            "has-symbols".into(),
            "has-tostringtag".into(),
            "hasown".into(),
            "hermes-estree".into(),
            "hermes-parser".into(),
            "ignore".into(),
            "import-fresh".into(),
            "imurmurhash".into(),
            "is-extglob".into(),
            "is-glob".into(),
            "isexe".into(),
            "jiti".into(),
            "js-tokens".into(),
            "js-yaml".into(),
            "jsesc".into(),
            "json-buffer".into(),
            "json-schema-traverse".into(),
            "json-stable-stringify-without-jsonify".into(),
            "json5".into(),
            "keyv".into(),
            "levn".into(),
            "lightningcss".into(),
            "lightningcss-linux-x64-gnu".into(),
            "lightningcss-linux-x64-musl".into(),
            "locate-path".into(),
            "lodash.merge".into(),
            "lru-cache".into(),
            "magic-string".into(),
            "math-intrinsics".into(),
            "mime-db".into(),
            "mime-types".into(),
            "minimatch".into(),
            "ms".into(),
            "nanoid".into(),
            "natural-compare".into(),
            "node-releases".into(),
            "optionator".into(),
            "p-limit".into(),
            "p-locate".into(),
            "parent-module".into(),
            "path-exists".into(),
            "path-key".into(),
            "picocolors".into(),
            "picomatch".into(),
            "postcss".into(),
            "prelude-ls".into(),
            "proxy-from-env".into(),
            "punycode".into(),
            "react-router".into(),
            "resolve-from".into(),
            "rolldown".into(),
            "scheduler".into(),
            "semver".into(),
            "set-cookie-parser".into(),
            "shebang-command".into(),
            "shebang-regex".into(),
            "source-map-js".into(),
            "strip-json-comments".into(),
            "supports-color".into(),
            "tapable".into(),
            "tinyglobby".into(),
            "ts-api-utils".into(),
            "type-check".into(),
            "undici-types".into(),
            "update-browserslist-db".into(),
            "uri-js".into(),
            "which".into(),
            "word-wrap".into(),
            "yallist".into(),
            "yocto-queue".into(),
            "zod-validation-error".into(),
        ],
        files: vec![
            StackFile {
                path: "README.md".into(),
                content: r###"# <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/doc/logo.svg" alt="offpkg Logo" width="36" height="36" align="center"/> Offpkg Vite+React Template 🚀

A premium, highly-opinionated Vite + React starter template designed for scalability, type-safety, and modern developer experience.

## ✨ Features

- **Next.js-like Architecture**: Structured layouts, pages, and routing.
- **Tailwind CSS v4**: Modern styling with CSS variables and OKLCH color spaces.
- **Zustand State Management**: Persistent global stores for theme and application state.
- **Type-Safe API & Validation**: Axios integration with Zod schemas and React Query (TanStack).
- **Premium UI Components**: Custom-built, accessible components inspired by Shadcn UI.
- **Enhanced Logging**: Structured, group-collapsed console output for a cleaner dev experience.

---

## 🛠️ Tech Stack

- **Framework**: [React 19](https://react.dev/)
- **Bundler**: [Vite 8](https://vite.dev/)
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/)
- **Router**: [React Router 7](https://reactrouter.com/)
- **State**: [Zustand](https://docs.pmnd.rs/zustand)
- **Data Fetching**: [TanStack Query v5](https://tanstack.com/query)
- **Validation**: [Zod](https://zod.dev/)
- **Forms**: [React Hook Form](https://react-hook-form.com/)
- **Icons**: [Lucide React](https://lucide.dev/)

---

## 🚀 Getting Started

### 1. Installation

```bash
bun install
```

### 2. Development

```bash
bun run dev
```

### 3. Build

```bash
bun run build
```

---

## 🎨 Theme Setup & Modification

### Theme Store
The theme state is managed by Zustand in `src/store/useThemeStore.ts`. It supports `light`, `dark`, and `system` modes with automatic persistence to `localStorage`.

### Theme Provider
Wrap your application (or specific sections) with `<ThemeProvider />` from `src/components/ThemeProvider.tsx`.

### Customizing Colors
Modify the CSS variables in `src/index.css` within the `@theme` block. We use OKLCH for better color perception.

```css
@theme {
  --color-primary: oklch(0.59 0.201 273.444);
  --color-background: oklch(1 0 0);
  /* ... */
}
```

---

## 📦 Package Usage Guides

### 🌐 API (Axios + React Query)
API calls are centralized in `src/api/axios.ts`. Use React Query for data fetching:

```tsx
const { data, isLoading } = useQuery({
  queryKey: ['users'],
  queryFn: () => axiosInstance.get('/users').then(res => res.data),
});
```

### 🛡️ Validation (Zod)
Define your data shapes in `src/types/schema.ts`:

```typescript
const UserSchema = z.object({
  id: z.string(),
  name: z.string(),
});
```

### 📝 Forms (React Hook Form)
Integrated with `@hookform/resolvers` for Zod support:

```tsx
const form = useForm({
  resolver: zodResolver(UserSchema),
});
```

### 📦 State (Zustand)
Create stores in `src/store/`:

```typescript
export const useAuthStore = create((set) => ({
  user: null,
  login: (user) => set({ user }),
}));
```

---

## 📂 Project Structure

```text
src/
├── api/          # Axios instance and API calls
├── assets/       # Static assets (images, svgs)
├── components/   # Reusable UI components
├── hooks/        # Custom React hooks
├── layouts/      # Page layouts (e.g., RootLayout)
├── lib/          # Utilities (logger, etc.)
├── pages/        # Route-level components
├── providers/    # Context/Query providers
├── store/        # Zustand stores
└── types/        # Zod schemas and TS types
```

---

## 📜 License
MIT
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "tsconfig.json".into(),
                content: r###"{
  "files": [],
  "references": [
    { "path": "./tsconfig.app.json" },
    { "path": "./tsconfig.node.json" }
  ]
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "index.html".into(),
                content: r###"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>offpkg vite+react</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "package.json".into(),
                content: r###"{
  "name": "offpkg-vite-react",
  "private": true,
  "version": "0.0.0",
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "tsc -b && vite build",
    "lint": "eslint .",
    "preview": "vite preview"
  },
  "dependencies": {
    "@hookform/resolvers": "^5.2.2",
    "@tailwindcss/vite": "^4.2.2",
    "@tanstack/react-query": "^5.95.2",
    "axios": "^1.13.6",
    "lucide-react": "^1.6.0",
    "react": "^19.2.4",
    "react-dom": "^19.2.4",
    "react-hook-form": "^7.72.0",
    "react-router-dom": "^7.13.2",
    "tailwindcss": "^4.2.2",
    "zod": "^4.3.6",
    "zustand": "^5.0.12"
  },
  "devDependencies": {
    "@eslint/js": "^9.39.4",
    "@types/node": "^24.12.0",
    "@types/react": "^19.2.14",
    "@types/react-dom": "^19.2.3",
    "@vitejs/plugin-react": "^6.0.1",
    "eslint": "^9.39.4",
    "eslint-plugin-react-hooks": "^7.0.1",
    "eslint-plugin-react-refresh": "^0.5.2",
    "globals": "^17.4.0",
    "typescript": "~5.9.3",
    "typescript-eslint": "^8.57.0",
    "vite": "^8.0.1"
  }
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: ".gitignore".into(),
                content: r###"# Logs
logs
*.log
npm-debug.log*
yarn-debug.log*
yarn-error.log*
pnpm-debug.log*
lerna-debug.log*

node_modules
dist
dist-ssr
*.local

# Editor directories and files
.vscode/*
!.vscode/extensions.json
.idea
.DS_Store
*.suo
*.ntvs*
*.njsproj
*.sln
*.sw?
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "tsconfig.app.json".into(),
                content: r###"{
  "compilerOptions": {
    "tsBuildInfoFile": "./node_modules/.tmp/tsconfig.app.tsbuildinfo",
    "target": "ES2023",
    "useDefineForClassFields": true,
    "lib": ["ES2023", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "types": ["vite/client"],
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "verbatimModuleSyntax": true,
    "moduleDetection": "force",
    "noEmit": true,
    "jsx": "react-jsx",

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "erasableSyntaxOnly": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedSideEffectImports": true,
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src"]
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "vite.config.ts".into(),
                content: r###"import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import path from 'path'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    tailwindcss(),
  ],
  server: {
    hmr: {
      overlay: true,
    },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
})
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "eslint.config.js".into(),
                content: r###"import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import tseslint from 'typescript-eslint'
import { defineConfig, globalIgnores } from 'eslint/config'

export default defineConfig([
  globalIgnores(['dist']),
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      js.configs.recommended,
      tseslint.configs.recommended,
      reactHooks.configs.flat.recommended,
      reactRefresh.configs.vite,
    ],
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
  },
])
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "tsconfig.node.json".into(),
                content: r###"{
  "compilerOptions": {
    "tsBuildInfoFile": "./node_modules/.tmp/tsconfig.node.tsbuildinfo",
    "target": "ES2023",
    "lib": ["ES2023"],
    "module": "ESNext",
    "types": ["node"],
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "verbatimModuleSyntax": true,
    "moduleDetection": "force",
    "noEmit": true,

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "erasableSyntaxOnly": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedSideEffectImports": true
  },
  "include": ["vite.config.ts"]
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "public/favicon.svg".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/favicon.svg").to_vec()),
            },
            StackFile {
                path: "public/icons.svg".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/icons.svg").to_vec()),
            },
            StackFile {
                path: "src/index.css".into(),
                content: r###"@import url('https://fonts.googleapis.com/css2?family=Manrope:wght@400;500;600;700&family=Geist:wght@400;500;600;700&display=swap');
@import "tailwindcss";

@theme {
  --color-background: oklch(var(--background));
  --color-foreground: oklch(var(--foreground));

  --color-primary: oklch(var(--primary));
  --color-primary-foreground: oklch(var(--primary-foreground));

  --color-secondary: oklch(var(--secondary));
  --color-secondary-foreground: oklch(var(--secondary-foreground));

  --color-muted: oklch(var(--muted));
  --color-muted-foreground: oklch(var(--muted-foreground));

  --color-accent: oklch(var(--accent));
  --color-accent-foreground: oklch(var(--accent-foreground));

  --color-destructive: oklch(var(--destructive));
  --color-destructive-foreground: oklch(var(--destructive-foreground));

  --color-border: oklch(var(--border));
  --color-input: oklch(var(--input));
  --color-ring: oklch(var(--ring));

  --radius-lg: 0.5rem;
  --radius-md: calc(0.5rem - 2px);
  --radius-sm: calc(0.5rem - 4px);

  --font-heading: 'Manrope', sans-serif;
  --font-body: 'Geist', sans-serif;
}

@layer base {
  :root {
    --background: 1 0 0;
    --foreground: 0.141 0.005 285.823;
    --primary: 0.59 0.201 273.444;
    --primary-foreground: 1 0 0;
    --secondary: 0.949 0.029 303.081;
    --secondary-foreground: 0.21 0.006 285.885;
    --muted: 0.963 0.023 308.198;
    --muted-foreground: 0.472 0.002 286.339;
    --accent: 0.949 0.029 303.081;
    --accent-foreground: 0.211 0.006 285.885;
    --destructive: 0.637 0.208 25.331;
    --destructive-foreground: 0.985 0 0;
    --border: 0.92 0.02 285; /* Adjusted for better visibility */
    --input: 0.92 0.02 285;
    --ring: 0.59 0.201 273.444;
    --radius: 0.5rem;
  }

  .dark {
    --background: 0.141 0.005 285.823;
    --foreground: 0.985 0 0;
    --primary: 0.665 0.179 278.961;
    --primary-foreground: 1 0 0;
    --secondary: 0.202 0.107 263.462;
    --secondary-foreground: 0.985 0 0;
    --muted: 0.167 0.112 264.144;
    --muted-foreground: 0.673 0 0;
    --accent: 0.202 0.107 263.462;
    --accent-foreground: 0.985 0 0;
    --destructive: 0.396 0.133 25.723;
    --destructive-foreground: 0.985 0 0;
    --border: 0.25 0.05 264; /* Adjusted for dark mode */
    --input: 0.25 0.05 264;
    --ring: 0.665 0.179 278.961;
  }
}

@layer base {
  * {
    @apply border-border;
  }
  body {
    @apply bg-background text-foreground font-body;
  }
  h1, h2, h3, h4, h5, h6 {
    @apply font-heading font-bold;
  }
}

"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/App.css".into(),
                content: r###".counter {
  font-size: 16px;
  padding: 5px 10px;
  border-radius: 5px;
  color: var(--accent);
  background: var(--accent-bg);
  border: 2px solid transparent;
  transition: border-color 0.3s;
  margin-bottom: 24px;

  &:hover {
    border-color: var(--accent-border);
  }
  &:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
}

.hero {
  position: relative;

  .base,
  .framework,
  .vite {
    inset-inline: 0;
    margin: 0 auto;
  }

  .base {
    width: 170px;
    position: relative;
    z-index: 0;
  }

  .framework,
  .vite {
    position: absolute;
  }

  .framework {
    z-index: 1;
    top: 34px;
    height: 28px;
    transform: perspective(2000px) rotateZ(300deg) rotateX(44deg) rotateY(39deg)
      scale(1.4);
  }

  .vite {
    z-index: 0;
    top: 107px;
    height: 26px;
    width: auto;
    transform: perspective(2000px) rotateZ(300deg) rotateX(40deg) rotateY(39deg)
      scale(0.8);
  }
}

#center {
  display: flex;
  flex-direction: column;
  gap: 25px;
  place-content: center;
  place-items: center;
  flex-grow: 1;

  @media (max-width: 1024px) {
    padding: 32px 20px 24px;
    gap: 18px;
  }
}

#next-steps {
  display: flex;
  border-top: 1px solid var(--border);
  text-align: left;

  & > div {
    flex: 1 1 0;
    padding: 32px;
    @media (max-width: 1024px) {
      padding: 24px 20px;
    }
  }

  .icon {
    margin-bottom: 16px;
    width: 22px;
    height: 22px;
  }

  @media (max-width: 1024px) {
    flex-direction: column;
    text-align: center;
  }
}

#docs {
  border-right: 1px solid var(--border);

  @media (max-width: 1024px) {
    border-right: none;
    border-bottom: 1px solid var(--border);
  }
}

#next-steps ul {
  list-style: none;
  padding: 0;
  display: flex;
  gap: 8px;
  margin: 32px 0 0;

  .logo {
    height: 18px;
  }

  a {
    color: var(--text-h);
    font-size: 16px;
    border-radius: 6px;
    background: var(--social-bg);
    display: flex;
    padding: 6px 12px;
    align-items: center;
    gap: 8px;
    text-decoration: none;
    transition: box-shadow 0.3s;

    &:hover {
      box-shadow: var(--shadow);
    }
    .button-icon {
      height: 18px;
      width: 18px;
    }
  }

  @media (max-width: 1024px) {
    margin-top: 20px;
    flex-wrap: wrap;
    justify-content: center;

    li {
      flex: 1 1 calc(50% - 8px);
    }

    a {
      width: 100%;
      justify-content: center;
      box-sizing: border-box;
    }
  }
}

#spacer {
  height: 88px;
  border-top: 1px solid var(--border);
  @media (max-width: 1024px) {
    height: 48px;
  }
}

.ticks {
  position: relative;
  width: 100%;

  &::before,
  &::after {
    content: '';
    position: absolute;
    top: -4.5px;
    border: 5px solid transparent;
  }

  &::before {
    left: 0;
    border-left-color: var(--border);
  }
  &::after {
    right: 0;
    border-right-color: var(--border);
  }
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/main.tsx".into(),
                content: r###"import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { QueryProvider } from '@/providers/QueryProvider.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <QueryProvider>
      <App />
    </QueryProvider>
  </StrictMode>,
)
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/App.tsx".into(),
                content: r###"import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import { RootLayout } from '@/layouts/RootLayout';
import { HomePage } from '@/pages/HomePage';
import { NotFoundPage } from '@/pages/NotFoundPage';
import './App.css';

const router = createBrowserRouter([
  {
    path: '/',
    element: <RootLayout />,
    children: [
      {
        index: true,
        element: <HomePage />,
      },
      // Add other routes here, e.g.:
      // { path: 'about', element: <AboutPage /> },
      {
        path: '*',
        element: <NotFoundPage />,
      },
    ],
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;



"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/assets/vite.svg".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/vite.svg").to_vec()),
            },
            StackFile {
                path: "src/assets/react.svg".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/react.svg").to_vec()),
            },
            StackFile {
                path: "src/assets/hero.png".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/hero.png").to_vec()),
            },
            StackFile {
                path: "src/providers/QueryProvider.tsx".into(),
                content: r###"import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 5, // 5 minutes
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

export function QueryProvider({ children }: { children: React.ReactNode }) {
  return (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/ThemeProvider.tsx".into(),
                content: r###"import React, { useEffect } from 'react';
import { useThemeStore } from '../store/useThemeStore';

interface ThemeProviderProps {
  children: React.ReactNode;
  inlineTheme?: Record<string, string>;
}

export function ThemeProvider({
  children,
  inlineTheme,
}: ThemeProviderProps) {
  const { theme } = useThemeStore();

  useEffect(() => {
    const root = window.document.documentElement;

    root.classList.remove('light', 'dark');

    if (theme === 'system') {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light';

      root.classList.add(systemTheme);
      return;
    }

    root.classList.add(theme);
  }, [theme]);

  return (
    <div style={inlineTheme as React.CSSProperties}>
      {children}
    </div>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/ThemeToggleButton.tsx".into(),
                content: r###"import { Moon, Sun} from 'lucide-react';
import { useThemeStore } from '../store/useThemeStore';
import Button from './button';


export function ThemeToggleButton() {
  const { theme, setTheme } = useThemeStore();

  const cycleTheme = () => {
    if (theme === 'light') setTheme('dark');
    else setTheme('light');
  };

  return (
    <Button
      variant="outline"
      size="icon"
      onClick={cycleTheme}
      title={`Current: ${theme} • Click to cycle`}
      className="fixed top-4 right-4 h-10 w-10 rounded-full"
    >
      <Sun
        className={`h-[1.2rem] w-[1.2rem] transition-all ${
          theme === 'light'
            ? 'rotate-0 scale-100'
            : 'rotate-90 scale-0'
        }`}
      />
      <Moon
        className={`absolute h-[1.2rem] w-[1.2rem] transition-all ${
          theme === 'dark'
            ? 'rotate-0 scale-100'
            : 'rotate-90 scale-0'
        }`}
      />
      <span className="sr-only">Toggle theme</span>
    </Button>
  );
}"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/button.tsx".into(),
                content: r###"import { forwardRef, type ButtonHTMLAttributes } from 'react';

type ButtonVariant = 'primary' | 'secondary' | 'outline' | 'ghost';
type ButtonSize = 'default' | 'sm' | 'lg' | 'icon';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
}

const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ 
    children, 
    variant = 'primary', 
    size = 'default', 
    className = '', 
    ...props 
  }, ref) => {
    
    const baseClasses = "inline-flex items-center justify-center font-medium font-body rounded-button transition-all active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-primary";

    const variantClasses = {
      primary: "bg-primary text-primary-foreground shadow-sm hover:shadow-md hover:bg-primary/90",
      secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
      outline: "border border-border bg-transparent hover:bg-muted hover:text-foreground",
      ghost: "hover:bg-muted hover:text-foreground",
    };

    const sizeClasses = {
      default: "px-4 py-2 text-sm",
      sm: "px-3 py-1.5 text-xs",
      lg: "px-6 py-3 text-base",
      icon: "h-10 w-10 p-0",
    };

    return (
      <button
        ref={ref}
        className={`${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${className}`}
        {...props}
      >
        {children}
      </button>
    );
  }
);

Button.displayName = "Button";

export default Button;"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/Navbar.tsx".into(),
                content: r###"import { Link } from 'react-router-dom';
import { ThemeToggleButton } from '@/components/ThemeToggleButton';
import { Home, Info, Mail } from 'lucide-react';

export function Navbar() {
  return (
    <nav className="fixed top-0 left-0 right-0 h-16 border-b border-border/40 bg-background/80 backdrop-blur-md z-50 flex items-center justify-between px-6">
      <div className="flex items-center gap-8">
        <Link to="/" className="text-xl font-heading font-bold tracking-tight text-primary transition-opacity hover:opacity-80 flex items-center gap-2">
          <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/doc/logo.svg" alt="Offpkg Logo" className="w-6 h-6" />
          <span>OFFPKG</span>
        </Link>
        <div className="hidden md:flex items-center gap-6">
          <Link to="/" className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors flex items-center gap-2">
            <Home className="w-4 h-4" /> Home
          </Link>
          <Link to="/about" className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors flex items-center gap-2">
            <Info className="w-4 h-4" /> About
          </Link>
          <Link to="/contact" className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors flex items-center gap-2">
            <Mail className="w-4 h-4" /> Contact
          </Link>
        </div>
      </div>
      <ThemeToggleButton />
    </nav>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/lib/logger.ts".into(),
                content: r###"type LogLevel = "info" | "warn" | "error" | "debug"

const isDev = import.meta.env.DEV

const styles: Record<LogLevel, string> = {
  info: "color: #3b82f6; font-weight: 600;",
  warn: "color: #f59e0b; font-weight: 600;",
  error: "color: #ef4444; font-weight: 600;",
  debug: "color: #10b981; font-weight: 600;",
}

const formatMessage = (level: LogLevel, message: string) => {
  const prefix = `[WEB] ${level.toUpperCase()}`
  return [`%c${prefix} %c${message}`, styles[level], "color: inherit; font-weight: normal;"]
}

function log(level: LogLevel, message: string, data?: unknown) {
  if (!isDev && level === "debug") return

  const [prompt, style, reset] = formatMessage(level, message)

  if (data === undefined) {
    if (level === "error") console.error(prompt, style, reset)
    else if (level === "warn") console.warn(prompt, style, reset)
    else console.log(prompt, style, reset)
    return
  }

  // Handle data with grouping for a cleaner console
  console.groupCollapsed(prompt, style, reset)
  
  if (data instanceof Error) {
    console.error(data.message)
    if (data.stack) console.debug(data.stack)
  } else {
    console.dir(data)
  }
  
  console.groupEnd()
}

export const logger = {
  info: (msg: string, data?: unknown) => log("info", msg, data),
  warn: (msg: string, data?: unknown) => log("warn", msg, data),
  error: (msg: string, data?: unknown) => log("error", msg, data),
  debug: (msg: string, data?: unknown) => log("debug", msg, data),
}"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/pages/NotFoundPage.tsx".into(),
                content: r###"import { Link } from 'react-router-dom';
import { MoveLeft } from 'lucide-react';

export function NotFoundPage() {
  return (
    <div className="min-h-[calc(100vh-4rem)] flex flex-col items-center justify-center p-8 text-center animate-in fade-in duration-700">
      <div className="relative mb-8">
        <h1 className="text-[12rem] font-black leading-none tracking-tighter text-muted-foreground/10 select-none">
          404
        </h1>
        <div className="absolute inset-0 flex items-center justify-center">
          <p className="text-4xl font-heading font-black tracking-tight">PAGE NOT FOUND</p>
        </div>
      </div>
      
      <p className="text-xl text-muted-foreground mb-12 max-w-md mx-auto leading-relaxed">
        The page you are looking for doesn't exist or has been moved to another universe.
      </p>

      <Link
        to="/"
        className="flex items-center gap-2 bg-primary text-primary-foreground px-8 py-4 rounded-2xl font-bold transition-all hover:gap-4 hover:pr-10 hover:shadow-xl active:scale-95"
      >
        <MoveLeft className="w-5 h-5" /> Back to Home
      </Link>
    </div>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/pages/HomePage.tsx".into(),
                content: r###"import { useState } from 'react';
import reactLogo from '../assets/react.svg';
import viteLogo from '../assets/vite.svg';
import heroImg from '../assets/hero.png';
import { Zap, ShieldCheck, Package } from 'lucide-react';

export function HomePage() {
  const [count, setCount] = useState(0);

  return (
    <div className="animate-in fade-in slide-in-from-bottom-4 duration-1000">
      <section id="center" className="min-h-[calc(100vh-4rem)] flex flex-col items-center justify-center p-8">
        <div className="hero relative mb-12">
          <div className="absolute -inset-4 bg-primary/20 blur-3xl rounded-full animate-pulse" />
          <img src={heroImg} className="relative base w-48 h-auto drop-shadow-2xl" alt="" />
          <img src={reactLogo} className="framework absolute -top-6 -right-6 w-14 h-14 animate-spin-slow" alt="React logo" />
          <img src={viteLogo} className="vite absolute -bottom-6 -left-6 w-14 h-14" alt="Vite logo" />
        </div>

        <div className="text-center mb-12 max-w-2xl">
          <h1 className="text-6xl font-heading mb-6 tracking-tighter leading-tight bg-gradient-to-r from-foreground to-foreground/70 bg-clip-text text-transparent">
            Get started Offpkg <br /> Vite+React
          </h1>
          <p className="text-muted-foreground text-xl leading-relaxed">
            The ultimate developer setup with
            <span className="text-primary font-semibold"> Tailwind v4, Zustand, Zod, </span> and
            <span className="text-primary font-semibold"> React Query</span>.
          </p>
        </div>

        <div className="flex flex-col items-center gap-4">
          <button
            className="group relative bg-primary text-primary-foreground px-8 py-4 rounded-2xl font-semibold text-lg transition-all hover:scale-105 active:scale-95 shadow-lg shadow-primary/20"
            onClick={() => setCount((c) => c + 1)}
          >
            Count is {count}
            <div className="absolute inset-0 rounded-2xl ring-1 ring-white/20 group-hover:ring-white/40 transition-all" />
          </button>
          <p className="text-sm text-muted-foreground">
            Edit <code className="bg-muted px-1.5 py-0.5 rounded font-mono">src/pages/HomePage.tsx</code> to test HMR
          </p>
        </div>
      </section>

      <div className="h-px bg-gradient-to-r from-transparent via-border to-transparent w-full" />

      <section id="features" className="p-16 max-w-6xl mx-auto grid md:grid-cols-3 gap-8">
        <FeatureCard
          title="Fast Refresh"
          desc="Lightning fast HMR provided by Vite 8 for an ultra-smooth dev experience."
          icon={<Zap className="w-8 h-8 text-yellow-500" />}
        />
        <FeatureCard
          title="Type Safe"
          desc="Zod and TypeScript integration ensures your data is always valid."
          icon={<ShieldCheck className="w-8 h-8 text-blue-500" />}
        />
        <FeatureCard
          title="State Master"
          desc="Global state management simplified with Zustand stores."
          icon={<Package className="w-8 h-8 text-purple-500" />}
        />
      </section>
    </div>
  );
}

function FeatureCard({ title, desc, icon }: { title: string; desc: string; icon: React.ReactNode }) {
  return (
    <div className="p-8 rounded-3xl border bg-card/50 backdrop-blur-sm text-card-foreground hover:border-primary/50 transition-colors group cursor-default">
      <div className="mb-4 group-hover:scale-110 transition-transform">{icon}</div>
      <h3 className="text-2xl font-bold mb-3">{title}</h3>
      <p className="text-muted-foreground leading-relaxed">{desc}</p>
    </div>
  );
}

"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/api/axios.ts".into(),
                content: r###"import axios from 'axios';
import { logger } from '../lib/logger';

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL || 'https://api.example.com',
  headers: {
    'Content-Type': 'application/json',
  },
});

api.interceptors.request.use(
  (config) => {
    logger.info(`Request: ${config.method?.toUpperCase()} ${config.url}`);
    return config;
  },
  (error) => {
    logger.error('Request Error', error);
    return Promise.reject(error);
  }
);

api.interceptors.response.use(
  (response) => {
    logger.info(`Response: ${response.status} ${response.config.url}`);
    return response;
  },
  (error) => {
    logger.error('Response Error', error.response?.data || error.message);
    return Promise.reject(error);
  }
);

export default api;
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/store/useAppStore.ts".into(),
                content: r###"import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface AppState {
  user: { name: string; email: string } | null;
  setUser: (user: { name: string; email: string } | null) => void;
  isLoading: boolean;
  setIsLoading: (loading: boolean) => void;
}

export const useAppStore = create<AppState>()(
  persist(
    (set) => ({
      user: null,
      setUser: (user) => set({ user }),
      isLoading: false,
      setIsLoading: (isLoading) => set({ isLoading }),
    }),
    {
      name: 'app-storage',
    }
  )
);
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/store/useThemeStore.ts".into(),
                content: r###"import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export type Theme = 'light' | 'dark' | 'system';

interface ThemeState {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

export const useThemeStore = create<ThemeState>()(
  persist(
    (set) => ({
      theme: 'system',
      setTheme: (theme) => set({ theme }),
    }),
    {
      name: 'theme-storage',
    }
  )
);
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/hooks/useUser.ts".into(),
                content: r###"import { useQuery } from '@tanstack/react-query';
import api from '../api/axios';
import { UserSchema } from '../types/schema';

export const useUser = (userId: string) => {
  return useQuery({
    queryKey: ['user', userId],
    queryFn: async () => {
      const { data } = await api.get(`/users/${userId}`);
      return UserSchema.parse(data);
    },
    enabled: !!userId,
  });
};
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/types/schema.ts".into(),
                content: r###"import { z } from 'zod';

export const UserSchema = z.object({
  id: z.string(),
  name: z.string().min(2, 'Name must be at least 2 characters'),
  email: z.string().email('Invalid email address'),
  role: z.enum(['admin', 'user', 'guest']),
});

export type User = z.infer<typeof UserSchema>;

export const LoginFormSchema = z.object({
  email: z.string().email(),
  password: z.string().min(6, 'Password must be at least 6 characters'),
});

export type LoginFormValues = z.infer<typeof LoginFormSchema>;
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/layouts/RootLayout.tsx".into(),
                content: r###"import { Outlet } from 'react-router-dom';
import { Navbar } from '@/components/Navbar';
import { ThemeProvider } from '@/components/ThemeProvider';

export function RootLayout() {
  return (
    <ThemeProvider>
      <div className="min-h-screen bg-background text-foreground transition-colors duration-300">
        <Navbar />
        <main className="pt-16">
          <Outlet />
        </main>
      </div>
    </ThemeProvider>
  );
}
"###.into(),
                binary_content: None,
            }
        ],
    }
}

pub fn react_vite_gsap() -> Stack {
    Stack {
        name: "react-vite-gsap".into(),
        runtime: "bun".into(),
        description: "React 19 + Vite 8 + Tailwind 4 + GSAP + Framer Motion + Lenis + shadcn/ui + Lordicon (Kinetic Motion Template)".into(),
        packages: vec![
            "@fontsource-variable/inter".into(),
            "@hookform/resolvers".into(),
            "@lordicon/react".into(),
            "@tailwindcss/vite".into(),
            "@tanstack/react-query".into(),
            "axios".into(),
            "class-variance-authority".into(),
            "clsx".into(),
            "framer-motion".into(),
            "gsap".into(),
            "lenis".into(),
            "lottie-react".into(),
            "lottie-web".into(),
            "lucide-react".into(),
            "radix-ui".into(),
            "react".into(),
            "react-dom".into(),
            "react-hook-form".into(),
            "react-router-dom".into(),
            "shadcn".into(),
            "tailwind-merge".into(),
            "tailwindcss".into(),
            "tw-animate-css".into(),
            "zod".into(),
            "zustand".into(),
        ],
        dev_packages: vec![
            "@eslint/js".into(),
            "@types/node".into(),
            "@types/react".into(),
            "@types/react-dom".into(),
            "@vitejs/plugin-react".into(),
            "eslint".into(),
            "eslint-plugin-react-hooks".into(),
            "eslint-plugin-react-refresh".into(),
            "globals".into(),
            "typescript".into(),
            "typescript-eslint".into(),
            "vite".into(),
        ],
        transitive_packages: vec![
            "@babel/code-frame".into(),
            "@babel/compat-data".into(),
            "@babel/core".into(),
            "@babel/generator".into(),
            "@babel/helper-compilation-targets".into(),
            "@babel/helper-globals".into(),
            "@babel/helper-module-imports".into(),
            "@babel/helper-module-transforms".into(),
            "@babel/helper-string-parser".into(),
            "@babel/helper-validator-identifier".into(),
            "@babel/helper-validator-option".into(),
            "@babel/helpers".into(),
            "@babel/parser".into(),
            "@babel/template".into(),
            "@babel/traverse".into(),
            "@babel/types".into(),
            "@eslint-community/eslint-utils".into(),
            "@eslint-community/regexpp".into(),
            "@eslint/config-array".into(),
            "@eslint/config-helpers".into(),
            "@eslint/core".into(),
            "@eslint/eslintrc".into(),
            "@eslint/object-schema".into(),
            "@eslint/plugin-kit".into(),
            "@humanfs/core".into(),
            "@humanfs/node".into(),
            "@humanwhocodes/module-importer".into(),
            "@humanwhocodes/retry".into(),
            "@jridgewell/gen-mapping".into(),
            "@jridgewell/remapping".into(),
            "@jridgewell/resolve-uri".into(),
            "@jridgewell/sourcemap-codec".into(),
            "@jridgewell/trace-mapping".into(),
            "@oxc-project/types".into(),
            "@rolldown/binding-linux-x64-gnu".into(),
            "@rolldown/binding-linux-x64-musl".into(),
            "@rolldown/pluginutils".into(),
            "@standard-schema/utils".into(),
            "@tailwindcss/node".into(),
            "@tailwindcss/oxide".into(),
            "@tailwindcss/oxide-linux-x64-gnu".into(),
            "@tailwindcss/oxide-linux-x64-musl".into(),
            "@tanstack/query-core".into(),
            "@types/estree".into(),
            "@types/json-schema".into(),
            "@typescript-eslint/eslint-plugin".into(),
            "@typescript-eslint/parser".into(),
            "@typescript-eslint/project-service".into(),
            "@typescript-eslint/scope-manager".into(),
            "@typescript-eslint/tsconfig-utils".into(),
            "@typescript-eslint/type-utils".into(),
            "@typescript-eslint/types".into(),
            "@typescript-eslint/typescript-estree".into(),
            "@typescript-eslint/utils".into(),
            "@typescript-eslint/visitor-keys".into(),
            "acorn".into(),
            "acorn-jsx".into(),
            "ajv".into(),
            "ansi-styles".into(),
            "argparse".into(),
            "asynckit".into(),
            "balanced-match".into(),
            "baseline-browser-mapping".into(),
            "brace-expansion".into(),
            "browserslist".into(),
            "call-bind-apply-helpers".into(),
            "callsites".into(),
            "caniuse-lite".into(),
            "chalk".into(),
            "color-convert".into(),
            "color-name".into(),
            "concat-map".into(),
            "cross-spawn".into(),
            "debug".into(),
            "deep-is".into(),
            "escape-string-regexp".into(),
            "eslint-scope".into(),
            "eslint-visitor-keys".into(),
            "espree".into(),
            "esquery".into(),
            "esrecurse".into(),
            "estraverse".into(),
            "esutils".into(),
            "fast-deep-equal".into(),
            "fast-json-stable-stringify".into(),
            "fast-levenshtein".into(),
            "file-entry-cache".into(),
            "find-up".into(),
            "flat-cache".into(),
            "flatted".into(),
            "fn.name".into(),
            "fraction.js".into(),
            "fsevents".into(),
            "get-tsconfig".into(),
            "glob-parent".into(),
            "graphemer".into(),
            "has-flag".into(),
            "ignore".into(),
            "import-fresh".into(),
            "imurmurhash".into(),
            "is-extglob".into(),
            "is-glob".into(),
            "isexe".into(),
            "js-yaml".into(),
            "json-buffer".into(),
            "json-schema-traverse".into(),
            "json-stable-stringify-without-jsonify".into(),
            "keyv".into(),
            "levn".into(),
            "locate-path".into(),
            "lodash.merge".into(),
            "minimatch".into(),
            "ms".into(),
            "natural-compare".into(),
            "normalize-range".into(),
            "optionator".into(),
            "p-limit".into(),
            "p-locate".into(),
            "parent-module".into(),
            "path-exists".into(),
            "path-key".into(),
            "picocolors".into(),
            "picomatch".into(),
            "postcss".into(),
            "postcss-value-parser".into(),
            "prelude-ls".into(),
            "punycode".into(),
            "queue-microtask".into(),
            "resolve-from".into(),
            "rolldown".into(),
            "run-parallel".into(),
            "semver".into(),
            "shebang-command".into(),
            "shebang-regex".into(),
            "source-map-js".into(),
            "strip-json-comments".into(),
            "supports-color".into(),
            "to-regex-range".into(),
            "ts-api-utils".into(),
            "type-check".into(),
            "uri-js".into(),
            "which".into(),
            "word-wrap".into(),
            "yocto-queue".into(),
        ],
        files: vec![
            StackFile {
                path: "README.md".into(),
                content: r####"# <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/doc/logo.svg" alt="offpkg Logo" width="36" height="36" align="center"/> Offpkg Vite+React Kinetic Template 🚀

A premium, highly-opinionated Vite + React starter template designed for creative visual web development with seamless smooth scrolling, physics-based UI motion, accessible components, and animated vector icons.

## ✨ Kinetic Features

- **Next.js-like Architecture**: Structured layouts, pages, and routing using React Router 7.
- **Ultra-Smooth scrolling (Lenis)**: Standardized momentum smooth scrolling that eliminates browser scrolling jitters.
- **GSAP & ScrollTrigger Timeline Controls**: Orchestrate visual scroll-driven reveals and staggered sequences.
- **Framer Motion Gestures**: Physics-based drag responses, spring curves, and layout states.
- **Interactive Lordicons**: Vector-based animated SVG icons configured to trigger on hover or click.
- **Lottie Animations**: Lightweight JSON-based vector animations for hero sections and landing illustrations.
- **Tailwind CSS v4 & Theme Store**: Modern design tokens with OKLCH color space supporting seamless dark/light modes.
- **Pre-configured shadcn/ui**: Built-in accessible accordion, tabs, button, and dialog primitives.
- **Zustand, React Query, & Zod**: Production-ready data fetching, client schema validations, and state stores.

---

## 🛠️ Technology Stack

- **Framework**: [React 19](https://react.dev/)
- **Bundler**: [Vite 8](https://vite.dev/)
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/)
- **Router**: [React Router 7](https://reactrouter.com/)
- **Animation Orchestrator**: [GSAP 3](https://gsap.com/)
- **Gestures & Layouts**: [Framer Motion 12](https://framer.com/motion)
- **Smooth Scroll**: [Lenis 1.3](https://github.com/darkroomengineering/lenis)
- **Animated Vector Icons**: [@lordicon/react](https://lordicon.com/) & [lottie-web](https://github.com/airbnb/lottie-web)
- **Rich Vector Illustrations**: [lottie-react](https://github.com/LottieFiles/lottie-react)
- **State Store**: [Zustand](https://docs.pmnd.rs/zustand)
- **Data Query**: [TanStack Query v5](https://tanstack.com/query)
- **Validation**: [Zod](https://zod.dev/)
- **Forms**: [React Hook Form](https://react-hook-form.com/)

---

## 🚀 Getting Started

### 1. Installation
```bash
bun install
```

### 2. Run Dev Server
```bash
bun run dev
```

### 3. Build Production Bundle
```bash
bun run build
```

---

## 🎨 Interactive Asset Guidelines

### Lordicon Animated Icons
Always use the `<LordIcon />` component located in `src/components/LordIcon.tsx` to display vector-based action icons. It fetches JSON endpoints dynamically and supports triggers (`hover`, `click`, `loop`).

```tsx
import { LordIcon } from '@/components/LordIcon';

<LordIcon 
  src="https://cdn.lordicon.com/wmwqvixz.json" 
  size={24} 
  trigger="hover" 
  colors="primary:currentColor"
/>
```

### Lottie Illustrations
Use the `<LottieAnimation />` component in `src/components/LottieAnimation.tsx` for large background animations or hero graphics.

```tsx
import { LottieAnimation } from '@/components/LottieAnimation';

<LottieAnimation 
  src="https://assets.lottiefiles.com/packages/lf20_kkflmtur.json"
  className="w-64 h-64"
  loop={true}
/>
```

### Motion Preference Accessibility
Both `LordIcon` and `LottieAnimation` check for user-level operating system preferences regarding reduced motion. Auto-loops are disabled when `prefers-reduced-motion` is active to maintain clear visual accessibility guidelines.

---

## 📂 Project Structure

```text
src/
├── api/            # Axios instance and Zod API schema validations
├── assets/         # Static frameworks and logo assets
├── components/     # Reusable UI wrappers and shadcn components
│   ├── ui/         # Radix accessible primitives (Accordion, Tabs, Dialog)
│   ├── LordIcon.tsx  # Dynamic Lordicon vector player
│   └── LottieAnimation.tsx # CDN Lottie animation loader
├── hooks/          # Custom hooks
├── layouts/        # Page layouts (RootLayout coordinates Lenis & GSAP)
├── lib/            # Utilities (logger, class merger)
├── pages/          # Route-level views (Home, About, Contact)
├── providers/      # Context providers (QueryClient)
├── store/          # Zustand states (useThemeStore, useAppStore)
└── types/          # Zod schema models and TypeScript interfaces
```

---

## 📜 License
MIT
"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "components.json".into(),
                content: r#"{
  "$schema": "https://ui.shadcn.com/schema.json",
  "style": "radix-nova",
  "rsc": false,
  "tsx": true,
  "tailwind": {
    "config": "",
    "css": "src/index.css",
    "baseColor": "neutral",
    "cssVariables": true,
    "prefix": ""
  },
  "iconLibrary": "lucide",
  "rtl": false,
  "aliases": {
    "components": "@/components",
    "utils": "@/lib/utils",
    "ui": "@/components/ui",
    "lib": "@/lib",
    "hooks": "@/hooks"
  },
  "menuColor": "default",
  "menuAccent": "subtle",
  "registries": {}
}
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "eslint.config.js".into(),
                content: r#"import js from '@eslint/js'
import globals from 'globals'
import reactHooks from 'eslint-plugin-react-hooks'
import reactRefresh from 'eslint-plugin-react-refresh'
import tseslint from 'typescript-eslint'
import { defineConfig, globalIgnores } from 'eslint/config'

export default defineConfig([
  globalIgnores(['dist']),
  {
    files: ['**/*.{ts,tsx}'],
    extends: [
      js.configs.recommended,
      tseslint.configs.recommended,
      reactHooks.configs.flat.recommended,
      reactRefresh.configs.vite,
    ],
    languageOptions: {
      ecmaVersion: 2020,
      globals: globals.browser,
    },
  },
])
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "index.html".into(),
                content: r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <link rel="icon" type="image/svg+xml" href="/favicon.svg" />
    <meta name="viewport" content="width=device-width, initial-scale=1.0" />
    <title>offpkg vite+react</title>
  </head>
  <body>
    <div id="root"></div>
    <script type="module" src="/src/main.tsx"></script>
  </body>
</html>
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg___eslint__js.md".into(),
                content: r###"# @eslint/js — offpkg docs
> **Version**: 10.0.1 · **Runtime**: bun · **npm**: https://www.npmjs.com/package/@eslint/js  

ESLint JavaScript language implementation
**Homepage**: https://eslint.org

---

> ✏️ Edit this file freely — it lives in ~/.offpkg/docs/bun/@eslint/js.md
> Every project you add @eslint/js to will get YOUR edited version.
> To regenerate from original: `offpkg docs reset @eslint/js --runtime bun`

## My Notes

<!-- Add your own notes, snippets, team conventions here -->

---

## Installation

```json
{ "dependencies": { "@eslint/js": "^10.0.1" } }
```

## Import

```typescript
import ... from '@eslint/js';
```

## Quick Start

```typescript
// Add your usage example here
```

## Links

- npm: https://www.npmjs.com/package/@eslint/js
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg___hookform__resolvers.md".into(),
                content: r###"# @hookform/resolvers

> Schema validation adapters for react-hook-form — Zod, Yup, Valibot, Joi, and more  
> **Version:** 5.2.2 · **Runtime:** bun · [npm](https://www.npmjs.com/package/@hookform/resolvers)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/@hookform/resolvers.md`  
> To regenerate from original: `offpkg docs reset @hookform/resolvers --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add @hookform/resolvers

# Plus your chosen validation library
bun add zod        # recommended
bun add yup
bun add valibot
```

---

## Usage Pattern

```tsx
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'  // swap per library
import * as z from 'zod'

const schema = z.object({ ... })
type FormData = z.infer<typeof schema>

const { register, handleSubmit, formState: { errors } } = useForm<FormData>({
  resolver: zodResolver(schema),
})
```

---

## Zod (recommended)

```tsx
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'

const schema = z.object({
  email: z.string().email('Invalid email'),
  password: z.string().min(8, 'Min 8 characters'),
  age: z.number({ coerce: true }).int().positive(),
})

type FormData = z.infer<typeof schema>

function LoginForm() {
  const { register, handleSubmit, formState: { errors } } = useForm<FormData>({
    resolver: zodResolver(schema),
    defaultValues: { email: '', password: '' },
  })

  return (
    <form onSubmit={handleSubmit(console.log)}>
      <input {...register('email')} />
      {errors.email && <p>{errors.email.message}</p>}

      <input type="password" {...register('password')} />
      {errors.password && <p>{errors.password.message}</p>}

      <input type="number" {...register('age', { valueAsNumber: true })} />
      {errors.age && <p>{errors.age.message}</p>}

      <button type="submit">Submit</button>
    </form>
  )
}
```

---

## Yup

```tsx
import { yupResolver } from '@hookform/resolvers/yup'
import * as yup from 'yup'

const schema = yup.object({
  name: yup.string().required('Required'),
  age: yup.number().required().positive().integer(),
})

useForm({ resolver: yupResolver(schema) })
```

---

## Valibot

```tsx
import { valibotResolver } from '@hookform/resolvers/valibot'
import * as v from 'valibot'

const schema = v.object({
  username: v.pipe(v.string(), v.minLength(3, 'Min 3 chars')),
  password: v.pipe(v.string(), v.nonEmpty('Required')),
})

useForm({ resolver: valibotResolver(schema) })
```

---

## Joi

```tsx
import { joiResolver } from '@hookform/resolvers/joi'
import Joi from 'joi'

const schema = Joi.object({
  name: Joi.string().required(),
  age: Joi.number().required(),
})

useForm({ resolver: joiResolver(schema) })
```

---

## All Supported Resolvers

| Library | Import path | Type inference |
|---------|-------------|----------------|
| Zod | `/zod` | ✅ |
| Yup | `/yup` | ✅ |
| Valibot | `/valibot` | ✅ |
| Joi | `/joi` | ❌ |
| ArkType | `/arktype` | ✅ |
| TypeBox | `/typebox` | ✅ |
| class-validator | `/class-validator` | ✅ |
| Effect | `/effect-ts` | ✅ |
| io-ts | `/io-ts` | ✅ |
| Superstruct | `/superstruct` | ✅ |
| Vest | `/vest` | ❌ |
| VineJS | `/vine` | ✅ |
| Standard Schema | `/standard-schema` | ✅ |

---

## TypeScript — Input vs Output types

When a schema transforms values (e.g. coerces strings to numbers):

```tsx
useForm<z.input<typeof schema>, any, z.output<typeof schema>>({
  resolver: zodResolver(schema),
})
// input type = what the form field receives (string from <input>)
// output type = what handleSubmit receives after transformation (number)
```

---

## Resources

- [GitHub](https://github.com/react-hook-form/resolvers)
- [npm](https://www.npmjs.com/package/@hookform/resolvers)
- [react-hook-form docs](https://react-hook-form.com/docs/useform#resolver)"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg___tailwindcss__vite.md".into(),
                content: r###"# @tailwindcss/vite — offpkg docs
> **Version**: 4.2.2 · **Runtime**: bun · **npm**: https://www.npmjs.com/package/@tailwindcss/vite  

A utility-first CSS framework for rapidly building custom user interfaces.
**Homepage**: https://tailwindcss.com

---

> ✏️ Edit this file freely — it lives in ~/.offpkg/docs/bun/@tailwindcss/vite.md
> Every project you add @tailwindcss/vite to will get YOUR edited version.
> To regenerate from original: `offpkg docs reset @tailwindcss/vite --runtime bun`

## My Notes

<!-- Add your own notes, snippets, team conventions here -->

---

## Installation

```json
{ "dependencies": { "@tailwindcss/vite": "^4.2.2" } }
```

## Import

```typescript
import ... from '@tailwindcss/vite';
```

## Quick Start

```typescript
// Add your usage example here
```

## Links

- npm: https://www.npmjs.com/package/@tailwindcss/vite
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg___tanstack__react-query.md".into(),
                content: r####"# @tanstack/react-query

> Server state management — fetching, caching, syncing async data in React  
> **Version:** 5.95.0 · **Runtime:** bun · [npm](https://www.npmjs.com/package/@tanstack/react-query) · [Docs](https://tanstack.com/query)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/@tanstack/react-query.md`  
> To regenerate from original: `offpkg docs reset @tanstack/react-query --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add @tanstack/react-query
bun add -d @tanstack/react-query-devtools
```

## Import

```ts
import { useQuery, useMutation, useQueryClient, QueryClient, QueryClientProvider } from '@tanstack/react-query'
```

> **Mental model:** TanStack Query owns **server state** (API data). Use `useState`/`zustand` for **client state** (modals, UI toggles).

---

## Setup

```tsx
// main.tsx
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { ReactQueryDevtools } from '@tanstack/react-query-devtools'

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 60 * 1000,     // 1 minute — data is fresh for this long
      retry: 2,                 // retry failed requests twice
    },
  },
})

createRoot(document.getElementById('root')!).render(
  <QueryClientProvider client={queryClient}>
    <App />
    <ReactQueryDevtools initialIsOpen={false} />
  </QueryClientProvider>
)
```

---

## `useQuery` — fetch & cache data

```tsx
const { data, isPending, isError, error, isFetching, refetch } = useQuery({
  queryKey: ['users'],           // cache key — array, treated like a dependency array
  queryFn: () => fetch('/api/users').then(r => r.json()),
})

// With parameter
const { data: user } = useQuery({
  queryKey: ['user', userId],    // changes when userId changes → auto refetch
  queryFn: () => api.getUser(userId),
  enabled: !!userId,             // only run when userId is truthy
})
```

### `useQuery` return values

```ts
data          // T | undefined — the resolved data
isPending     // true while loading for the first time
isFetching    // true whenever a fetch is in flight (including background)
isError       // true if the last fetch failed
error         // Error | null
isSuccess     // true if data is available
dataUpdatedAt // timestamp of last successful fetch
refetch()     // manually trigger a refetch
```

### `useQuery` options

```ts
useQuery({
  queryKey: ['todos'],
  queryFn: fetchTodos,
  staleTime: 1000 * 60 * 5,    // 5 min — don't refetch if data is fresh
  gcTime: 1000 * 60 * 10,      // 10 min — keep in cache after unmount (was cacheTime in v4)
  refetchOnWindowFocus: true,   // refetch when tab regains focus (default: true)
  refetchOnMount: true,         // refetch when component mounts (default: true)
  refetchInterval: 5000,        // poll every 5 seconds (false to disable)
  retry: 3,                     // number of retries on failure
  enabled: true,                // conditionally run the query
  select: (data) => data.items, // transform data — only rerenders if result changes
  placeholderData: [],          // shown while loading (not cached)
  initialData: cachedData,      // treated as real data, skips loading state
})
```

---

## `useMutation` — create, update, delete

```tsx
const mutation = useMutation({
  mutationFn: (newUser: CreateUserDTO) =>
    fetch('/api/users', {
      method: 'POST',
      body: JSON.stringify(newUser),
      headers: { 'Content-Type': 'application/json' },
    }).then(r => r.json()),

  onSuccess: (data) => {
    queryClient.invalidateQueries({ queryKey: ['users'] }) // refetch users list
  },
  onError: (error) => {
    toast.error(error.message)
  },
  onSettled: () => {
    // runs after success or error
  },
})

// Trigger it
mutation.mutate({ name: 'Jane', email: 'jane@example.com' })

// Or async/await
await mutation.mutateAsync({ name: 'Jane' })
```

### `useMutation` states

```ts
mutation.isPending   // true while running
mutation.isSuccess   // true after success
mutation.isError     // true after failure
mutation.isIdle      // true before first call
mutation.data        // T — result data
mutation.error       // Error | null
mutation.variables   // the variables passed to mutate()
mutation.reset()     // reset to idle state
```

---

## `useQueryClient` — imperative cache control

```tsx
const queryClient = useQueryClient()

// Invalidate — mark stale → triggers refetch on next render
queryClient.invalidateQueries({ queryKey: ['users'] })

// Invalidate all queries starting with 'user'
queryClient.invalidateQueries({ queryKey: ['user'] })

// Manually set cache data (e.g. after create/update)
queryClient.setQueryData(['user', id], updatedUser)

// Update part of cached data
queryClient.setQueryData(['users'], (old: User[]) =>
  old.map(u => u.id === id ? { ...u, name: 'Jane' } : u)
)

// Prefetch — load data before navigation
await queryClient.prefetchQuery({
  queryKey: ['user', id],
  queryFn: () => api.getUser(id),
})

// Read cache without subscribing
const users = queryClient.getQueryData<User[]>(['users'])

// Remove from cache
queryClient.removeQueries({ queryKey: ['users'] })

// Cancel in-flight requests
await queryClient.cancelQueries({ queryKey: ['users'] })
```

---

## Query Keys — best practice

```ts
// Treat like a dependency array — include everything the queryFn uses
['users']                           // all users
['user', userId]                    // specific user
['user', userId, 'posts']           // user's posts
['users', { status: 'active' }]    // filtered
['search', { q: 'flutter', page: 1 }]
```

---

## Parallel & Dependent Queries

```tsx
// Parallel — both run at the same time
const usersQuery = useQuery({ queryKey: ['users'], queryFn: fetchUsers })
const postsQuery = useQuery({ queryKey: ['posts'], queryFn: fetchPosts })

// Dependent — waits for userId before running
const { data: user } = useQuery({ queryKey: ['user', id], queryFn: () => fetchUser(id) })
const { data: orders } = useQuery({
  queryKey: ['orders', user?.id],
  queryFn: () => fetchOrders(user!.id),
  enabled: !!user,   // only runs after user is loaded
})
```

---

## Infinite / Paginated Queries

```tsx
import { useInfiniteQuery } from '@tanstack/react-query'

const { data, fetchNextPage, hasNextPage, isFetchingNextPage } = useInfiniteQuery({
  queryKey: ['posts'],
  queryFn: ({ pageParam }) => fetchPosts({ page: pageParam }),
  initialPageParam: 1,
  getNextPageParam: (lastPage, pages) =>
    lastPage.hasMore ? pages.length + 1 : undefined,
})

// data.pages is an array of pages — flatten to render
const allPosts = data?.pages.flatMap(p => p.items) ?? []
```

---

## Optimistic Updates

```tsx
const queryClient = useQueryClient()

const mutation = useMutation({
  mutationFn: updateUser,
  onMutate: async (updated) => {
    await queryClient.cancelQueries({ queryKey: ['user', updated.id] })
    const previous = queryClient.getQueryData<User>(['user', updated.id])
    queryClient.setQueryData(['user', updated.id], updated)  // update cache immediately
    return { previous }  // return for rollback
  },
  onError: (err, updated, context) => {
    // Roll back on failure
    queryClient.setQueryData(['user', updated.id], context?.previous)
  },
  onSettled: (_, __, updated) => {
    queryClient.invalidateQueries({ queryKey: ['user', updated.id] })
  },
})
```

---

## Custom Hook Pattern (recommended)

```ts
// hooks/useUsers.ts
export function useUsers() {
  return useQuery({ queryKey: ['users'], queryFn: api.getUsers })
}

export function useUser(id: string) {
  return useQuery({ queryKey: ['user', id], queryFn: () => api.getUser(id), enabled: !!id })
}

export function useCreateUser() {
  const queryClient = useQueryClient()
  return useMutation({
    mutationFn: api.createUser,
    onSuccess: () => queryClient.invalidateQueries({ queryKey: ['users'] }),
  })
}
```

---

## Resources

- [TanStack Query Docs](https://tanstack.com/query/latest)
- [GitHub](https://github.com/TanStack/query)
- [DevTools](https://tanstack.com/query/latest/docs/framework/react/devtools)
- [v4 → v5 Migration Guide](https://tanstack.com/query/v5/docs/framework/react/guides/migrating-to-v5)"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg___types__node.md".into(),
                content: r####"# @types/node — offpkg docs
> **Version**: 25.5.0 · **Runtime**: bun · **npm**: https://www.npmjs.com/package/@types/node  

TypeScript definitions for node
**Homepage**: https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/node

---

> ✏️ Edit this file freely — it lives in ~/.offpkg/docs/bun/@types/node.md
> Every project you add @types/node to will get YOUR edited version.
> To regenerate from original: `offpkg docs reset @types/node --runtime bun`

## My Notes

<!-- Add your own notes, snippets, team conventions here -->

---

# Installation
> `npm install --save @types/node`

# Summary
This package contains type definitions for node (https://nodejs.org/).

# Details
Files were exported from https://github.com/DefinitelyTyped/DefinitelyTyped/tree/master/types/node.

### Additional Details
 * Last updated: Thu, 12 Mar 2026 15:47:58 GMT
 * Dependencies: [undici-types](https://npmjs.com/package/undici-types)

# Credits
These definitions were written by [Microsoft TypeScript](https://github.com/Microsoft), [Alberto Schiabel](https://github.com/jkomyno), [Andrew Makarov](https://github.com/r3nya), [Benjamin Toueg](https://github.com/btoueg), [David Junger](https://github.com/touffy), [Mohsen Azimi](https://github.com/mohsen1), [Nikita Galkin](https://github.com/galkin), [Sebastian Silbermann](https://github.com/eps1lon), [Wilco Bakker](https://github.com/WilcoBakker), [Marcin Kopacz](https://github.com/chyzwar), [Trivikram Kamat](https://github.com/trivikr), [Junxiao Shi](https://github.com/yoursunny), [Ilia Baryshnikov](https://github.com/qwelias), [ExE Boss](https://github.com/ExE-Boss), [Piotr Błażejewicz](https://github.com/peterblazejewicz), [Anna Henningsen](https://github.com/addaleax), [Victor Perin](https://github.com/victorperin), [NodeJS Contributors](https://github.com/NodeJS), [Linus Unnebäck](https://github.com/LinusU), [wafuwafu13](https://github.com/wafuwafu13), [Matteo Collina](https://github.com/mcollina), [Dmitry Semigradsky](https://github.com/Semigradsky), [René](https://github.com/Renegade334), and [Yagiz Nizipli](https://github.com/anonrig).
"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_axios.md".into(),
                content: r####"# axios

> Promise-based HTTP client for the browser and Node.js  
> **Version:** 1.9.0 · **Runtime:** bun · [npm](https://www.npmjs.com/package/axios) · [Docs](https://axios-http.com/docs/intro)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/axios.md`  
> Every project you add axios to will use YOUR version of this file.  
> To regenerate from original: `offpkg docs reset axios --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add axios
```

## Import

```js
import axios from 'axios'

// Named imports
import axios, { isCancel, AxiosError } from 'axios'

// CommonJS
const axios = require('axios')
```

---

## Usage

### GET

```js
// Simple
const res = await axios.get('/users?id=1')
console.log(res.data)

// With params
const res = await axios.get('/users', { params: { id: 1 } })

// Async/await with error handling
try {
  const res = await axios.get('/users/1')
  console.log(res.data)
} catch (err) {
  console.error(err)
}
```

### POST

```js
const res = await axios.post('/users', {
  name: 'John',
  email: 'john@example.com'
})
```

### PUT / PATCH / DELETE

```js
await axios.put('/users/1', { name: 'Updated' })
await axios.patch('/users/1', { name: 'Patched' })
await axios.delete('/users/1')
```

### Generic config-based call

```js
await axios({
  method: 'post',
  url: '/users',
  data: { name: 'John' }
})
```

### Concurrent requests

```js
const [users, posts] = await Promise.all([
  axios.get('/users'),
  axios.get('/posts')
])
```

---

## Creating an Instance

Use instances to avoid repeating `baseURL`, headers, or timeouts across requests.

```js
const api = axios.create({
  baseURL: 'https://api.example.com',
  timeout: 5000,
  headers: { 'Authorization': `Bearer ${TOKEN}` }
})

// Use it like the global axios
const res = await api.get('/users')
```

---

## Request Config

Only `url` is required. All other fields are optional.

```js
{
  url: '/users',
  method: 'get',                          // default
  baseURL: 'https://api.example.com',
  headers: { 'X-Custom': 'value' },
  params: { id: 1 },                      // appended to URL as query string
  data: { name: 'John' },                 // request body (POST/PUT/PATCH/DELETE)
  timeout: 5000,                          // ms — 0 means no timeout (default)
  responseType: 'json',                   // 'arraybuffer' | 'blob' | 'document' | 'json' | 'text' | 'stream'
  withCredentials: false,                 // send cookies with cross-site requests
  maxRedirects: 5,
  auth: {
    username: 'user',
    password: 'pass'
  },
  validateStatus: (status) => status < 500  // customize which status codes reject
}
```

---

## Response Schema

```js
const res = await axios.get('/users/1')

res.data        // parsed response body
res.status      // HTTP status code (e.g. 200)
res.statusText  // HTTP status message (e.g. "OK")
res.headers     // response headers
res.config      // the config used for the request
res.request     // the underlying request object
```

---

## Config Defaults

Set defaults applied to every request globally or per-instance.

```js
// Global defaults
axios.defaults.baseURL = 'https://api.example.com'
axios.defaults.headers.common['Authorization'] = `Bearer ${TOKEN}`

// Instance defaults
const api = axios.create({ baseURL: 'https://api.example.com' })
api.defaults.headers.common['Authorization'] = `Bearer ${TOKEN}`
```

**Precedence (highest to lowest):**  
per-request config → instance defaults → global defaults

---

## Interceptors

Run logic before every request is sent or after every response is received.

```js
const api = axios.create()

// Request interceptor — runs before the request
api.interceptors.request.use(
  (config) => {
    config.headers['X-Request-Time'] = Date.now()
    return config
  },
  (error) => Promise.reject(error)
)

// Response interceptor — runs after the response
api.interceptors.response.use(
  (response) => response,
  (error) => {
    if (error.response?.status === 401) {
      // handle token refresh or redirect
    }
    return Promise.reject(error)
  }
)
```

**Remove an interceptor:**

```js
const id = api.interceptors.request.use(fn)
api.interceptors.request.eject(id)
```

**Clear all interceptors:**

```js
api.interceptors.request.clear()
api.interceptors.response.clear()
```

> **Note:** Request interceptors run in **reverse order** (LIFO).  
> Response interceptors run in **insertion order** (FIFO).

---

## Error Handling

```js
try {
  await axios.get('/users/1')
} catch (err) {
  if (err.response) {
    // Server responded with a non-2xx status
    console.log(err.response.status)
    console.log(err.response.data)
  } else if (err.request) {
    // Request was made but no response received
    console.log('No response received')
  } else {
    // Error setting up the request
    console.log(err.message)
  }
}
```

**Common error codes:**

| Code | Meaning |
|------|---------|
| `ERR_NETWORK` | Network failure or CORS issue |
| `ECONNABORTED` | Request timed out or was aborted |
| `ETIMEDOUT` | Timeout (when `clarifyTimeoutError: true`) |
| `ERR_CANCELED` | Manually canceled via AbortController |
| `ERR_BAD_REQUEST` | 4xx response |
| `ERR_BAD_RESPONSE` | 5xx or unparseable response |
| `ERR_INVALID_URL` | Malformed URL |

---

## Timeouts

```js
try {
  const res = await axios.get('/slow-endpoint', { timeout: 5000 })
} catch (err) {
  if (axios.isAxiosError(err) && err.code === 'ECONNABORTED') {
    console.error('Request timed out')
  }
}
```

---

## Cancellation

Use the native `AbortController` (recommended):

```js
const controller = new AbortController()

axios.get('/users', { signal: controller.signal })
  .then(res => console.log(res.data))
  .catch(err => {
    if (axios.isCancel(err)) console.log('Canceled')
  })

// Cancel the request
controller.abort()
```

---

## Form Data

### URL-encoded (`application/x-www-form-urlencoded`)

```js
const params = new URLSearchParams({ foo: 'bar', baz: '123' })
await axios.post('/submit', params)
```

**Auto-serialization** — set the content-type header and axios handles the rest:

```js
await axios.post('/submit', { foo: 'bar' }, {
  headers: { 'content-type': 'application/x-www-form-urlencoded' }
})
```

### Multipart (`multipart/form-data`)

```js
const form = new FormData()
form.append('name', 'John')
form.append('file', fileInput.files[0])
await axios.post('/upload', form)
```

**Auto-serialization:**

```js
await axios.post('/upload', { name: 'John' }, {
  headers: { 'Content-Type': 'multipart/form-data' }
})
```

**Shorthand methods:** `axios.postForm`, `axios.putForm`, `axios.patchForm`

```js
await axios.postForm('/upload', { file: fileInput.files[0] })
```

---

## Upload / Download Progress

```js
await axios.post('/upload', data, {
  onUploadProgress: ({ progress, rate, estimated }) => {
    console.log(`${(progress * 100).toFixed(1)}% — ${(rate / 1024).toFixed(1)} KB/s`)
  },
  onDownloadProgress: ({ progress }) => {
    console.log(`Download: ${(progress * 100).toFixed(1)}%`)
  }
})
```

> Progress events are throttled to **3 times per second**.

---

## Rate Limiting *(Node.js only)*

```js
await axios.post('/upload', buffer, {
  maxRate: [100 * 1024, 100 * 1024]  // [uploadLimit, downloadLimit] in bytes/s
})
```

---

## Headers (`AxiosHeaders`)

```js
// Inside a request interceptor
config.headers.set('Authorization', `Bearer ${token}`)
config.headers.setContentType('application/json')

// Disable a header
config.headers.set('User-Agent', false)
```

---

## TypeScript

```ts
import axios, { AxiosResponse, AxiosError } from 'axios'

interface User {
  id: number
  name: string
}

const res: AxiosResponse<User> = await axios.get<User>('/users/1')
console.log(res.data.name)
```

---

## Proxy *(Node.js only)*

```js
await axios.get('/data', {
  proxy: {
    protocol: 'https',
    host: '127.0.0.1',
    port: 9000,
    auth: { username: 'user', password: 'pass' }
  }
})
```

---

## Resources

- [Official Docs](https://axios-http.com/docs/intro)
- [GitHub](https://github.com/axios/axios)
- [Changelog](https://github.com/axios/axios/blob/main/CHANGELOG.md)"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_eslint-plugin-react-hooks.md".into(),
                content: r###"# eslint-plugin-react-hooks — offpkg docs
> **Version**: 7.0.1 · **Runtime**: bun · **npm**: https://www.npmjs.com/package/eslint-plugin-react-hooks  

ESLint rules for React Hooks
**Homepage**: https://react.dev/

---

> ✏️ Edit this file freely — it lives in ~/.offpkg/docs/bun/eslint-plugin-react-hooks.md
> Every project you add eslint-plugin-react-hooks to will get YOUR edited version.
> To regenerate from original: `offpkg docs reset eslint-plugin-react-hooks --runtime bun`

## My Notes

<!-- Add your own notes, snippets, team conventions here -->

---

## Installation

```json
{ "dependencies": { "eslint-plugin-react-hooks": "^7.0.1" } }
```

## Import

```typescript
import ... from 'eslint-plugin-react-hooks';
```

## Quick Start

```typescript
// Add your usage example here
```

## Links

- npm: https://www.npmjs.com/package/eslint-plugin-react-hooks
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_eslint-plugin-react-refresh.md".into(),
                content: r####"# eslint-plugin-react-refresh — offpkg docs
> **Version**: 0.5.2 · **Runtime**: bun · **npm**: https://www.npmjs.com/package/eslint-plugin-react-refresh  

Validate that your components can safely be updated with Fast Refresh
**Homepage**: https://github.com/ArnaudBarre/eslint-plugin-react-refresh#readme

---

> ✏️ Edit this file freely — it lives in ~/.offpkg/docs/bun/eslint-plugin-react-refresh.md
> Every project you add eslint-plugin-react-refresh to will get YOUR edited version.
> To regenerate from original: `offpkg docs reset eslint-plugin-react-refresh --runtime bun`

## My Notes

<!-- Add your own notes, snippets, team conventions here -->

---

# eslint-plugin-react-refresh [![npm](https://img.shields.io/npm/v/eslint-plugin-react-refresh)](https://www.npmjs.com/package/eslint-plugin-react-refresh)

Validate that your components can safely be updated with Fast Refresh.

## Explainer

"Fast Refresh", also known as "hot reloading", is a feature in many modern bundlers.
If you update some React component(s) on disk, then the bundler will know to update only the impacted parts of your page -- without a full page reload.

`eslint-plugin-react-refresh` enforces that your components are structured in a way that integrations such as [react-refresh](https://www.npmjs.com/package/react-refresh) expect.

### Limitations

⚠️ To avoid false positives, by default this plugin is only applied on `tsx` & `jsx` files. See [Options](#options) to run on JS files. ⚠️

The plugin relies on naming conventions (i.e. use PascalCase for components, camelCase for util functions). This is why there are some limitations:

- `export *` are not supported and will be reported as an error
- Anonymous function are not supported (i.e `export default function() {}`)
- Class components are not supported
- All-uppercase function export is considered an error when not using direct named export (ex `const CMS = () => <></>; export { CMS }`)

## Installation

```sh
npm i -D eslint-plugin-react-refresh
```

## Usage

This plugin provides a single rule, `react-refresh/only-export-components`. There are multiple ways to enable it.

### Recommended config

```js
import { defineConfig } from "eslint/config";
import { reactRefresh } from "eslint-plugin-react-refresh";

export default defineConfig(
  /* Main config */
  reactRefresh.configs.recommended(),
);
```

### Vite config

This enables the `allowConstantExport` option which is supported by Vite React plugins.

```js
import { defineConfig } from "eslint/config";
import { reactRefresh } from "eslint-plugin-react-refresh";

export default defineConfig(
  /* Main config */
  reactRefresh.configs.vite(),
);
```

### Next config

This allows exports like `fetchCache` and `revalidate` which are used in Page or Layout components and don't trigger a full page reload.

```js
import { defineConfig } from "eslint/config";
import { reactRefresh } from "eslint-plugin-react-refresh";

export default defineConfig(
  /* Main config */
  reactRefresh.configs.next(),
);
```

### Without config

```js
import { defineConfig } from "eslint/config";
import { reactRefresh } from "eslint-plugin-react-refresh";

export default defineConfig({
  // in main config for TSX/JSX source files
  plugins: {
    "react-refresh": reactRefresh.plugin,
  },
  rules: {
    "react-refresh/only-export-components": "error",
  },
});
```

## Examples

These examples are from enabling `react-refresh/only-exports-components`.

### Fail

```jsx
export const foo = () => {};
export const Bar = () => <></>;
```

```jsx
export default function () {}
export default compose()(MainComponent)
```

```jsx
export * from "./foo";
```

```jsx
const Tab = () => {};
export const tabs = [<Tab />, <Tab />];
```

```jsx
const App = () => {};
createRoot(document.getElementById("root")).render(<App />);
```

### Pass

```jsx
export default function Foo() {
  return <></>;
}
```

```jsx
const foo = () => {};
export const Bar = () => <></>;
```

```jsx
import { App } from "./App";
createRoot(document.getElementById("root")).render(<App />);
```

## Options

These options are all present on `react-refresh/only-exports-components`.

```ts
interface Options {
  extraHOCs?: string[];
  allowExportNames?: string[];
  allowConstantExport?: boolean;
  checkJS?: boolean;
}

const defaultOptions: Options = {
  extraHOCs: [],
  allowExportNames: [],
  allowConstantExport: false,
  checkJS: false,
};
```

### extraHOCs <small>(v0.5.0)</small>

If you're exporting components wrapped in non built-in React HOC (memo, forwardRef, lazy), you can use this option to avoid false positives.

```json
{
  "react-refresh/only-export-components": [
    "error",
    { "extraHOCs": ["observer", "withAuth"] }
  ]
}
```

### allowExportNames <small>(v0.4.4)</small>

> Default: `[]`

If you use a framework that handles HMR of some specific exports, you can use this option to avoid warning for them.

Example for [Remix](https://remix.run/docs/en/main/discussion/hot-module-replacement#supported-exports):

```json
{
  "react-refresh/only-export-components": [
    "error",
    { "allowExportNames": ["meta", "links", "headers", "loader", "action"] }
  ]
}
```

### allowConstantExport <small>(v0.4.0)</small>

> Default: `false` (`true` in `vite` config)

Don't warn when a constant (string, number, boolean, templateLiteral) is exported aside one or more components.

This should be enabled if the fast refresh implementation correctly handles this case (HMR when the constant doesn't change, propagate update to importers when the constant changes.). Vite supports it, PR welcome if you notice other integrations works well.

```json
{
  "react-refresh/only-export-components": [
    "error",
    { "allowConstantExport": true }
  ]
}
```

Enabling this option allows code such as the following:

```jsx
export const CONSTANT = 3;
export const Foo = () => <></>;
```

### checkJS <small>(v0.3.3)</small>

> Default: `false`

If you're using JSX inside `.js` files (which I don't recommend because it forces you to configure every tool you use to switch the parser), you can still use the plugin by enabling this option. To reduce the number of false positive, only files importing `react` are checked.

```json
{
  "react-refresh/only-export-components": ["error", { "checkJS": true }]
}
```
"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_eslint.md".into(),
                content: r####"# eslint — offpkg docs
> **Version**: 10.1.0 · **Runtime**: bun · **npm**: https://www.npmjs.com/package/eslint  

An AST-based pattern checker for JavaScript.
**Homepage**: https://eslint.org

---

> ✏️ Edit this file freely — it lives in ~/.offpkg/docs/bun/eslint.md
> Every project you add eslint to will get YOUR edited version.
> To regenerate from original: `offpkg docs reset eslint --runtime bun`

## My Notes

<!-- Add your own notes, snippets, team conventions here -->

---

[![npm version](https://img.shields.io/npm/v/eslint.svg)](https://www.npmjs.com/package/eslint)
[![Downloads](https://img.shields.io/npm/dm/eslint.svg)](https://www.npmjs.com/package/eslint)
[![Build Status](https://github.com/eslint/eslint/workflows/CI/badge.svg)](https://github.com/eslint/eslint/actions)
<br>
[![Open Collective Backers](https://img.shields.io/opencollective/backers/eslint)](https://opencollective.com/eslint)
[![Open Collective Sponsors](https://img.shields.io/opencollective/sponsors/eslint)](https://opencollective.com/eslint)

# ESLint

[Website](https://eslint.org) |
[Configure ESLint](https://eslint.org/docs/latest/use/configure) |
[Rules](https://eslint.org/docs/rules/) |
[Contribute to ESLint](https://eslint.org/docs/latest/contribute) |
[Report Bugs](https://eslint.org/docs/latest/contribute/report-bugs) |
[Code of Conduct](https://eslint.org/conduct) |
[X](https://x.com/geteslint) |
[Discord](https://eslint.org/chat) |
[Mastodon](https://fosstodon.org/@eslint) |
[Bluesky](https://bsky.app/profile/eslint.org)

ESLint is a tool for identifying and reporting on patterns found in ECMAScript/JavaScript code. In many ways, it is similar to JSLint and JSHint with a few exceptions:

- ESLint uses [Espree](https://github.com/eslint/js/tree/main/packages/espree) for JavaScript parsing.
- ESLint uses an AST to evaluate patterns in code.
- ESLint is completely pluggable, every single rule is a plugin and you can add more at runtime.

## Table of Contents

1. [Installation and Usage](#installation-and-usage)
1. [Configuration](#configuration)
1. [Version Support](#version-support)
1. [Code of Conduct](#code-of-conduct)
1. [Filing Issues](#filing-issues)
1. [Frequently Asked Questions](#frequently-asked-questions)
1. [Releases](#releases)
1. [Security Policy](#security-policy)
1. [Semantic Versioning Policy](#semantic-versioning-policy)
1. [ESM Dependencies](#esm-dependencies)
1. [License](#license)
1. [Team](#team)
1. [Sponsors](#sponsors)
1. [Technology Sponsors](#technology-sponsors) <!-- markdownlint-disable-line MD051 -->

## Installation and Usage

### Prerequisites

To use ESLint, you must have [Node.js](https://nodejs.org/) (`^20.19.0`, `^22.13.0`, or `>=24`) installed and built with SSL support. (If you are using an official Node.js distribution, SSL is always built in.)

If you use ESLint's TypeScript type definitions, TypeScript 5.3 or later is required.

### npm Installation

You can install and configure ESLint using this command:

```shell
npm init @eslint/config@latest
```

After that, you can run ESLint on any file or directory like this:

```shell
npx eslint yourfile.js
```

### pnpm Installation

To use ESLint with pnpm, we recommend setting up a `.npmrc` file with at least the following settings:

```text
auto-install-peers=true
node-linker=hoisted
```

This ensures that pnpm installs dependencies in a way that is more compatible with npm and is less likely to produce errors.

## Configuration

You can configure rules in your `eslint.config.js` files as in this example:

```js
import { defineConfig } from "eslint/config";

export default defineConfig([
	{
		files: ["**/*.js", "**/*.cjs", "**/*.mjs"],
		rules: {
			"prefer-const": "warn",
			"no-constant-binary-expression": "error",
		},
	},
]);
```

The names `"prefer-const"` and `"no-constant-binary-expression"` are the names of [rules](https://eslint.org/docs/rules) in ESLint. The first value is the error level of the rule and can be one of these values:

- `"off"` or `0` - turn the rule off
- `"warn"` or `1` - turn the rule on as a warning (doesn't affect exit code)
- `"error"` or `2` - turn the rule on as an error (exit code will be 1)

The three error levels allow you fine-grained control over how ESLint applies rules (for more configuration options and details, see the [configuration docs](https://eslint.org/docs/latest/use/configure)).

## Version Support

The ESLint team provides ongoing support for the current version and six months of limited support for the previous version. Limited support includes critical bug fixes, security issues, and compatibility issues only.

ESLint offers commercial support for both current and previous versions through our partners, [Tidelift][tidelift] and [HeroDevs][herodevs].

See [Version Support](https://eslint.org/version-support) for more details.

## Code of Conduct

ESLint adheres to the [OpenJS Foundation Code of Conduct](https://eslint.org/conduct).

## Filing Issues

Before filing an issue, please be sure to read the guidelines for what you're reporting:

- [Bug Report](https://eslint.org/docs/latest/contribute/report-bugs)
- [Propose a New Rule](https://eslint.org/docs/latest/contribute/propose-new-rule)
- [Proposing a Rule Change](https://eslint.org/docs/latest/contribute/propose-rule-change)
- [Request a Change](https://eslint.org/docs/latest/contribute/request-change)

## Frequently Asked Questions

### Does ESLint support JSX?

Yes, ESLint natively supports parsing JSX syntax (this must be enabled in [configuration](https://eslint.org/docs/latest/use/configure)). Please note that supporting JSX syntax _is not_ the same as supporting React. React applies specific semantics to JSX syntax that ESLint doesn't recognize. We recommend using [eslint-plugin-react](https://www.npmjs.com/package/eslint-plugin-react) if you are using React and want React semantics.

### Does Prettier replace ESLint?

No, ESLint and Prettier have different jobs: ESLint is a linter (looking for problematic patterns) and Prettier is a code formatter. Using both tools is common, refer to [Prettier's documentation](https://prettier.io/docs/en/install#eslint-and-other-linters) to learn how to configure them to work well with each other.

### What ECMAScript versions does ESLint support?

ESLint has full support for ECMAScript 3, 5, and every year from 2015 up until the most recent stage 4 specification (the default). You can set your desired ECMAScript syntax and other settings (like global variables) through [configuration](https://eslint.org/docs/latest/use/configure).

### What about experimental features?

ESLint's parser only officially supports the latest final ECMAScript standard. We will make changes to core rules in order to avoid crashes on stage 3 ECMAScript syntax proposals (as long as they are implemented using the correct experimental ESTree syntax). We may make changes to core rules to better work with language extensions (such as JSX, Flow, and TypeScript) on a case-by-case basis.

In other cases (including if rules need to warn on more or fewer cases due to new syntax, rather than just not crashing), we recommend you use other parsers and/or rule plugins. If you are using Babel, you can use [@babel/eslint-parser](https://www.npmjs.com/package/@babel/eslint-parser) and [@babel/eslint-plugin](https://www.npmjs.com/package/@babel/eslint-plugin) to use any option available in Babel.

Once a language feature has been adopted into the ECMAScript standard (stage 4 according to the [TC39 process](https://tc39.github.io/process-document/)), we will accept issues and pull requests related to the new feature, subject to our [contributing guidelines](https://eslint.org/docs/latest/contribute). Until then, please use the appropriate parser and plugin(s) for your experimental feature.

### Which Node.js versions does ESLint support?

ESLint updates the supported Node.js versions with each major release of ESLint. At that time, ESLint's supported Node.js versions are updated to be:

1. The most recent maintenance release of Node.js
1. The lowest minor version of the Node.js LTS release that includes the features the ESLint team wants to use.
1. The Node.js Current release

ESLint is also expected to work with Node.js versions released after the Node.js Current release.

Refer to the [Quick Start Guide](https://eslint.org/docs/latest/use/getting-started#prerequisites) for the officially supported Node.js versions for a given ESLint release.

### Where to ask for help?

Open a [discussion](https://github.com/eslint/eslint/discussions) or stop by our [Discord server](https://eslint.org/chat).

### Why doesn't ESLint lock dependency versions?

Lock files like `package-lock.json` are helpful for deployed applications. They ensure that dependencies are consistent between environments and across deployments.

Packages like `eslint` that get published to the npm registry do not include lock files. `npm install eslint` as a user will respect version constraints in ESLint's `package.json`. ESLint and its dependencies will be included in the user's lock file if one exists, but ESLint's own lock file would not be used.

We intentionally don't lock dependency versions so that we have the latest compatible dependency versions in development and CI that our users get when installing ESLint in a project.

The Twilio blog has a [deeper dive](https://www.twilio.com/blog/lockfiles-nodejs) to learn more.

## Releases

We have scheduled releases every two weeks on Friday or Saturday. You can follow a [release issue](https://github.com/eslint/eslint/issues?q=is%3Aopen+is%3Aissue+label%3Arelease) for updates about the scheduling of any particular release.

## Security Policy

ESLint takes security seriously. We work hard to ensure that ESLint is safe for everyone and that security issues are addressed quickly and responsibly. Read the full [security policy](https://github.com/eslint/.github/blob/master/SECURITY.md).

## Semantic Versioning Policy

ESLint follows [semantic versioning](https://semver.org). However, due to the nature of ESLint as a code quality tool, it's not always clear when a minor or major version bump occurs. To help clarify this for everyone, we've defined the following semantic versioning policy for ESLint:

- Patch release (intended to not break your lint build)
    - A bug fix in a rule that results in ESLint reporting fewer linting errors.
    - A bug fix to the CLI or core (including formatters).
    - Improvements to documentation.
    - Non-user-facing changes such as refactoring code, adding, deleting, or modifying tests, and increasing test coverage.
    - Re-releasing after a failed release (i.e., publishing a release that doesn't work for anyone).
- Minor release (might break your lint build)
    - A bug fix in a rule that results in ESLint reporting more linting errors.
    - A new rule is created.
    - A new option to an existing rule that does not result in ESLint reporting more linting errors by default.
    - A new addition to an existing rule to support a newly-added language feature (within the last 12 months) that will result in ESLint reporting more linting errors by default.
    - An existing rule is deprecated.
    - A new CLI capability is created.
    - New capabilities to the public API are added (new classes, new methods, new arguments to existing methods, etc.).
    - A new formatter is created.
    - `eslint:recommended` is updated and will result in strictly fewer linting errors (e.g., rule removals).
- Major release (likely to break your lint build)
    - `eslint:recommended` is updated and may result in new linting errors (e.g., rule additions, most rule option updates).
    - A new option to an existing rule that results in ESLint reporting more linting errors by default.
    - An existing formatter is removed.
    - Part of the public API is removed or changed in an incompatible way. The public API includes:
        - Rule schemas
        - Configuration schema
        - Command-line options
        - Node.js API
        - Rule, formatter, parser, plugin APIs

According to our policy, any minor update may report more linting errors than the previous release (ex: from a bug fix). As such, we recommend using the tilde (`~`) in `package.json` e.g. `"eslint": "~3.1.0"` to guarantee the results of your builds.

## ESM Dependencies

Since ESLint is a CommonJS package, there are restrictions on which ESM-only packages can be used as dependencies.

Packages that are controlled by the ESLint team and have no external dependencies can be safely loaded synchronously using [`require(esm)`](https://nodejs.org/api/modules.html#loading-ecmascript-modules-using-require) and therefore used in any contexts.

For external packages, we don't use `require(esm)` because a package could add a top-level `await` and thus break ESLint. We can use an external ESM-only package only in case it is needed only in asynchronous code, in which case it can be loaded using dynamic `import()`.

## License

MIT License

Copyright OpenJS Foundation and other contributors, <www.openjsf.org>

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in
all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
THE SOFTWARE.

## Team

These folks keep the project moving and are resources for help.

<!-- NOTE: This section is autogenerated. Do not manually edit.-->

<!--teamstart-->

### Technical Steering Committee (TSC)

The people who manage releases, review feature requests, and meet regularly to ensure ESLint is properly maintained.

<table><tbody><tr><td align="center" valign="top" width="11%">
<a href="https://github.com/nzakas">
<img src="https://github.com/nzakas.png?s=75" width="75" height="75" alt="Nicholas C. Zakas's Avatar"><br />
Nicholas C. Zakas
</a>
</td><td align="center" valign="top" width="11%">
<a href="https://github.com/fasttime">
<img src="https://github.com/fasttime.png?s=75" width="75" height="75" alt="Francesco Trotta's Avatar"><br />
Francesco Trotta
</a>
</td><td align="center" valign="top" width="11%">
<a href="https://github.com/mdjermanovic">
<img src="https://github.com/mdjermanovic.png?s=75" width="75" height="75" alt="Milos Djermanovic's Avatar"><br />
Milos Djermanovic
</a>
</td></tr></tbody></table>

### Reviewers

The people who review and implement new features.

<table><tbody><tr><td align="center" valign="top" width="11%">
<a href="https://github.com/aladdin-add">
<img src="https://github.com/aladdin-add.png?s=75" width="75" height="75" alt="唯然's Avatar"><br />
唯然
</a>
</td><td align="center" valign="top" width="11%">
<a href="https://github.com/snitin315">
<img src="https://github.com/snitin315.png?s=75" width="75" height="75" alt="Nitin Kumar's Avatar"><br />
Nitin Kumar
</a>
</td></tr></tbody></table>

### Committers

The people who review and fix bugs and help triage issues.

<table><tbody><tr><td align="center" valign="top" width="11%">
<a href="https://github.com/DMartens">
<img src="https://github.com/DMartens.png?s=75" width="75" height="75" alt="fnx's Avatar"><br />
fnx
</a>
</td><td align="center" valign="top" width="11%">
<a href="https://github.com/JoshuaKGoldberg">
<img src="https://github.com/JoshuaKGoldberg.png?s=75" width="75" height="75" alt="Josh Goldberg ✨'s Avatar"><br />
Josh Goldberg ✨
</a>
</td><td align="center" valign="top" width="11%">
<a href="https://github.com/SwetaTanwar">
<img src="https://github.com/SwetaTanwar.png?s=75" width="75" height="75" alt="Sweta Tanwar's Avatar"><br />
Sweta Tanwar
</a>
</td><td align="center" valign="top" width="11%">
<a href="https://github.com/Tanujkanti4441">
<img src="https://github.com/Tanujkanti4441.png?s=75" width="75" height="75" alt="Tanuj Kanti's Avatar"><br />
Tanuj Kanti
</a>
</td><td align="center" valign="top" width="11%">
<a href="https://github.com/lumirlumir">
<img src="https://github.com/lumirlumir.png?s=75" width="75" height="75" alt="루밀LuMir's Avatar"><br />
루밀LuMir
</a>
</td><td align="center" valign="top" width="11%">
<a href="https://github.com/Pixel998">
<img src="https://github.com/Pixel998.png?s=75" width="75" height="75" alt="Pixel998's Avatar"><br />
Pixel998
</a>
</td></tr></tbody></table>

### Website Team

Team members who focus specifically on eslint.org

<table><tbody><tr><td align="center" valign="top" width="11%">
<a href="https://github.com/amareshsm">
<img src="https://github.com/amareshsm.png?s=75" width="75" height="75" alt="Amaresh  S M's Avatar"><br />
Amaresh  S M
</a>
</td><td align="center" valign="top" width="11%">
<a href="https://github.com/harish-sethuraman">
<img src="https://github.com/harish-sethuraman.png?s=75" width="75" height="75" alt="Harish's Avatar"><br />
Harish
</a>
</td><td align="center" valign="top" width="11%">
<a href="https://github.com/kecrily">
<img src="https://github.com/kecrily.png?s=75" width="75" height="75" alt="Percy Ma's Avatar"><br />
Percy Ma
</a>
</td></tr></tbody></table>

<!--teamend-->

<!-- NOTE: This section is autogenerated. Do not manually edit.-->
<!--sponsorsstart-->

## Sponsors

The following companies, organizations, and individuals support ESLint's ongoing maintenance and development. [Become a Sponsor](https://eslint.org/donate)
to get your logo on our READMEs and [website](https://eslint.org/sponsors).

<h3>Platinum Sponsors</h3>
<p><a href="https://automattic.com"><img src="https://images.opencollective.com/automattic/d0ef3e1/logo.png" alt="Automattic" height="128"></a></p><h3>Gold Sponsors</h3>
<p><a href="https://qlty.sh/"><img src="https://images.opencollective.com/qltysh/33d157d/logo.png" alt="Qlty Software" height="96"></a></p><h3>Silver Sponsors</h3>
<p><a href="https://vite.dev/"><img src="https://images.opencollective.com/vite/d472863/logo.png" alt="Vite" height="64"></a> <a href="https://liftoff.io/"><img src="https://images.opencollective.com/liftoff/2d6c3b6/logo.png" alt="Liftoff" height="64"></a> <a href="https://stackblitz.com"><img src="https://avatars.githubusercontent.com/u/28635252" alt="StackBlitz" height="64"></a></p><h3>Bronze Sponsors</h3>
<p><a href="https://syntax.fm"><img src="https://github.com/syntaxfm.png" alt="Syntax" height="32"></a> <a href="https://cybozu.co.jp/"><img src="https://images.opencollective.com/cybozu/933e46d/logo.png" alt="Cybozu" height="32"></a> <a href="https://opensource.sap.com"><img src="https://avatars.githubusercontent.com/u/2531208" alt="SAP" height="32"></a> <a href="https://www.crawljobs.com/"><img src="https://images.opencollective.com/crawljobs-poland/fa43a17/logo.png" alt="CrawlJobs" height="32"></a> <a href="https://depot.dev"><img src="https://images.opencollective.com/depot/39125a1/logo.png" alt="Depot" height="32"></a> <a href="https://www.n-ix.com/"><img src="https://images.opencollective.com/n-ix-ltd/575a7a5/logo.png" alt="N-iX Ltd" height="32"></a> <a href="https://icons8.com/"><img src="https://images.opencollective.com/icons8/7fa1641/logo.png" alt="Icons8" height="32"></a> <a href="https://discord.com"><img src="https://images.opencollective.com/discordapp/f9645d9/logo.png" alt="Discord" height="32"></a> <a href="https://www.gitbook.com"><img src="https://avatars.githubusercontent.com/u/7111340" alt="GitBook" height="32"></a> <a href="https://herocoders.com"><img src="https://avatars.githubusercontent.com/u/37549774" alt="HeroCoders" height="32"></a> <a href="https://www.lambdatest.com"><img src="https://avatars.githubusercontent.com/u/171592363" alt="TestMu AI Open Source Office (Formerly LambdaTest)" height="32"></a></p>
<h3>Technology Sponsors</h3>
Technology sponsors allow us to use their products and services for free as part of a contribution to the open source ecosystem and our work.
<p><a href="https://netlify.com"><img src="https://raw.githubusercontent.com/eslint/eslint.org/main/src/assets/images/techsponsors/netlify-icon.svg" alt="Netlify" height="32"></a> <a href="https://algolia.com"><img src="https://raw.githubusercontent.com/eslint/eslint.org/main/src/assets/images/techsponsors/algolia-icon.svg" alt="Algolia" height="32"></a> <a href="https://1password.com"><img src="https://raw.githubusercontent.com/eslint/eslint.org/main/src/assets/images/techsponsors/1password-icon.svg" alt="1Password" height="32"></a></p>

<!--sponsorsend-->

[tidelift]: https://tidelift.com/funding/github/npm/eslint
[herodevs]: https://www.herodevs.com/support/eslint-nes?utm_source=ESLintWebsite&utm_medium=ESLintWebsite&utm_campaign=ESLintNES&utm_id=ESLintNES
"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_framer-motion.md".into(),
                content: r####"# framer-motion

> Production-ready motion and physics gesture library for React  
> **Version:** 12.40.0 · **Runtime:** bun · [npm](https://www.npmjs.com/package/framer-motion) · [Docs](https://framer.com/motion)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/framer-motion.md`  
> To regenerate from original: `offpkg docs reset framer-motion --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add framer-motion
```

---

## Usage

### Simple Transitions
Wrap any HTML element with `motion.` to enable animated properties:

```tsx
import { motion } from 'framer-motion';

export default function FadeIn() {
  return (
    <motion.div
      initial={{ opacity: 0, y: 20 }}
      animate={{ opacity: 1, y: 0 }}
      transition={{ duration: 0.6 }}
    >
      Hello Motion
    </motion.div>
  );
}
```

### Drag Gestures
Framer Motion handles touch and drag gestures with absolute ease:

```tsx
<motion.div
  drag
  dragConstraints={{ left: -100, right: 100, top: -50, bottom: 50 }}
  dragElastic={0.15}
  whileDrag={{ scale: 1.1, cursor: "grabbing" }}
  transition={{ type: "spring", stiffness: 300, damping: 20 }}
  className="w-16 h-16 bg-primary rounded-full cursor-grab"
/>
```

### Exit Animations (AnimatePresence)
```tsx
import { motion, AnimatePresence } from 'framer-motion';

export default function ToggleModal({ isOpen }: { isOpen: boolean }) {
  return (
    <AnimatePresence>
      {isOpen && (
        <motion.div
          initial={{ opacity: 0, scale: 0.9 }}
          animate={{ opacity: 1, scale: 1 }}
          exit={{ opacity: 0, scale: 0.9 }}
          className="modal"
        >
          Modal Content
        </motion.div>
      )}
    </AnimatePresence>
  );
}
```
"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_globals.md".into(),
                content: r###"# globals — offpkg docs
> **Version**: 17.4.0 · **Runtime**: bun · **npm**: https://www.npmjs.com/package/globals  

Global identifiers from different JavaScript environments
**Homepage**: https://github.com/sindresorhus/globals#readme

---

> ✏️ Edit this file freely — it lives in ~/.offpkg/docs/bun/globals.md
> Every project you add globals to will get YOUR edited version.
> To regenerate from original: `offpkg docs reset globals --runtime bun`

## My Notes

<!-- Add your own notes, snippets, team conventions here -->

---

# globals

> Global identifiers from different JavaScript environments

It's just a [JSON file](globals.json), so you can use it in any environment.

This package is used by ESLint 8 and earlier. For ESLint 9 and later, you should depend on this package directly in [your ESLint config](https://eslint.org/docs/latest/use/configure/language-options#predefined-global-variables).

## Install

```sh
npm install globals
```

## Usage

```js
import globals from 'globals';

console.log(globals.browser);
/*
{
	addEventListener: false,
	applicationCache: false,
	ArrayBuffer: false,
	atob: false,
	…
}
*/
```

Each global is given a value of `true` or `false`. A value of `true` indicates that the variable may be overwritten. A value of `false` indicates that the variable should be considered read-only. This information is used by static analysis tools to flag incorrect behavior. We assume all variables should be `false` unless we hear otherwise.

For Node.js this package provides two sets of globals:

- `globals.nodeBuiltin`: Globals available to all code running in Node.js.
	These will usually be available as properties on the `globalThis` object and include `process`, `Buffer`, but not CommonJS arguments like `require`.
	See: https://nodejs.org/api/globals.html
- `globals.node`: A combination of the globals from `nodeBuiltin` plus all CommonJS arguments ("CommonJS module scope").
	See: https://nodejs.org/api/modules.html#modules_the_module_scope

When analyzing code that is known to run outside of a CommonJS wrapper, for example, JavaScript modules, `nodeBuiltin` can find accidental CommonJS references.
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_gsap.md".into(),
                content: r####"# gsap

> Professional-grade animation and scroll-triggered timelines for modern web apps  
> **Version:** 3.15.0 · **Runtime:** bun · [npm](https://www.npmjs.com/package/gsap) · [Docs](https://gsap.com/docs/v3/)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/gsap.md`  
> To regenerate from original: `offpkg docs reset gsap --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add gsap
```

---

## Usage

### Register Plugins
Always register `ScrollTrigger` or other plugins at your application root:

```typescript
import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

gsap.registerPlugin(ScrollTrigger);
```

### Simple Tween
```typescript
gsap.to(".box", { x: 200, duration: 1, ease: "power2.out" });
```

### Timelines
```typescript
const tl = gsap.timeline({ defaults: { ease: "power3.inOut" } });

tl.to(".title", { y: 0, opacity: 1, duration: 0.6 })
  .to(".card", { scale: 1, opacity: 1, stagger: 0.1, duration: 0.4 }, "-=0.3");
```

### ScrollTrigger
```typescript
gsap.to(".card-reveal", {
  opacity: 1,
  y: 0,
  stagger: 0.15,
  scrollTrigger: {
    trigger: ".grid-container",
    start: "top 80%", // plays when container top hits 80% of viewport height
    toggleActions: "play none none none",
  }
});
```
"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_lenis.md".into(),
                content: r####"# lenis

> High-performance smooth scroll library for modern browsers  
> **Version:** 1.3.23 · **Runtime:** bun · [npm](https://www.npmjs.com/package/lenis) · [Docs](https://github.com/darkroomengineering/lenis)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/lenis.md`  
> To regenerate from original: `offpkg docs reset lenis --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add lenis
```

---

## Usage

### Simple React Setup
Initialize Lenis in a high-level component (like `RootLayout`):

```tsx
import { useEffect } from 'react';
import Lenis from 'lenis';

export function ScrollWrapper() {
  useEffect(() => {
    const lenis = new Lenis({
      autoRaf: true, // Auto runs requestAnimationFrame
      duration: 1.2,
      easing: (t) => Math.min(1, 1.001 - Math.pow(2, -10 * t)),
    });

    return () => {
      lenis.destroy();
    };
  }, []);

  return null;
}
```

### Syncing with GSAP ScrollTrigger
To ensure ScrollTrigger behaves correctly with smooth inertia scrolling:

```typescript
import { useEffect } from 'react';
import Lenis from 'lenis';
import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

gsap.registerPlugin(ScrollTrigger);

useEffect(() => {
  const lenis = new Lenis({ autoRaf: true });

  // Update ScrollTrigger coordinates on scroll
  lenis.on('scroll', ScrollTrigger.update);

  return () => {
    lenis.destroy();
  };
}, []);
```
"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_lordicons.md".into(),
                content: r####"# lordicons

> Highly-interactive, vector-based animated icons for user interfaces  
> **Version:** 1.11.0 · **Runtime:** bun · [npm](https://www.npmjs.com/package/@lordicon/react) · [Docs](https://lordicon.com/docs/react)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/lordicons.md`  
> To regenerate from original: `offpkg docs reset lordicons --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add @lordicon/react lottie-web
```

---

## Usage

### Dynamic Lordicon Loader
Create a wrapper to dynamically fetch JSON data from URLs or CDN endpoints:

```tsx
import { useEffect, useRef, useState } from 'react';
import { Player } from '@lordicon/react';

export function LordIcon({ src, size = 32, trigger = 'hover' }) {
  const ref = useRef<Player>(null);
  const [data, setData] = useState(null);

  useEffect(() => {
    fetch(src).then(res => res.json()).then(setData);
  }, [src]);

  const onEnter = () => trigger === 'hover' && ref.current?.playFromBeginning();

  if (!data) return <div style={{ width: size, height: size }} className="animate-pulse bg-muted/20" />;

  return (
    <div onMouseEnter={onEnter} style={{ width: size, height: size }}>
      <Player ref={ref} icon={data} size={size} />
    </div>
  );
}
```

### Supported Trigger Constants
Triggers indicate when and how the vector animation plays:
*   `hover` — Plays from beginning once when hovered.
*   `click` — Plays once when clicked.
*   `loop` — Loops constantly.
*   `morph` — Transforms dynamically based on state transitions.
"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_lottie.md".into(),
                content: r####"# lottie

> Lightweight vector-based visual animations rendered in real-time  
> **Version:** 2.4.1 · **Runtime:** bun · [npm](https://www.npmjs.com/package/lottie-react) · [Docs](https://github.com/LottieFiles/lottie-react)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/lottie.md`  
> To regenerate from original: `offpkg docs reset lottie --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add lottie-react
```

---

## Usage

### Simple Usage
```tsx
import Lottie from 'lottie-react';
import animationData from './assets/my-animation.json';

export function Illustration() {
  return (
    <div className="w-64 h-64">
      <Lottie animationData={animationData} loop={true} />
    </div>
  );
}
```

### Loading Lottie from a CDN Link
To prevent packaging heavy JSON files inside your bundle, fetch files dynamically:

```tsx
import { useEffect, useState } from 'react';
import Lottie from 'lottie-react';

export function LottieLoader({ src }: { src: string }) {
  const [data, setData] = useState(null);

  useEffect(() => {
    fetch(src).then(res => res.json()).then(setData);
  }, [src]);

  if (!data) return <div className="animate-pulse bg-muted/20 w-64 h-64" />;

  return <Lottie animationData={data} loop={true} />;
}
```

### Accessibility Compliance (Reduced Motion)
Respect preferences of users who request reduced motion:

```typescript
const prefersReduced = window.matchMedia('(prefers-reduced-motion: reduce)').matches;
// Disable loops or animations if prefersReduced is true
```
"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_lucide-react.md".into(),
                content: r####"# lucide-react — offpkg docs
> **Version**: 1.6.0 · **Runtime**: bun · **npm**: https://www.npmjs.com/package/lucide-react  

A Lucide icon library package for React applications.
**Homepage**: https://lucide.dev

---

> ✏️ Edit this file freely — it lives in ~/.offpkg/docs/bun/lucide-react.md
> Every project you add lucide-react to will get YOUR edited version.
> To regenerate from original: `offpkg docs reset lucide-react --runtime bun`

## My Notes

<!-- Add your own notes, snippets, team conventions here -->

---

<p align="center">
  <a href="https://github.com/lucide-icons/lucide">
    <img src="https://lucide.dev/package-logos/lucide-react.svg" alt="Lucide icon library for React applications." width="540">
  </a>
</p>

<p align="center">
Lucide icon library for React applications.
</p>

<div align="center">

  [![npm](https://img.shields.io/npm/v/lucide-react?color=blue)](https://www.npmjs.com/package/lucide-react)
  ![NPM Downloads](https://img.shields.io/npm/dw/lucide-react)
  [![GitHub](https://img.shields.io/github/license/lucide-icons/lucide)](https://lucide.dev/license)
</div>

<p align="center">
  <a href="https://lucide.dev/guide/">About</a>
  ·
  <a href="https://lucide.dev/icons/">Icons</a>
  ·
  <a href="https://lucide.dev/guide/react">Documentation</a>
  ·
  <a href="https://lucide.dev/license">License</a>
</p>

# Lucide React

Implementation of the Lucide icon library for React applications.

## Installation

```sh
pnpm add lucide-react
```

```sh
npm install lucide-react
```

```sh
yarn add lucide-react
```

```sh
bun add lucide-react
```

## Documentation

For full documentation, visit [lucide.dev](https://lucide.dev/guide/packages/lucide-react)

## Community

Join the [Discord server](https://discord.gg/EH6nSts) to chat with the maintainers and other users.

## License

Lucide is licensed under the ISC license. See [LICENSE](https://lucide.dev/license).

## Sponsors

<a href="https://vercel.com?utm_source=lucide&utm_campaign=oss">
  <img src="https://lucide.dev/vercel.svg" alt="Powered by Vercel" width="200" />
</a>

<a href="https://www.digitalocean.com/?refcode=b0877a2caebd&utm_campaign=Referral_Invite&utm_medium=Referral_Program&utm_source=badge"><img src="https://lucide.dev/digitalocean.svg" width="200" alt="DigitalOcean Referral Badge" /></a>

[//]: <> (Open Collective backers)
### Awesome backers 🍺

<a href="https://github.com/pdfme/pdfme"><img src="https://lucide.dev/sponsors/pdfme.svg" width="180" alt="pdfme – Open-source PDF generation library built with TypeScript and React." /></a>
<a href="https://www.paxhistoria.co/"><img src="https://lucide.dev/sponsors/paxhistoria.svg?" width="180" alt="Pax Historia – An alternate history sandbox game" /></a>

### Backers ☕

<a href="https://www.fina.money/"><img src="https://lucide.dev/sponsors/fina-money.png" width="180" alt="Fina Money – Modular Finance Tracker" /></a>

### Other contributors 💸

You can find all our past and non-recurring financial contributors at [our Open Collective page](https://opencollective.com/lucide-icons).
"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_react-dom.md".into(),
                content: r###"# react-dom

> React renderer for the browser DOM  
> **Version:** 19.2.4 · **Runtime:** bun · [npm](https://www.npmjs.com/package/react-dom) · [Docs](https://react.dev/reference/react-dom)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/react-dom.md`  
> To regenerate from original: `offpkg docs reset react-dom --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add react react-dom
bun add -d @types/react @types/react-dom
```

> Always install `react` and `react-dom` together — they must be the same version.

---

## Entry Point

```tsx
// main.tsx
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>
)
```

---

## `createRoot` API

```tsx
import { createRoot } from 'react-dom/client'

const root = createRoot(document.getElementById('root')!)

// Render / update
root.render(<App />)

// Unmount and clean up
root.unmount()
```

---

## Portals — render outside the component tree

Useful for modals, tooltips, and overlays that need to escape CSS overflow or z-index constraints.

```tsx
import { createPortal } from 'react-dom'

function Modal({ children }: { children: React.ReactNode }) {
  return createPortal(
    <div className="modal-overlay">
      <div className="modal">{children}</div>
    </div>,
    document.body  // renders here, not in parent's DOM position
  )
}
```

---

## `flushSync` — force synchronous DOM update

Use sparingly — bypasses React's batching. Useful when you need the DOM updated before reading a layout measurement.

```tsx
import { flushSync } from 'react-dom'

flushSync(() => {
  setCount(count + 1)
})
// DOM is updated here before the next line
inputRef.current?.scrollIntoView()
```

---

## `useFormStatus` — form submission state (React 19)

Must be used inside a `<form>` with a Server Action.

```tsx
import { useFormStatus } from 'react-dom'

function SubmitButton() {
  const { pending } = useFormStatus()

  return (
    <button type="submit" disabled={pending}>
      {pending ? 'Saving...' : 'Save'}
    </button>
  )
}
```

---

## Hydration (SSR)

```tsx
import { hydrateRoot } from 'react-dom/client'

// Attach React to server-rendered HTML
hydrateRoot(
  document.getElementById('root')!,
  <App />
)
```

---

## Imports at a Glance

```tsx
// Client rendering
import { createRoot } from 'react-dom/client'
import { hydrateRoot } from 'react-dom/client'

// Utilities
import { createPortal } from 'react-dom'
import { flushSync } from 'react-dom'

// React 19 form hooks
import { useFormStatus } from 'react-dom'
import { useFormState } from 'react-dom'  // renamed to useActionState in React 19
```

---

## Resources

- [react-dom API Reference](https://react.dev/reference/react-dom)
- [createRoot](https://react.dev/reference/react-dom/client/createRoot)
- [Portals](https://react.dev/reference/react-dom/createPortal)"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_react-hook-form.md".into(),
                content: r###"# react-hook-form

> Performant, flexible form library for React using hooks  
> **Version:** 7.72.0 · **Runtime:** bun · [npm](https://www.npmjs.com/package/react-hook-form) · [Docs](https://react-hook-form.com)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/react-hook-form.md`  
> To regenerate from original: `offpkg docs reset react-hook-form --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add react-hook-form
```

---

## Usage

```tsx
import { useForm } from 'react-hook-form'

type FormData = {
  email: string
  password: string
}

function LoginForm() {
  const {
    register,
    handleSubmit,
    formState: { errors, isSubmitting },
  } = useForm<FormData>()

  const onSubmit = async (data: FormData) => {
    await signIn(data)
  }

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <input {...register('email', { required: 'Email is required' })} />
      {errors.email && <p>{errors.email.message}</p>}

      <input
        type="password"
        {...register('password', { minLength: { value: 8, message: 'Min 8 chars' } })}
      />
      {errors.password && <p>{errors.password.message}</p>}

      <button type="submit" disabled={isSubmitting}>
        {isSubmitting ? 'Logging in...' : 'Login'}
      </button>
    </form>
  )
}
```

---

## `register` — Validation Rules

```tsx
register('field', {
  required: 'This field is required',
  minLength: { value: 3, message: 'Min 3 characters' },
  maxLength: { value: 100, message: 'Max 100 characters' },
  min: { value: 0, message: 'Must be positive' },
  max: { value: 99, message: 'Max 99' },
  pattern: { value: /^\S+@\S+$/, message: 'Invalid email' },
  validate: (value) => value !== 'admin' || 'Username "admin" not allowed',
  validate: {
    notEmpty: (v) => v.trim().length > 0 || 'Cannot be blank',
    noSpaces: (v) => !v.includes(' ') || 'No spaces allowed',
  },
})
```

---

## `useForm` Options

```ts
const form = useForm<FormData>({
  defaultValues: {
    email: '',
    password: '',
  },
  mode: 'onSubmit',       // when to validate: 'onSubmit' | 'onBlur' | 'onChange' | 'all'
  reValidateMode: 'onChange',
  shouldFocusError: true,
  criteriaMode: 'firstError',  // 'all' to collect all errors per field
})
```

---

## `formState`

```ts
const { formState } = useForm()

formState.errors           // validation errors by field name
formState.isSubmitting     // true while handleSubmit is running
formState.isSubmitted      // true after first submit
formState.isDirty          // true if any field was changed
formState.isValid          // true if no errors
formState.touchedFields    // fields the user has interacted with
formState.dirtyFields      // fields that differ from defaultValues
```

---

## Utility Methods

```ts
const { reset, setValue, getValues, watch, setError, clearErrors, trigger } = useForm()

// Reset form to defaultValues (or new values)
reset()
reset({ email: 'new@example.com' })

// Set a field value
setValue('email', 'user@example.com')
setValue('email', 'user@example.com', { shouldValidate: true, shouldDirty: true })

// Get current values
const values = getValues()
const email = getValues('email')

// Watch field(s) for re-rendering
const email = watch('email')
const [email, password] = watch(['email', 'password'])

// Set a manual error
setError('email', { type: 'manual', message: 'Email already taken' })

// Trigger validation manually
await trigger()           // all fields
await trigger('email')    // specific field
```

---

## `Controller` — For UI Library Inputs

Use `Controller` when wrapping third-party components (Select, DatePicker, etc.):

```tsx
import { useForm, Controller } from 'react-hook-form'

<Controller
  name="country"
  control={control}
  rules={{ required: true }}
  render={({ field }) => (
    <Select {...field} options={countryOptions} />
  )}
/>
```

---

## Schema Validation (with Zod)

```bash
bun add @hookform/resolvers zod
```

```tsx
import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import * as z from 'zod'

const schema = z.object({
  email: z.string().email(),
  password: z.string().min(8),
})

type FormData = z.infer<typeof schema>

const { register, handleSubmit, formState: { errors } } = useForm<FormData>({
  resolver: zodResolver(schema),
})
```

---

## Resources

- [Official Docs](https://react-hook-form.com)
- [API Reference](https://react-hook-form.com/docs)
- [Resolvers (Zod, Yup, Joi...)](https://github.com/react-hook-form/resolvers)"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_react-router-dom.md".into(),
                content: r###"# react-router-dom — offpkg docs
> **Version**: 7.13.2 · **Runtime**: bun · **npm**: https://www.npmjs.com/package/react-router-dom  

Declarative routing for React web applications
**Homepage**: https://github.com/remix-run/react-router#readme

---

> ✏️ Edit this file freely — it lives in ~/.offpkg/docs/bun/react-router-dom.md
> Every project you add react-router-dom to will get YOUR edited version.
> To regenerate from original: `offpkg docs reset react-router-dom --runtime bun`

## My Notes

<!-- Add your own notes, snippets, team conventions here -->

---

## Installation

```json
{ "dependencies": { "react-router-dom": "^7.13.2" } }
```

## Import

```typescript
import ... from 'react-router-dom';
```

## Quick Start

```typescript
// Add your usage example here
```

## Links

- npm: https://www.npmjs.com/package/react-router-dom
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_react.md".into(),
                content: r####"# react

> JavaScript library for building user interfaces  
> **Version:** 19.2.4 · **Runtime:** bun · [npm](https://www.npmjs.com/package/react) · [Docs](https://react.dev)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/react.md`  
> To regenerate from original: `offpkg docs reset react --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add react react-dom
bun add -d @types/react @types/react-dom
```

## Import

```tsx
import React, { useState, useEffect, useRef, useMemo, useCallback } from 'react'
import ReactDOM from 'react-dom/client'
```

---

## Component

```tsx
// Function component (standard)
function Greeting({ name }: { name: string }) {
  return <h1>Hello, {name}!</h1>
}

// Arrow function component
const Button = ({ onClick, children }: { onClick: () => void; children: React.ReactNode }) => (
  <button onClick={onClick}>{children}</button>
)
```

---

## Hooks

### `useState`

```tsx
const [count, setCount] = useState(0)
const [user, setUser] = useState<User | null>(null)

setCount(count + 1)
setCount(prev => prev + 1)          // functional update
setUser(prev => ({ ...prev, name: 'Jane' }))
```

### `useEffect`

```tsx
// On mount
useEffect(() => {
  fetchData()
}, [])

// On dependency change
useEffect(() => {
  document.title = `${count} items`
}, [count])

// Cleanup on unmount
useEffect(() => {
  const sub = subscribe()
  return () => sub.unsubscribe()
}, [])
```

### `useRef`

```tsx
const inputRef = useRef<HTMLInputElement>(null)
inputRef.current?.focus()

// Mutable value — does NOT trigger re-render
const timerRef = useRef<number>(0)
timerRef.current = setTimeout(fn, 1000)
```

### `useMemo` — expensive computation cache

```tsx
const sorted = useMemo(
  () => items.slice().sort((a, b) => a.name.localeCompare(b.name)),
  [items]
)
```

### `useCallback` — stable function reference

```tsx
const handleClick = useCallback(() => {
  setCount(c => c + 1)
}, []) // only recreated if deps change
```

### `useContext`

```tsx
const ThemeContext = React.createContext<'light' | 'dark'>('light')

// Provide
<ThemeContext.Provider value="dark">
  <App />
</ThemeContext.Provider>

// Consume
const theme = useContext(ThemeContext)
```

### `useReducer`

```tsx
type Action = { type: 'increment' } | { type: 'reset' }

function reducer(state: number, action: Action): number {
  switch (action.type) {
    case 'increment': return state + 1
    case 'reset': return 0
  }
}

const [count, dispatch] = useReducer(reducer, 0)
dispatch({ type: 'increment' })
```

### `useId` — unique IDs for accessibility

```tsx
const id = useId()
<label htmlFor={id}>Name</label>
<input id={id} />
```

### `useTransition` — mark state update as non-urgent

```tsx
const [isPending, startTransition] = useTransition()

startTransition(() => {
  setFilter(input)  // non-blocking update
})
```

---

## React 19 — New Features

### `use()` hook — read resources in render

```tsx
import { use } from 'react'

function UserCard({ userPromise }: { userPromise: Promise<User> }) {
  const user = use(userPromise)  // suspends until resolved
  return <p>{user.name}</p>
}
```

### `useOptimistic` — optimistic UI updates

```tsx
const [optimisticLikes, addOptimisticLike] = useOptimistic(
  likes,
  (state, newLike) => [...state, newLike]
)

async function handleLike() {
  addOptimisticLike(tempLike)   // shows immediately
  await saveLike(tempLike)      // actual API call
}
```

### Server Actions (React 19 + frameworks)

```tsx
async function createUser(formData: FormData) {
  'use server'
  const name = formData.get('name')
  await db.users.create({ name })
}

<form action={createUser}>
  <input name="name" />
  <button type="submit">Create</button>
</form>
```

---

## JSX Patterns

```tsx
// Conditional rendering
{isLoggedIn && <Dashboard />}
{isLoggedIn ? <Dashboard /> : <Login />}

// Lists
{items.map(item => (
  <li key={item.id}>{item.name}</li>
))}

// Fragment (no extra DOM node)
<>
  <h1>Title</h1>
  <p>Body</p>
</>

// Spread props
<Button {...buttonProps} />

// Children
function Card({ children }: { children: React.ReactNode }) {
  return <div className="card">{children}</div>
}
```

---

## TypeScript Patterns

```tsx
// Component props
interface ButtonProps {
  label: string
  onClick: () => void
  variant?: 'primary' | 'secondary'
  disabled?: boolean
  children?: React.ReactNode
}

// Event handlers
const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {}
const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {}
const handleClick = (e: React.MouseEvent<HTMLButtonElement>) => {}
const handleKeyDown = (e: React.KeyboardEvent) => {}

// Ref types
const divRef = useRef<HTMLDivElement>(null)
const inputRef = useRef<HTMLInputElement>(null)

// Generic component
function List<T extends { id: string }>({ items, renderItem }: {
  items: T[]
  renderItem: (item: T) => React.ReactNode
}) {
  return <ul>{items.map(item => <li key={item.id}>{renderItem(item)}</li>)}</ul>
}
```

---

## Entry Point (`react-dom`)

```tsx
// main.tsx
import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import App from './App'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <App />
  </StrictMode>
)
```

---

## Custom Hooks

```tsx
function useLocalStorage<T>(key: string, initial: T) {
  const [value, setValue] = useState<T>(() => {
    const stored = localStorage.getItem(key)
    return stored ? JSON.parse(stored) : initial
  })

  const set = useCallback((val: T) => {
    setValue(val)
    localStorage.setItem(key, JSON.stringify(val))
  }, [key])

  return [value, set] as const
}
```

---

## Resources

- [react.dev](https://react.dev)
- [Hooks Reference](https://react.dev/reference/react)
- [React 19 Changelog](https://react.dev/blog/2024/12/05/react-19)"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_shadcn.md".into(),
                content: r####"# shadcn

> Beautiful, accessible, fully-customizable UI components built on Radix Primitives  
> **Version:** 4.10.0 · **Runtime:** bun · [npm](https://www.npmjs.com/package/shadcn) · [Docs](https://ui.shadcn.com)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/shadcn.md`  
> To regenerate from original: `offpkg docs reset shadcn --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bunx shadcn@latest init
bunx shadcn@latest add accordion tabs dialog
```

---

## Usage

### The Class Merger (`cn`)
All shadcn components utilize a custom Tailwind utility to safely merge classes without duplicates:

```typescript
// src/lib/utils.ts
import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
```

### Rendering a Component (e.g. Accordion)
```tsx
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion"

export function AccordionDemo() {
  return (
    <Accordion type="single" collapsible>
      <AccordionItem value="item-1">
        <AccordionTrigger>Is it styled?</AccordionTrigger>
        <AccordionContent>
          Yes. It aligns with your Tailwind CSS variables theme out of the box.
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  )
}
```
"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_tailwindcss.md".into(),
                content: r####"# tailwindcss

> Utility-first CSS framework  
> **Version:** 4.2.2 · **Runtime:** bun · [Docs](https://tailwindcss.com) · [Cheat sheet](https://nerdcave.com/tailwind-cheat-sheet)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/tailwindcss.md`  
> To regenerate from original: `offpkg docs reset tailwindcss --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install & Setup (v4)

```bash
bun add -d tailwindcss @tailwindcss/vite   # with Vite
# or
bun add -d tailwindcss @tailwindcss/postcss postcss
```

```ts
// vite.config.ts
import tailwindcss from '@tailwindcss/vite'
plugins: [tailwindcss()]
```

```css
/* globals.css — v4: one import replaces all three directives */
@import "tailwindcss";

/* Custom theme tokens — generates bg-primary, text-primary, etc. */
@theme {
  --color-primary: #3b82f6;
  --color-secondary: #8b5cf6;
  --font-sans: 'Inter', sans-serif;
  --spacing-18: 4.5rem;
}
```

> **v4 key change:** No `tailwind.config.js` needed. All config lives in your CSS file via `@theme {}`.

---

## SPACING SCALE

Default scale: `0 0.5 1 1.5 2 2.5 3 3.5 4 5 6 7 8 9 10 11 12 14 16 20 24 28 32 36 40 44 48 52 56 60 64 72 80 96`  
Each unit = `0.25rem` (4px). So `p-4` = `1rem` = `16px`, `p-8` = `2rem` = `32px`.

**Quick mental model:**
```
p-1  =  4px      p-2  =  8px      p-4  =  16px
p-6  =  24px     p-8  =  32px     p-12 =  48px
p-16 =  64px     p-24 =  96px
```

---

## LAYOUT

### Display
```
block inline inline-block flex inline-flex grid inline-grid
table hidden contents
```

**Examples:**
```html
<!-- Card layout -->
<div class="flex gap-4">
  <div class="hidden md:block">Sidebar</div>  <!-- hide on mobile -->
  <main class="flex-1">Content</main>
</div>

<!-- Inline badge -->
<span class="inline-block px-2 py-0.5 text-xs bg-blue-100 text-blue-700 rounded-full">
  New
</span>
```

### Position
```
static relative absolute fixed sticky
inset-0  inset-x-0  inset-y-0
top-0  right-0  bottom-0  left-0
top-4  -top-2  top-1/2
z-0  z-10  z-20  z-30  z-40  z-50  z-auto  -z-10
```

**Examples:**
```html
<!-- Overlay modal -->
<div class="fixed inset-0 z-50 bg-black/50 flex items-center justify-center">
  <div class="relative bg-white rounded-xl p-6 w-full max-w-md">
    <button class="absolute top-4 right-4">✕</button>
    Modal content
  </div>
</div>

<!-- Sticky header -->
<header class="sticky top-0 z-40 bg-white border-b shadow-sm">...</header>

<!-- Badge on avatar -->
<div class="relative inline-block">
  <img class="w-10 h-10 rounded-full" src="..." />
  <span class="absolute -top-1 -right-1 w-3 h-3 bg-green-500 rounded-full border-2 border-white"></span>
</div>
```

### Overflow
```
overflow-hidden  overflow-auto  overflow-scroll  overflow-visible
overflow-x-hidden  overflow-y-auto
```

**Examples:**
```html
<!-- Horizontal scrollable tabs -->
<div class="flex overflow-x-auto gap-2 pb-2 scrollbar-hide">
  <button class="shrink-0 px-4 py-2">Tab 1</button>
  <button class="shrink-0 px-4 py-2">Tab 2</button>
</div>

<!-- Clip image in card -->
<div class="rounded-xl overflow-hidden">
  <img class="w-full h-48 object-cover" src="..." />
</div>
```

---

## FLEXBOX

```
flex          flex-row         flex-col
flex-wrap     flex-nowrap      flex-wrap-reverse
flex-1        flex-auto        flex-none        flex-initial

justify-start   justify-center   justify-end
justify-between  justify-around  justify-evenly  justify-stretch

items-start   items-center   items-end   items-baseline  items-stretch

self-start    self-center    self-end    self-auto

grow          grow-0
shrink        shrink-0

gap-4         gap-x-4         gap-y-4
```

**Examples:**
```html
<!-- Navbar -->
<nav class="flex items-center justify-between px-6 py-4">
  <span class="font-bold text-lg">Logo</span>
  <div class="flex items-center gap-4">
    <a href="#">About</a>
    <button class="bg-blue-600 text-white px-4 py-2 rounded-lg">Sign in</button>
  </div>
</nav>

<!-- Card with footer pinned to bottom -->
<div class="flex flex-col h-full">
  <div class="flex-1 p-4">Content grows here</div>
  <div class="p-4 border-t">Footer stays at bottom</div>
</div>

<!-- Centered hero -->
<section class="flex flex-col items-center justify-center min-h-screen text-center">
  <h1 class="text-5xl font-bold">Hello</h1>
  <p class="mt-4 text-gray-500 max-w-md">Subtitle goes here</p>
</section>

<!-- Tags that wrap -->
<div class="flex flex-wrap gap-2">
  <span class="px-3 py-1 bg-gray-100 rounded-full text-sm">React</span>
  <span class="px-3 py-1 bg-gray-100 rounded-full text-sm">TypeScript</span>
</div>
```

---

## GRID

```
grid
grid-cols-1  grid-cols-2  grid-cols-3 ... grid-cols-12  grid-cols-none
grid-rows-1  grid-rows-2 ...

col-span-1   col-span-2   col-span-full
col-start-1  col-end-3
row-span-2   row-start-1

gap-4    gap-x-4    gap-y-4

auto-cols-auto   auto-cols-fr   auto-cols-min   auto-cols-max
auto-rows-auto   auto-rows-fr
```

**Examples:**
```html
<!-- Responsive card grid -->
<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6">
  <div class="bg-white rounded-xl shadow p-4">Card 1</div>
  <div class="bg-white rounded-xl shadow p-4">Card 2</div>
  <div class="bg-white rounded-xl shadow p-4">Card 3</div>
</div>

<!-- Dashboard layout with sidebar -->
<div class="grid grid-cols-[240px_1fr] min-h-screen">
  <aside class="bg-gray-900 text-white p-4">Sidebar</aside>
  <main class="p-6">Main content</main>
</div>

<!-- Feature grid with featured item spanning 2 cols -->
<div class="grid grid-cols-3 gap-4">
  <div class="col-span-2 bg-blue-50 rounded-xl p-6">Featured</div>
  <div class="bg-gray-50 rounded-xl p-6">Side</div>
  <div class="bg-gray-50 rounded-xl p-6">Item</div>
  <div class="bg-gray-50 rounded-xl p-6">Item</div>
  <div class="bg-gray-50 rounded-xl p-6">Item</div>
</div>
```

---

## SIZING

### Width
```
w-0  w-px  w-0.5  w-1 ... w-96
w-auto  w-full  w-screen  w-svw  w-dvw
w-1/2  w-1/3  w-2/3  w-1/4  w-3/4
w-fit  w-min  w-max
w-[350px]  w-[50%]  w-[calc(100%-2rem)]
```

### Height
```
h-0 h-px ... h-96
h-auto  h-full  h-screen  h-svh  h-dvh
h-1/2  h-1/3  h-fit  h-min  h-max
h-[500px]
```

> **Tip:** Use `h-svh` / `w-svw` instead of `h-screen` on mobile — avoids the browser toolbar overlap issue.

### Min / Max
```
min-w-0   min-w-full   min-w-min   min-w-max   min-w-fit
max-w-xs  max-w-sm  max-w-md  max-w-lg  max-w-xl
max-w-2xl  max-w-3xl  max-w-4xl  max-w-5xl  max-w-6xl  max-w-7xl
max-w-full  max-w-screen-xl  max-w-none  max-w-prose
min-h-0   min-h-full   min-h-screen
max-h-full  max-h-screen  max-h-96
```

**Examples:**
```html
<!-- Centered page container -->
<div class="max-w-5xl mx-auto px-4">Page content</div>

<!-- Readable prose width -->
<article class="max-w-prose mx-auto">Long form text...</article>

<!-- Avatar sizes -->
<img class="w-8 h-8 rounded-full" />   <!-- small: 32px -->
<img class="w-10 h-10 rounded-full" /> <!-- medium: 40px -->
<img class="w-12 h-12 rounded-full" /> <!-- large: 48px -->

<!-- Full-height sidebar that scrolls -->
<aside class="w-64 min-h-screen overflow-y-auto">...</aside>
```

---

## SPACING

### Padding
```
p-4           px-4 py-4
pt-4 pr-4 pb-4 pl-4
ps-4 pe-4     (logical: start/end)
p-[1.25rem]
```

### Margin
```
m-4  mx-4  my-4  mx-auto
mt-4  mr-4  mb-4  ml-4
-mt-2  -mx-4
m-[20px]
```

### Space Between (flex/grid children)
```
space-x-4   space-y-4
space-x-reverse   space-y-reverse
```

> **`space-x` vs `gap`:** Prefer `gap` on flex/grid containers. `space-x` adds margin to children and can break with wrapping. `gap` is cleaner and works correctly with `flex-wrap`.

**Examples:**
```html
<!-- Section spacing -->
<section class="py-16 px-4">
  <h2 class="text-3xl font-bold mb-2">Title</h2>
  <p class="text-gray-500 mb-8">Subtitle</p>
  <div class="grid grid-cols-3 gap-6">...</div>
</section>

<!-- Button with icon -->
<button class="flex items-center gap-2 px-4 py-2 bg-blue-600 text-white rounded-lg">
  <svg class="w-4 h-4">...</svg>
  <span>Download</span>
</button>

<!-- Negative margin pull trick -->
<div class="mt-8 -mx-4 px-4 bg-gray-50 py-6">
  Full-bleed section in a constrained container
</div>
```

---

## TYPOGRAPHY

### Font Size
```
text-xs    (0.75rem / 12px)
text-sm    (0.875rem / 14px)
text-base  (1rem / 16px)
text-lg    (1.125rem / 18px)
text-xl    (1.25rem / 20px)
text-2xl   (1.5rem / 24px)
text-3xl   (1.875rem / 30px)
text-4xl   (2.25rem / 36px)
text-5xl   (3rem / 48px)
text-6xl   (3.75rem / 60px)
text-7xl   (4.5rem / 72px)
text-8xl   (6rem / 96px)
text-9xl   (8rem / 128px)
text-[1.375rem]
```

### Font Weight
```
font-thin (100)    font-extralight (200)  font-light (300)
font-normal (400)  font-medium (500)      font-semibold (600)
font-bold (700)    font-extrabold (800)   font-black (900)
```

### Font Family
```
font-sans   font-serif   font-mono
```

### Text Alignment
```
text-left   text-center   text-right   text-justify   text-start   text-end
```

### Text Transform
```
uppercase   lowercase   capitalize   normal-case
```

### Text Decoration
```
underline   overline   line-through   no-underline
decoration-solid   decoration-dashed   decoration-dotted   decoration-double
decoration-blue-500   decoration-2
```

### Line Height
```
leading-none (1)    leading-tight (1.25)   leading-snug (1.375)
leading-normal (1.5) leading-relaxed (1.625) leading-loose (2)
leading-[1.8]
```

### Letter Spacing
```
tracking-tighter  tracking-tight  tracking-normal
tracking-wide     tracking-wider  tracking-widest
tracking-[0.05em]
```

### Truncate / Clamp
```
truncate         (single line ellipsis)
line-clamp-2     (clamp to 2 lines)
line-clamp-3
overflow-hidden
```

### Whitespace
```
whitespace-normal   whitespace-nowrap   whitespace-pre
whitespace-pre-wrap  whitespace-pre-line  whitespace-break-spaces
```

**Examples:**
```html
<!-- Page heading -->
<h1 class="text-4xl md:text-6xl font-black tracking-tight leading-tight">
  Build faster.
</h1>

<!-- Subheading with muted color -->
<p class="text-lg text-gray-500 leading-relaxed max-w-xl">
  A utility-first CSS framework packed with classes.
</p>

<!-- Label / caption -->
<span class="text-xs font-medium uppercase tracking-widest text-gray-400">
  Category
</span>

<!-- Card title that truncates -->
<h3 class="text-base font-semibold truncate">
  This is a very long title that will be cut off
</h3>

<!-- Multi-line clamp for card descriptions -->
<p class="text-sm text-gray-600 line-clamp-3">
  Long description text that will be clamped after 3 lines...
</p>

<!-- Monospace code block -->
<code class="font-mono text-sm bg-gray-100 px-2 py-0.5 rounded">
  bun run dev
</code>

<!-- Price display -->
<div class="flex items-baseline gap-1">
  <span class="text-3xl font-bold">$29</span>
  <span class="text-gray-400 line-through text-sm">$49</span>
</div>
```

---

## COLORS

### Text
```
text-black   text-white   text-transparent   text-current
text-slate-500   text-gray-700   text-zinc-900
text-red-500     text-orange-500  text-amber-500  text-yellow-500
text-lime-500    text-green-500   text-emerald-500 text-teal-500
text-cyan-500    text-sky-500     text-blue-500   text-indigo-500
text-violet-500  text-purple-500  text-fuchsia-500 text-pink-500 text-rose-500
text-blue-500/75  (with 75% opacity)
```

Shades: `50 100 200 300 400 500 600 700 800 900 950`

### Background
```
bg-white   bg-black   bg-transparent
bg-blue-500   bg-blue-500/50    (50% opacity)
bg-gradient-to-r  from-blue-500  via-purple-500  to-pink-500
bg-gradient-to-br  from-slate-900  to-slate-700
```

### Background Size / Position
```
bg-auto   bg-cover   bg-contain
bg-center bg-top bg-bottom bg-left bg-right
bg-no-repeat  bg-repeat  bg-repeat-x  bg-repeat-y
```

### Border Color
```
border-gray-200   border-blue-500   border-transparent
```

**Examples:**
```html
<!-- Status badges -->
<span class="bg-green-100 text-green-700 px-2 py-0.5 rounded-full text-xs font-medium">Active</span>
<span class="bg-yellow-100 text-yellow-700 px-2 py-0.5 rounded-full text-xs font-medium">Pending</span>
<span class="bg-red-100 text-red-700 px-2 py-0.5 rounded-full text-xs font-medium">Error</span>
<span class="bg-gray-100 text-gray-600 px-2 py-0.5 rounded-full text-xs font-medium">Draft</span>

<!-- Gradient hero background -->
<section class="bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 text-white py-24">
  <h1 class="text-5xl font-bold">Hello</h1>
</section>

<!-- Gradient text -->
<h1 class="bg-gradient-to-r from-blue-600 to-violet-600 bg-clip-text text-transparent text-5xl font-black">
  Gradient Text
</h1>

<!-- Semi-transparent overlay -->
<div class="absolute inset-0 bg-black/60 rounded-xl"></div>

<!-- Glass card -->
<div class="bg-white/10 backdrop-blur-md border border-white/20 rounded-xl p-6">
  Glass effect
</div>

<!-- Hero image with overlay text -->
<div class="relative h-64 rounded-xl overflow-hidden">
  <img class="w-full h-full object-cover" src="..." />
  <div class="absolute inset-0 bg-gradient-to-t from-black/70 to-transparent"></div>
  <p class="absolute bottom-4 left-4 text-white font-semibold">Caption text</p>
</div>
```

---

## BORDERS

```
border        (1px solid)
border-2      border-4      border-8
border-0      border-t-2    border-b    border-l-4    border-r

border-solid  border-dashed  border-dotted  border-double  border-none

border-gray-300   border-red-500

rounded-none   rounded-sm   rounded   rounded-md   rounded-lg
rounded-xl     rounded-2xl  rounded-3xl   rounded-full

rounded-t-lg   rounded-b-xl   rounded-l-md   rounded-r-full
rounded-tl-lg  rounded-tr-xl  rounded-bl-md  rounded-br-full
```

**Examples:**
```html
<!-- Card with subtle border -->
<div class="border border-gray-200 rounded-xl p-5 bg-white shadow-sm">Card</div>

<!-- Input field -->
<input class="w-full border border-gray-300 rounded-lg px-3 py-2
              focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent" />

<!-- Input with error state -->
<input class="border-2 border-red-400 rounded-lg px-3 py-2 focus:outline-none focus:ring-2 focus:ring-red-400" />

<!-- Divider line -->
<hr class="border-t border-gray-100 my-6" />

<!-- Left accent border -->
<div class="border-l-4 border-blue-500 pl-4 text-gray-700">
  Info message with accent
</div>

<!-- Avatar / circle -->
<img class="w-12 h-12 rounded-full border-2 border-white ring-2 ring-blue-500" />

<!-- Pill / tag -->
<span class="border border-gray-300 rounded-full px-3 py-1 text-sm">Tag</span>

<!-- Dashed upload zone -->
<div class="border-2 border-dashed border-gray-300 rounded-xl p-8 text-center
            hover:border-blue-400 transition-colors cursor-pointer">
  Drop files here
</div>
```

---

## SHADOWS

```
shadow-xs    shadow-sm    shadow    shadow-md
shadow-lg    shadow-xl    shadow-2xl   shadow-inner   shadow-none
shadow-blue-500/50    (colored shadow)
```

### Ring (outline ring)
```
ring         (3px)
ring-1  ring-2  ring-4  ring-8
ring-blue-500    ring-offset-2   ring-offset-white
ring-inset
```

**Examples:**
```html
<!-- Elevated card -->
<div class="bg-white rounded-xl shadow-md hover:shadow-xl transition-shadow p-5">
  Lifts on hover
</div>

<!-- Floating action button -->
<button class="bg-blue-600 text-white rounded-full p-4 shadow-lg shadow-blue-500/40
               hover:shadow-xl hover:shadow-blue-500/50 transition-all">
  <svg class="w-6 h-6">...</svg>
</button>

<!-- Focused input ring -->
<input class="border border-gray-300 rounded-lg px-3 py-2
              focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2" />

<!-- Inset shadow (like pressed/sunken) -->
<div class="bg-gray-100 rounded-lg shadow-inner p-4">Inset input</div>
```

---

## EFFECTS

### Opacity
```
opacity-0   opacity-5   opacity-10   opacity-20   opacity-25
opacity-50  opacity-75  opacity-90   opacity-95   opacity-100
opacity-[0.35]
```

### Background Opacity (v4: use / modifier)
```
bg-black/50     text-white/80     border-gray-500/30
```

### Filter
```
blur-none  blur-sm  blur  blur-md  blur-lg  blur-xl  blur-2xl  blur-3xl
blur-[4px]
brightness-0  brightness-50  brightness-75  brightness-100  brightness-125  brightness-150
contrast-0   contrast-50   contrast-100   contrast-150   contrast-200
grayscale   grayscale-0
invert   invert-0
sepia    sepia-0
drop-shadow-md   drop-shadow-lg
backdrop-blur-sm  backdrop-blur-md  backdrop-blur-xl
backdrop-brightness-50  backdrop-contrast-150
```

**Examples:**
```html
<!-- Disabled state -->
<button class="opacity-50 cursor-not-allowed" disabled>Disabled</button>

<!-- Image hover: reveal on hover -->
<div class="group relative overflow-hidden rounded-xl">
  <img class="w-full h-64 object-cover transition-transform duration-300 group-hover:scale-105" />
  <div class="absolute inset-0 bg-black/0 group-hover:bg-black/40 transition-colors duration-300 flex items-center justify-center">
    <span class="text-white font-semibold opacity-0 group-hover:opacity-100 transition-opacity">View</span>
  </div>
</div>

<!-- Skeleton loader -->
<div class="animate-pulse">
  <div class="h-4 bg-gray-200 rounded w-3/4 mb-2"></div>
  <div class="h-4 bg-gray-200 rounded w-1/2"></div>
</div>

<!-- Glassmorphism card -->
<div class="backdrop-blur-xl bg-white/20 border border-white/30 rounded-2xl p-6">
  Glass card
</div>

<!-- Blurred background behind modal -->
<div class="fixed inset-0 backdrop-blur-sm bg-black/30 z-40"></div>

<!-- Grayscale image that colors on hover -->
<img class="grayscale hover:grayscale-0 transition-all duration-300" />
```

---

## TRANSFORMS

```
scale-0   scale-50   scale-75   scale-90   scale-95   scale-100   scale-105   scale-110   scale-125   scale-150
scale-x-0  scale-y-150
-scale-x-100   (flip horizontal)

rotate-0   rotate-1   rotate-2   rotate-3   rotate-6
rotate-12  rotate-45  rotate-90  rotate-180
-rotate-6  rotate-[17deg]

translate-x-4   translate-y-4  -translate-x-1/2   translate-y-1/2
translate-x-full  -translate-y-full

skew-x-2   skew-y-3

origin-center  origin-top  origin-top-right  origin-right
origin-bottom  origin-bottom-left  origin-left  origin-top-left
```

**Examples:**
```html
<!-- Button press effect -->
<button class="active:scale-95 transition-transform">Click me</button>

<!-- Hover lift -->
<div class="hover:-translate-y-1 transition-transform">Card</div>

<!-- Centered absolutely with translate trick -->
<div class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2">
  Perfectly centered
</div>

<!-- Rotate icon on state change -->
<svg class="transition-transform duration-200 rotate-0 group-open:rotate-180">
  <!-- chevron icon -->
</svg>

<!-- Flip horizontally (mirror) -->
<svg class="-scale-x-100">...</svg>

<!-- Card tilt on hover (with group) -->
<div class="group cursor-pointer">
  <div class="transition-transform duration-300 group-hover:rotate-1 group-hover:scale-105">
    Card content
  </div>
</div>
```

---

## TRANSITIONS & ANIMATION

```
transition-none   transition-all   transition   transition-colors
transition-opacity  transition-shadow  transition-transform

duration-75   duration-100   duration-150   duration-200
duration-300  duration-500   duration-700   duration-1000

ease-linear   ease-in   ease-out   ease-in-out

delay-0   delay-75   delay-100   delay-150   delay-300

animate-none
animate-spin      (rotate 360° loop)
animate-ping      (scale + fade loop — notifications)
animate-pulse     (opacity pulse — skeleton)
animate-bounce    (bounce loop)
```

> **Rule of thumb:** Use `duration-150` to `duration-200` for interactions (hover, click). Use `duration-300` to `duration-500` for entrance/exit animations. Faster feels snappier, slower feels heavier.

**Examples:**
```html
<!-- Smooth color + shadow on hover -->
<button class="bg-blue-600 text-white px-4 py-2 rounded-lg
               transition-all duration-200
               hover:bg-blue-700 hover:shadow-lg hover:shadow-blue-500/30
               active:scale-95">
  Button
</button>

<!-- Notification dot (ping) -->
<span class="relative flex h-3 w-3">
  <span class="animate-ping absolute inline-flex h-full w-full rounded-full bg-red-400 opacity-75"></span>
  <span class="relative inline-flex rounded-full h-3 w-3 bg-red-500"></span>
</span>

<!-- Loading spinner -->
<svg class="animate-spin h-5 w-5 text-blue-600">
  <!-- spinner SVG path -->
</svg>

<!-- Skeleton loading card -->
<div class="animate-pulse rounded-xl bg-gray-200 h-40 w-full"></div>

<!-- Staggered fade-in (use with JS class toggle) -->
<div class="opacity-0 translate-y-4 transition-all duration-500 delay-100 [&.visible]:opacity-100 [&.visible]:translate-y-0">
  Item 1
</div>

<!-- Transition only specific properties -->
<div class="transition-colors duration-200 bg-gray-100 hover:bg-blue-50">
  Only color transitions — no layout jitter
</div>
```

---

## INTERACTIVITY

```
cursor-auto   cursor-default   cursor-pointer   cursor-wait
cursor-text   cursor-move      cursor-not-allowed   cursor-grab  cursor-grabbing

select-none   select-text   select-all   select-auto

pointer-events-none   pointer-events-auto

resize   resize-none   resize-x   resize-y

scroll-auto   scroll-smooth
scroll-m-4   scroll-p-4    (scroll margin/padding)
snap-start   snap-center   snap-end
snap-x   snap-y   snap-mandatory   snap-proximity
```

**Examples:**
```html
<!-- Drag handle -->
<div class="cursor-grab active:cursor-grabbing p-2 text-gray-400">⋮⋮</div>

<!-- Non-interactive overlay (clicks pass through) -->
<div class="pointer-events-none absolute inset-0 border-2 border-blue-500 rounded-xl"></div>

<!-- Prevent text selection on double-click (e.g. buttons) -->
<button class="select-none px-4 py-2">Click me</button>

<!-- Smooth scroll to section -->
<html class="scroll-smooth">
<!-- then: -->
<a href="#section">Go to section</a>
<section id="section" class="scroll-mt-20">  <!-- offset for sticky header -->

<!-- Horizontal snap carousel -->
<div class="flex overflow-x-auto snap-x snap-mandatory gap-4 pb-4">
  <div class="snap-center shrink-0 w-80 bg-white rounded-xl p-4 shadow">Slide 1</div>
  <div class="snap-center shrink-0 w-80 bg-white rounded-xl p-4 shadow">Slide 2</div>
  <div class="snap-center shrink-0 w-80 bg-white rounded-xl p-4 shadow">Slide 3</div>
</div>
```

---

## RESPONSIVE PREFIXES

```
sm:   (640px+)
md:   (768px+)
lg:   (1024px+)
xl:   (1280px+)
2xl:  (1536px+)
```

> **Mobile-first:** Write the base style for mobile, then add `sm:` / `md:` / `lg:` overrides for larger screens.

**Examples:**
```html
<!-- Responsive grid -->
<div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">

<!-- Responsive text -->
<h1 class="text-2xl sm:text-3xl md:text-4xl lg:text-5xl font-bold">Responsive heading</h1>

<!-- Show/hide per breakpoint -->
<div class="hidden md:block">Only on desktop</div>
<div class="block md:hidden">Only on mobile</div>

<!-- Stack on mobile, row on desktop -->
<div class="flex flex-col md:flex-row gap-4">
  <aside class="w-full md:w-64">Sidebar</aside>
  <main class="flex-1">Content</main>
</div>

<!-- Responsive padding -->
<div class="px-4 sm:px-6 lg:px-8 py-8 sm:py-12 lg:py-16">Section</div>

<!-- Font size ramp -->
<p class="text-sm md:text-base lg:text-lg leading-relaxed">Body text</p>
```

---

## STATE VARIANTS

```
hover:bg-blue-600       focus:ring-2         active:scale-95
focus-within:border-blue-500
focus-visible:outline-2
disabled:opacity-50      disabled:cursor-not-allowed
checked:bg-blue-500
placeholder:text-gray-400
first:pt-0               last:pb-0           odd:bg-gray-50    even:bg-white
group-hover:text-blue-500   (parent has class="group")
peer-checked:text-blue-500  (sibling has class="peer")
```

**Examples:**
```html
<!-- Full interactive button -->
<button class="bg-blue-600 text-white px-4 py-2 rounded-lg font-medium
               hover:bg-blue-700
               focus-visible:ring-2 focus-visible:ring-blue-500 focus-visible:ring-offset-2
               active:scale-95
               disabled:opacity-50 disabled:cursor-not-allowed
               transition-all duration-150">
  Submit
</button>

<!-- Table with alternating rows -->
<tr class="odd:bg-white even:bg-gray-50 hover:bg-blue-50 transition-colors">

<!-- List without extra padding on first/last items -->
<ul>
  <li class="py-3 first:pt-0 last:pb-0 border-b last:border-0">Item</li>
</ul>

<!-- Group hover — child reacts to parent hover -->
<div class="group flex items-center gap-3 p-3 rounded-lg hover:bg-gray-50 cursor-pointer">
  <img class="w-10 h-10 rounded-full" />
  <div>
    <p class="font-medium group-hover:text-blue-600 transition-colors">Username</p>
    <p class="text-sm text-gray-500">@handle</p>
  </div>
  <svg class="ml-auto opacity-0 group-hover:opacity-100 transition-opacity text-gray-400">
    <!-- chevron -->
  </svg>
</div>

<!-- Peer — custom checkbox label -->
<label class="flex items-center gap-2 cursor-pointer">
  <input type="checkbox" class="peer sr-only" />
  <div class="w-10 h-6 rounded-full bg-gray-300 peer-checked:bg-blue-600 transition-colors"></div>
  Toggle
</label>

<!-- Focus within — highlight form field wrapper -->
<div class="border border-gray-300 rounded-lg focus-within:border-blue-500 focus-within:ring-2 focus-within:ring-blue-500/30 transition-all">
  <input class="outline-none px-3 py-2 w-full rounded-lg" placeholder="Search..." />
</div>

<!-- Placeholder styling -->
<input class="placeholder:text-gray-400 placeholder:italic px-3 py-2 border rounded-lg"
       placeholder="Enter your email" />
```

---

## DARK MODE

```css
@import "tailwindcss";
```

```html
<!-- Toggle with class on <html> -->
<html class="dark">
```

**Examples:**
```html
<!-- Page background -->
<body class="bg-white dark:bg-gray-950 text-gray-900 dark:text-gray-100">

<!-- Card -->
<div class="bg-white dark:bg-gray-800 border border-gray-200 dark:border-gray-700 rounded-xl p-5 shadow-sm dark:shadow-none">
  <h2 class="text-gray-900 dark:text-white font-semibold">Card title</h2>
  <p class="text-gray-500 dark:text-gray-400 text-sm mt-1">Description</p>
</div>

<!-- Input -->
<input class="bg-white dark:bg-gray-900 text-gray-900 dark:text-white
              border border-gray-300 dark:border-gray-600
              placeholder:text-gray-400 dark:placeholder:text-gray-500
              rounded-lg px-3 py-2 focus:ring-2 focus:ring-blue-500 outline-none" />

<!-- Button -->
<button class="bg-blue-600 dark:bg-blue-500 text-white
               hover:bg-blue-700 dark:hover:bg-blue-400
               px-4 py-2 rounded-lg">
  Submit
</button>

<!-- Navigation sidebar -->
<nav class="bg-gray-900 dark:bg-gray-950 text-gray-300 dark:text-gray-400 p-4">
  <a class="hover:text-white dark:hover:text-white hover:bg-gray-800 dark:hover:bg-gray-800
            block px-3 py-2 rounded-lg transition-colors">
    Dashboard
  </a>
</nav>

<!-- Code block -->
<pre class="bg-gray-100 dark:bg-gray-800 text-gray-800 dark:text-gray-200 rounded-lg p-4 text-sm font-mono overflow-x-auto">
  code here
</pre>
```

---

## ARBITRARY VALUES

```html
<!-- Sizing -->
w-[350px]           h-[calc(100vh-4rem)]       w-[calc(100%-2rem)]

<!-- Spacing -->
p-[1.25rem]        mt-[17px]                   gap-[1.375rem]

<!-- Typography -->
text-[1.375rem]    leading-[1.8]               tracking-[0.05em]

<!-- Colors -->
bg-[#1a1a2e]       text-[rgb(100,200,50)]      border-[#e5e7eb]

<!-- Position -->
top-[117px]        left-[calc(50%-11rem)]

<!-- Borders -->
border-[3px]       rounded-[10px]

<!-- Other -->
z-[100]            opacity-[0.35]              duration-[400ms]
bg-[url('/img/hero.png')]
grid-cols-[1fr_2fr_1fr]
shadow-[0_4px_24px_rgba(0,0,0,0.1)]
```

> **When to use arbitrary values:** For one-off pixel-precise values that don't fit the design system. If you're using the same arbitrary value in multiple places, add it to `@theme` instead.

---

## COMPONENT PATTERNS

### Button variants

```html
<!-- Primary -->
<button class="bg-blue-600 hover:bg-blue-700 text-white font-medium px-4 py-2 rounded-lg
               active:scale-95 transition-all duration-150 focus-visible:ring-2
               focus-visible:ring-blue-500 focus-visible:ring-offset-2 disabled:opacity-50">
  Primary
</button>

<!-- Secondary / outline -->
<button class="border border-gray-300 hover:border-gray-400 text-gray-700 bg-white
               hover:bg-gray-50 font-medium px-4 py-2 rounded-lg transition-all duration-150">
  Secondary
</button>

<!-- Ghost -->
<button class="text-gray-600 hover:text-gray-900 hover:bg-gray-100 font-medium px-4 py-2 rounded-lg transition-colors">
  Ghost
</button>

<!-- Destructive -->
<button class="bg-red-600 hover:bg-red-700 text-white font-medium px-4 py-2 rounded-lg transition-colors">
  Delete
</button>
```

### Input field

```html
<div class="space-y-1.5">
  <label class="block text-sm font-medium text-gray-700">Email</label>
  <input
    type="email"
    placeholder="you@example.com"
    class="w-full bg-white border border-gray-300 rounded-lg px-3 py-2 text-sm
           placeholder:text-gray-400
           focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent
           disabled:bg-gray-50 disabled:text-gray-500 disabled:cursor-not-allowed
           transition-shadow"
  />
  <p class="text-xs text-red-500">Error message here</p>
</div>
```

### Card

```html
<div class="bg-white rounded-2xl border border-gray-200 shadow-sm overflow-hidden
            hover:shadow-md transition-shadow duration-200">
  <img class="w-full h-48 object-cover" src="..." />
  <div class="p-5">
    <span class="text-xs font-medium uppercase tracking-wider text-blue-600">Category</span>
    <h3 class="mt-1 text-lg font-semibold text-gray-900 line-clamp-2">Card title here</h3>
    <p class="mt-2 text-sm text-gray-500 line-clamp-3">Description text...</p>
    <div class="mt-4 flex items-center justify-between">
      <div class="flex items-center gap-2">
        <img class="w-6 h-6 rounded-full" src="..." />
        <span class="text-sm text-gray-600">Author Name</span>
      </div>
      <span class="text-xs text-gray-400">Mar 23</span>
    </div>
  </div>
</div>
```

### Alert / Toast

```html
<!-- Info -->
<div class="flex gap-3 bg-blue-50 border border-blue-200 rounded-lg p-4">
  <svg class="w-5 h-5 text-blue-500 shrink-0 mt-0.5">...</svg>
  <div>
    <p class="text-sm font-medium text-blue-800">Note</p>
    <p class="text-sm text-blue-700 mt-0.5">This is an informational message.</p>
  </div>
</div>

<!-- Error -->
<div class="flex gap-3 bg-red-50 border border-red-200 rounded-lg p-4">
  <svg class="w-5 h-5 text-red-500 shrink-0 mt-0.5">...</svg>
  <p class="text-sm text-red-700">Something went wrong. Please try again.</p>
</div>
```

### Avatar group

```html
<div class="flex -space-x-2">
  <img class="w-8 h-8 rounded-full border-2 border-white" src="..." />
  <img class="w-8 h-8 rounded-full border-2 border-white" src="..." />
  <img class="w-8 h-8 rounded-full border-2 border-white" src="..." />
  <div class="w-8 h-8 rounded-full border-2 border-white bg-gray-200 flex items-center justify-center text-xs font-medium text-gray-600">
    +4
  </div>
</div>
```

### Empty state

```html
<div class="flex flex-col items-center justify-center py-16 text-center">
  <div class="w-16 h-16 bg-gray-100 rounded-full flex items-center justify-center mb-4">
    <svg class="w-8 h-8 text-gray-400">...</svg>
  </div>
  <h3 class="text-base font-semibold text-gray-900">No results found</h3>
  <p class="mt-1 text-sm text-gray-500 max-w-xs">Try adjusting your search or filter.</p>
  <button class="mt-4 bg-blue-600 text-white px-4 py-2 rounded-lg text-sm font-medium hover:bg-blue-700 transition-colors">
    Clear filters
  </button>
</div>
```

---

## v3 → v4 CLASS CHANGES

| v3 | v4 |
|----|----|
| `shadow-sm` | `shadow-xs` |
| `shadow` | `shadow-sm` |
| `shadow-md` | `shadow-md` ✓ |
| `rounded-sm` | `rounded-xs` |
| `rounded` | `rounded-sm` |
| `blur-sm` | `blur-xs` |
| `blur` | `blur-sm` |
| `ring` | `ring-3` |
| `outline-none` | `outline-hidden` |
| `flex-shrink` | `shrink` |
| `flex-grow` | `grow` |
| `bg-opacity-50` | `bg-black/50` |
| `text-opacity-75` | `text-white/75` |
| `border-opacity-50` | `border-gray-500/50` |
| `overflow-ellipsis` | `text-ellipsis` |
| `decoration-slice` | `box-decoration-slice` |

Upgrade: `bunx @tailwindcss/upgrade`

---

## Resources

- [Docs](https://tailwindcss.com/docs)
- [v4 Overview](https://tailwindcss.com/blog/tailwindcss-v4)
- [Interactive Cheat Sheet](https://nerdcave.com/tailwind-cheat-sheet)
- [Tailwind UI](https://tailwindui.com) (paid components)
- [Heroicons](https://heroicons.com) (free SVG icons made for Tailwind)
- [shadcn/ui](https://ui.shadcn.com) (copy-paste component library)
EOF"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_typescript-eslint.md".into(),
                content: r###"# typescript-eslint — offpkg docs
> **Version**: 8.57.2 · **Runtime**: bun · **npm**: https://www.npmjs.com/package/typescript-eslint  

Tooling which enables you to use TypeScript with ESLint
**Homepage**: https://typescript-eslint.io/packages/typescript-eslint

---

> ✏️ Edit this file freely — it lives in ~/.offpkg/docs/bun/typescript-eslint.md
> Every project you add typescript-eslint to will get YOUR edited version.
> To regenerate from original: `offpkg docs reset typescript-eslint --runtime bun`

## My Notes

<!-- Add your own notes, snippets, team conventions here -->

---

## Installation

```json
{ "dependencies": { "typescript-eslint": "^8.57.2" } }
```

## Import

```typescript
import ... from 'typescript-eslint';
```

## Quick Start

```typescript
// Add your usage example here
```

## Links

- npm: https://www.npmjs.com/package/typescript-eslint
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_typescript.md".into(),
                content: r###"# typescript

> Typed superset of JavaScript  
> **Version:** 5.9.3 · **Runtime:** bun · [npm](https://www.npmjs.com/package/typescript) · [Docs](https://www.typescriptlang.org)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/typescript.md`  
> To regenerate from original: `offpkg docs reset typescript --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add -d typescript
bunx tsc --init   # generates tsconfig.json
```

> **Note:** Bun runs TypeScript natively — you don't need `ts-node` or compilation for running scripts. TypeScript here is for type checking and editor support.

---

## `tsconfig.json` (recommended for Bun projects)

```json
{
  "compilerOptions": {
    "target": "ES2023",
    "module": "ESNext",
    "moduleResolution": "bundler",
    "lib": ["ES2023"],

    "strict": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true,

    "skipLibCheck": true,
    "esModuleInterop": true,
    "allowImportingTsExtensions": true,
    "noEmit": true,

    "outDir": "./dist",
    "rootDir": "./src",
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src/**/*"],
  "exclude": ["node_modules", "dist"]
}
```

**Key options:**
- `strict: true` — enables all strict checks (required by many libraries)
- `noEmit: true` — type-check only, let Bun handle execution
- `moduleResolution: "bundler"` — correct for Bun and Vite
- `noUncheckedIndexedAccess` — array/object access returns `T | undefined`

---

## Type Checking

```bash
# Type-check without emitting files
bunx tsc --noEmit

# Watch mode
bunx tsc --noEmit --watch
```

---

## Core Types

```ts
// Primitives
let name: string = 'Jane'
let age: number = 30
let active: boolean = true
let id: bigint = 9007199254740991n
let sym: symbol = Symbol('id')

// Arrays
let tags: string[] = ['a', 'b']
let tags: Array<string> = ['a', 'b']

// Tuple
let coords: [number, number] = [0, 0]

// Object
let user: { name: string; age?: number } = { name: 'Jane' }

// Union
let value: string | number
let status: 'active' | 'inactive' | 'pending'

// Intersection
type AdminUser = User & { role: 'admin' }

// Any / Unknown / Never
let anything: any
let safe: unknown
function fail(): never { throw new Error('') }

// Null / Undefined
let maybe: string | null = null
let optional: string | undefined
```

---

## Interfaces & Types

```ts
interface User {
  id: number
  email: string
  name?: string           // optional
  readonly createdAt: Date
}

type Status = 'active' | 'inactive'

type ID = string | number

// Extending
interface Admin extends User {
  role: 'admin'
  permissions: string[]
}

// Merging (interface only)
interface Window {
  myCustomProp: string
}
```

---

## Generics

```ts
function identity<T>(value: T): T {
  return value
}

function first<T>(arr: T[]): T | undefined {
  return arr[0]
}

interface ApiResponse<T> {
  data: T
  error: string | null
  status: number
}

// Constraints
function getLength<T extends { length: number }>(item: T): number {
  return item.length
}

// Default generic
type Pagination<T = unknown> = {
  items: T[]
  total: number
  page: number
}
```

---

## Utility Types

```ts
// Make all properties optional
type PartialUser = Partial<User>

// Make all properties required
type RequiredUser = Required<User>

// Make all properties read-only
type ReadonlyUser = Readonly<User>

// Pick specific properties
type PublicUser = Pick<User, 'id' | 'name'>

// Omit specific properties
type UserWithoutPassword = Omit<User, 'password'>

// Record — typed key-value map
type UserMap = Record<string, User>

// Extract / Exclude from union
type StringOrNumber = string | number | boolean
type OnlyStrings = Extract<StringOrNumber, string>    // string
type NoStrings = Exclude<StringOrNumber, string>      // number | boolean

// Return type of a function
type Result = ReturnType<typeof fetchUser>

// Parameters of a function
type Params = Parameters<typeof fetchUser>

// Infer from Promise
type Resolved = Awaited<Promise<User>>

// Non-nullable
type NonNull = NonNullable<string | null | undefined>  // string
```

---

## Type Narrowing

```ts
function process(value: string | number) {
  if (typeof value === 'string') {
    return value.toUpperCase()  // string here
  }
  return value.toFixed(2)       // number here
}

// instanceof
if (error instanceof Error) { error.message }

// in operator
if ('email' in user) { user.email }

// Type predicates
function isUser(obj: unknown): obj is User {
  return typeof obj === 'object' && obj !== null && 'email' in obj
}

// Assertion functions
function assertDefined<T>(val: T | null | undefined): asserts val is T {
  if (val == null) throw new Error('Expected defined value')
}
```

---

## `satisfies` Operator

```ts
const config = {
  port: 3000,
  host: 'localhost',
} satisfies Record<string, string | number>

config.port  // inferred as number, not string | number
```

---

## `as const`

```ts
const ROLES = ['admin', 'user', 'guest'] as const
type Role = typeof ROLES[number]  // 'admin' | 'user' | 'guest'

const config = { port: 3000, host: 'localhost' } as const
// All properties are readonly literal types
```

---

## Resources

- [TypeScript Docs](https://www.typescriptlang.org/docs)
- [tsconfig Reference](https://www.typescriptlang.org/tsconfig)
- [Utility Types](https://www.typescriptlang.org/docs/handbook/utility-types.html)
- [TypeScript Playground](https://www.typescriptlang.org/play)"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_vite.md".into(),
                content: r####"# vite

> Next-generation frontend build tool — instant dev server, fast HMR, optimized builds  
> **Version:** 8.0.1 · **Runtime:** bun · [npm](https://www.npmjs.com/package/vite) · [Docs](https://vite.dev)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/vite.md`  
> To regenerate from original: `offpkg docs reset vite --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
# Scaffold a new project
bun create vite my-app --template react-ts

# Add to existing project
bun add -d vite @vitejs/plugin-react
```

## Templates

```bash
bun create vite my-app --template react-ts      # React + TypeScript
bun create vite my-app --template react         # React + JS
bun create vite my-app --template vue-ts
bun create vite my-app --template svelte-ts
bun create vite my-app --template vanilla-ts
```

---

## Config (`vite.config.ts`)

```ts
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import path from 'path'

export default defineConfig({
  plugins: [react()],

  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },

  server: {
    port: 3000,
    open: true,               // auto-open browser
    proxy: {
      '/api': {
        target: 'http://localhost:8000',
        changeOrigin: true,
      },
    },
  },

  build: {
    outDir: 'dist',
    sourcemap: true,
    minify: 'esbuild',        // 'esbuild' | 'terser' | false
    target: 'es2020',
    chunkSizeWarningLimit: 1000, // KB
  },

  preview: {
    port: 4173,
  },
})
```

---

## CLI Commands

```bash
bun run dev        # start dev server (HMR enabled)
bun run build      # production build → dist/
bun run preview    # preview production build locally
bun run vite       # same as dev
```

---

## Environment Variables

Vite exposes env vars prefixed with `VITE_` to client code:

```bash
# .env
VITE_API_URL=https://api.example.com
VITE_APP_NAME=MyApp
```

```ts
// Access in code
const apiUrl = import.meta.env.VITE_API_URL
const isDev = import.meta.env.DEV          // boolean
const isProd = import.meta.env.PROD        // boolean
const mode = import.meta.env.MODE         // 'development' | 'production'
```

### Multiple `.env` files

```
.env                  # always loaded
.env.local            # always loaded, gitignored
.env.development      # loaded in dev mode
.env.production       # loaded in build mode
.env.production.local # loaded in build mode, gitignored
```

---

## Path Aliases (`@/`)

```ts
// vite.config.ts
resolve: {
  alias: {
    '@': path.resolve(__dirname, './src'),
  },
},
```

```json
// tsconfig.json — must match
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@/*": ["./src/*"]
    }
  }
}
```

```ts
// Usage
import { Button } from '@/components/Button'
import { api } from '@/lib/api'
```

---

## Plugins

```bash
bun add -d @vitejs/plugin-react          # React + Fast Refresh
bun add -d @vitejs/plugin-react-swc      # React + SWC (faster)
bun add -d @tailwindcss/vite             # Tailwind v4
bun add -d vite-plugin-svgr              # import SVG as React components
bun add -d vite-tsconfig-paths           # auto-read tsconfig paths
```

```ts
import react from '@vitejs/plugin-react'
import svgr from 'vite-plugin-svgr'
import tsconfigPaths from 'vite-tsconfig-paths'
import tailwindcss from '@tailwindcss/vite'

plugins: [react(), svgr(), tsconfigPaths(), tailwindcss()]
```

---

## Static Assets

```ts
// Import image — returns URL string
import logo from './assets/logo.png'
<img src={logo} />

// Raw file content
import readme from './README.md?raw'

// SVG as component (requires vite-plugin-svgr)
import Logo from './logo.svg?react'
<Logo className="w-8 h-8" />

// Public folder — served at root, never processed
// Put in /public/ → reference as /logo.png
```

---

## Build Output

```bash
dist/
├── index.html
├── assets/
│   ├── index-[hash].js      # main bundle
│   ├── vendor-[hash].js     # auto code-split chunk
│   └── index-[hash].css
```

### Manual chunks

```ts
build: {
  rollupOptions: {
    output: {
      manualChunks: {
        'react-vendor': ['react', 'react-dom'],
        'query': ['@tanstack/react-query'],
      },
    },
  },
},
```

---

## Resources

- [Vite Docs](https://vite.dev/guide/)
- [Config Reference](https://vite.dev/config/)
- [Plugins](https://vite.dev/plugins/)
- [GitHub](https://github.com/vitejs/vite)"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_zod.md".into(),
                content: r####"# zod

> TypeScript-first schema validation with static type inference  
> **Version:** 4.3.6 · **Runtime:** bun · [npm](https://www.npmjs.com/package/zod) · [Docs](https://zod.dev)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/zod.md`  
> To regenerate from original: `offpkg docs reset zod --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add zod
```

Requires `strict: true` in `tsconfig.json`.

## Import

```ts
import * as z from 'zod'
// or
import { z } from 'zod'
```

> **Zod v4 import:** Use `import { z } from 'zod'` — the named export is new in v4 and preferred.

---

## Core idea

Write the schema once — get validation **and** the TypeScript type for free:

```ts
import { z } from 'zod'

const UserSchema = z.object({
  id: z.number(),
  email: z.string().email(),
  name: z.string().optional(),
})

type User = z.infer<typeof UserSchema>
// { id: number; email: string; name?: string }

// Validate at runtime
const user = UserSchema.parse(untrustedData)      // throws on failure
const result = UserSchema.safeParse(untrustedData) // never throws
```

---

## Parse vs SafeParse

```ts
// parse — throws ZodError if invalid
try {
  const user = UserSchema.parse(data)
} catch (err) {
  if (err instanceof z.ZodError) {
    console.log(err.issues)
  }
}

// safeParse — always returns a result object
const result = UserSchema.safeParse(data)

if (result.success) {
  console.log(result.data)     // typed and validated
} else {
  console.log(result.error.issues)       // array of issues
  console.log(result.error.flatten())    // { fieldErrors, formErrors }
  console.log(result.error.format())     // nested object of errors
}

// Async variants (for async refinements)
const result = await UserSchema.safeParseAsync(data)
```

---

## Primitive Types

```ts
z.string()
z.number()
z.boolean()
z.bigint()
z.date()
z.symbol()
z.undefined()
z.null()
z.void()
z.any()
z.unknown()
z.never()
```

---

## String Validation

```ts
z.string()
  .min(3, 'At least 3 characters')
  .max(100, 'Too long')
  .length(10, 'Exactly 10 characters')
  .email('Invalid email')
  .url('Invalid URL')
  .uuid('Invalid UUID')
  .cuid()
  .cuid2()
  .nanoid()
  .regex(/^\d{4}$/, 'Must be 4 digits')
  .startsWith('https://')
  .endsWith('.com')
  .includes('@')
  .toLowerCase()           // transforms to lowercase
  .toUpperCase()
  .trim()                  // strips whitespace
  .nonempty('Required')    // disallow empty string ''
```

**Common patterns:**
```ts
const emailSchema = z.string().trim().toLowerCase().email()
const slugSchema  = z.string().regex(/^[a-z0-9-]+$/, 'Invalid slug')
const urlSchema   = z.string().url()
const uuidSchema  = z.string().uuid()
const phoneSchema = z.string().regex(/^\+?[\d\s\-()]{7,15}$/)
```

---

## Number Validation

```ts
z.number()
  .int('Must be an integer')
  .positive('Must be positive')
  .negative()
  .nonnegative()
  .nonpositive()
  .gt(0)     .gte(1)    // greater than / greater than or equal
  .lt(100)   .lte(99)   // less than / less than or equal
  .multipleOf(5)
  .safe()               // within Number.MAX_SAFE_INTEGER
  .finite()             // not Infinity
```

**Coerce strings from HTML inputs:**
```ts
// HTML <input type="number"> returns a string — coerce it
const ageSchema = z.coerce.number().int().positive().max(120)
const priceSchema = z.coerce.number().multipleOf(0.01).nonnegative()
```

---

## Object

```ts
const UserSchema = z.object({
  id: z.number(),
  email: z.string().email(),
  name: z.string().optional(),
  role: z.enum(['admin', 'user', 'guest']),
  createdAt: z.date().optional(),
})

// Extend — add fields
const AdminSchema = UserSchema.extend({
  permissions: z.array(z.string()),
})

// Merge two schemas
const MergedSchema = SchemaA.merge(SchemaB)

// Pick — keep only these fields
const PublicUserSchema = UserSchema.pick({ id: true, name: true, email: true })

// Omit — remove these fields
const CreateUserSchema = UserSchema.omit({ id: true, createdAt: true })

// Partial — make all fields optional (great for PATCH endpoints)
const UpdateUserSchema = UserSchema.partial()

// Required — make all fields required
const RequiredSchema = UserSchema.required()

// Passthrough — allow unknown keys (don't strip them)
UserSchema.passthrough()

// Strict — reject unknown keys
z.strictObject({ name: z.string() })

// Strip (default) — remove unknown keys silently
UserSchema.strip()
```

---

## Array

```ts
z.array(z.string())
z.array(z.number()).nonempty('At least one item required')
z.array(z.string()).min(1).max(10)
z.array(z.string()).length(3)

// Typed tuple
const CoordSchema = z.tuple([z.number(), z.number()])
type Coord = z.infer<typeof CoordSchema> // [number, number]

// Rest element
const ArgsSchema = z.tuple([z.string()]).rest(z.number())
// [string, ...number[]]
```

---

## Enum

```ts
// Zod enum (most common)
const RoleSchema = z.enum(['admin', 'user', 'guest'])
type Role = z.infer<typeof RoleSchema> // 'admin' | 'user' | 'guest'

// Access values
RoleSchema.options  // ['admin', 'user', 'guest']
RoleSchema.enum.admin // 'admin'

// From a const array (keeps type narrow)
const ROLES = ['admin', 'user', 'guest'] as const
const RoleSchema = z.enum(ROLES)

// Native TypeScript enum
enum Status { Active, Inactive }
z.nativeEnum(Status)
```

---

## Union & Intersection

```ts
// Union
const IdSchema = z.union([z.string(), z.number()])
const IdSchema = z.string().or(z.number())  // shorthand

// Discriminated union — faster and better errors
const ShapeSchema = z.discriminatedUnion('type', [
  z.object({ type: z.literal('circle'),    radius: z.number() }),
  z.object({ type: z.literal('rect'),      width: z.number(), height: z.number() }),
  z.object({ type: z.literal('triangle'),  base: z.number(), height: z.number() }),
])

// Intersection
const AdminSchema = UserSchema.and(z.object({ adminKey: z.string() }))
```

---

## Optional / Nullable / Default

```ts
z.string().optional()          // string | undefined
z.string().nullable()          // string | null
z.string().nullish()           // string | null | undefined
z.string().default('fallback') // uses 'fallback' when input is undefined
z.string().catch('fallback')   // uses 'fallback' on validation failure (never throws)
```

---

## Literal

```ts
z.literal('admin')     // exactly 'admin'
z.literal(42)
z.literal(true)

// Multiple literals
const StatusSchema = z.union([
  z.literal('active'),
  z.literal('inactive'),
  z.literal('banned'),
])
```

---

## Transform

Transforms run **after** validation and can change the output type.

```ts
// String → number
const NumberFromString = z.string()
  .transform((val) => parseInt(val, 10))
  .pipe(z.number().int())  // validate the result too

// Object → different shape
const ApiUserSchema = z.object({ first_name: z.string(), last_name: z.string() })
  .transform(({ first_name, last_name }) => ({
    name: `${first_name} ${last_name}`,
  }))

// Trim and lowercase email
const emailSchema = z.string()
  .transform((s) => s.trim().toLowerCase())
  .pipe(z.string().email())

// Input vs output types differ after transform
type Input  = z.input<typeof NumberFromString>   // string
type Output = z.output<typeof NumberFromString>  // number
// same as z.infer<> which gives the output
```

---

## Refinements (custom validation)

```ts
// Single refinement
const PasswordSchema = z.string()
  .min(8)
  .refine((val) => /[A-Z]/.test(val), { message: 'Need one uppercase letter' })
  .refine((val) => /[0-9]/.test(val), { message: 'Need one number' })

// Cross-field validation
const SignupSchema = z.object({
  password: z.string().min(8),
  confirm:  z.string(),
}).refine((data) => data.password === data.confirm, {
  message: 'Passwords do not match',
  path: ['confirm'],    // attach error to this field
})

// Async refinement (e.g. check DB)
const EmailSchema = z.string().email().superRefine(async (email, ctx) => {
  const exists = await checkEmailInDb(email)
  if (exists) {
    ctx.addIssue({
      code: z.ZodIssueCode.custom,
      message: 'Email already in use',
    })
  }
})
```

---

## Record & Map

```ts
// Record — typed key-value object
const CacheSchema = z.record(z.string(), z.number())
type Cache = z.infer<typeof CacheSchema> // Record<string, number>

// Record with enum keys
const ScoreSchema = z.record(z.enum(['win', 'loss', 'draw']), z.number())

// Map
const MapSchema = z.map(z.string(), z.number())
// Map<string, number>

// Set
const TagSetSchema = z.set(z.string())
// Set<string>
```

---

## Error Handling in Detail

```ts
const result = UserSchema.safeParse(data)

if (!result.success) {
  // Flat — great for forms
  const flat = result.error.flatten()
  // {
  //   formErrors: [],          // top-level errors
  //   fieldErrors: {           // per-field errors
  //     email: ['Invalid email'],
  //     name: ['Too short'],
  //   }
  // }

  // Format — nested object
  const formatted = result.error.format()
  // { email: { _errors: ['Invalid email'] }, name: { _errors: [...] } }

  // Raw issues array
  result.error.issues.forEach((issue) => {
    console.log(issue.path)    // ['email']
    console.log(issue.message) // 'Invalid email'
    console.log(issue.code)    // 'invalid_string'
  })
}
```

---

## Real-world Patterns

### API request body validation (with Hono)

```ts
import { z } from 'zod'

const CreatePostSchema = z.object({
  title:     z.string().min(1).max(200).trim(),
  content:   z.string().min(1),
  tags:      z.array(z.string()).max(5).default([]),
  published: z.boolean().default(false),
})

app.post('/posts', async (c) => {
  const result = CreatePostSchema.safeParse(await c.req.json())
  if (!result.success) {
    return c.json({ errors: result.error.flatten().fieldErrors }, 400)
  }
  const post = await db.createPost(result.data)
  return c.json(post, 201)
})
```

### Environment variable validation

```ts
const EnvSchema = z.object({
  DATABASE_URL: z.string().url(),
  PORT:         z.coerce.number().default(3000),
  NODE_ENV:     z.enum(['development', 'production', 'test']).default('development'),
  JWT_SECRET:   z.string().min(32),
})

export const env = EnvSchema.parse(process.env)
// Now env.PORT is a number, not a string
```

### Shared schema between server and client

```ts
// shared/schemas.ts
export const UserSchema = z.object({
  id:    z.string().uuid(),
  email: z.string().email(),
  name:  z.string(),
  role:  z.enum(['admin', 'user']),
})

export const CreateUserSchema = UserSchema.omit({ id: true })
export const UpdateUserSchema = UserSchema.partial().omit({ id: true })

export type User       = z.infer<typeof UserSchema>
export type CreateUser = z.infer<typeof CreateUserSchema>
export type UpdateUser = z.infer<typeof UpdateUserSchema>
```

### PATCH endpoint (partial update)

```ts
const UpdateUserSchema = z.object({
  name:  z.string().min(1).optional(),
  email: z.string().email().optional(),
  role:  z.enum(['admin', 'user']).optional(),
}).refine(
  (data) => Object.keys(data).length > 0,
  { message: 'At least one field required' }
)
```

---

## Resources

- [Zod Docs](https://zod.dev)
- [GitHub](https://github.com/colinhacks/zod)
- [v4 Migration Guide](https://zod.dev/v4)
- [Zod Playground](https://zod.dev/play)"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "offpkg_docs/offpkg_zustand.md".into(),
                content: r####"# zustand

> Minimal, fast state management for React  
> **Version:** 5.0.12 · **Runtime:** bun · [npm](https://www.npmjs.com/package/zustand) · [Docs](https://zustand.docs.pmnd.rs)

---

> ✏️ **This file is yours to edit** — it lives at `~/.offpkg/docs/bun/zustand.md`  
> To regenerate from original: `offpkg docs reset zustand --runtime bun`

---

## My Notes

<!-- Add your own notes, snippets, and team conventions here -->

---

## Install

```bash
bun add zustand
```

---

## Usage

### Create a Store

```ts
import { create } from 'zustand'

interface BearState {
  bears: number
  increase: () => void
  reset: () => void
}

const useBearStore = create<BearState>((set) => ({
  bears: 0,
  increase: () => set((state) => ({ bears: state.bears + 1 })),
  reset: () => set({ bears: 0 }),
}))
```

### Use in Components

```tsx
function BearCount() {
  const bears = useBearStore((state) => state.bears)
  return <p>{bears} bears</p>
}

function Controls() {
  const increase = useBearStore((state) => state.increase)
  return <button onClick={increase}>+1</button>
}
```

> No providers needed — works anywhere in the tree.

---

## Selecting State

```ts
// Single value — re-renders only when this value changes
const bears = useBearStore((state) => state.bears)

// Multiple values — use useShallow to avoid unnecessary re-renders
import { useShallow } from 'zustand/react/shallow'

const { bears, fish } = useBearStore(
  useShallow((state) => ({ bears: state.bears, fish: state.fish }))
)

// Fetch all state (re-renders on any change — use sparingly)
const state = useBearStore()
```

---

## Updating State

```ts
// Merge update (shallow merge — does NOT deep merge)
set({ bears: 5 })

// Functional update — read current state
set((state) => ({ bears: state.bears + 1 }))

// Replace entire state (second arg = true)
set({}, true)

// Read state outside of set with get
const useStore = create<State>((set, get) => ({
  count: 0,
  double: () => {
    const current = get().count
    set({ count: current * 2 })
  },
}))
```

---

## Async Actions

```ts
const useStore = create<State>((set) => ({
  users: [],
  loading: false,
  fetchUsers: async () => {
    set({ loading: true })
    const users = await api.getUsers()
    set({ users, loading: false })
  },
}))
```

---

## Outside React

```ts
// Read state without a hook
const bears = useBearStore.getState().bears

// Write state
useBearStore.setState({ bears: 10 })

// Subscribe to all changes
const unsub = useBearStore.subscribe((state) => {
  console.log('new state:', state.bears)
})
unsub() // unsubscribe
```

---

## Middleware

### Persist (localStorage / sessionStorage)

```ts
import { create } from 'zustand'
import { persist, createJSONStorage } from 'zustand/middleware'

const useStore = create(
  persist<State>(
    (set) => ({
      token: null,
      setToken: (token) => set({ token }),
    }),
    {
      name: 'auth-storage',                               // localStorage key
      storage: createJSONStorage(() => sessionStorage),   // default: localStorage
    }
  )
)
```

### Immer (nested state mutations)

```ts
import { immer } from 'zustand/middleware/immer'

const useStore = create(
  immer<State>((set) => ({
    user: { profile: { name: 'Jane' } },
    updateName: (name: string) =>
      set((state) => {
        state.user.profile.name = name  // mutate directly
      }),
  }))
)
```

### DevTools (Redux DevTools extension)

```ts
import { devtools } from 'zustand/middleware'

const useStore = create(
  devtools<State>(
    (set) => ({ bears: 0, increase: () => set({ bears: 1 }) }),
    { name: 'BearStore' }
  )
)
```

### Combining Middleware

```ts
const useStore = create<State>()(
  devtools(
    persist(
      immer((set) => ({
        // your state
      })),
      { name: 'my-store' }
    )
  )
)
```

---

## Slices Pattern (large stores)

```ts
// bears.ts
export const createBearSlice = (set) => ({
  bears: 0,
  addBear: () => set((s) => ({ bears: s.bears + 1 })),
})

// fish.ts
export const createFishSlice = (set) => ({
  fish: 0,
  addFish: () => set((s) => ({ fish: s.fish + 1 })),
})

// store.ts
import { create } from 'zustand'

const useStore = create<BearSlice & FishSlice>((...args) => ({
  ...createBearSlice(...args),
  ...createFishSlice(...args),
}))
```

---

## Resources

- [Docs](https://zustand.docs.pmnd.rs)
- [GitHub](https://github.com/pmndrs/zustand)
- [Guides](https://zustand.docs.pmnd.rs/guides)"####.into(),
                binary_content: None,
            },
            StackFile {
                path: "package.json".into(),
                content: r#"{
  "dependencies": {
    "@fontsource-variable/inter": "^5.2.8",
    "@hookform/resolvers": "^5.2.2",
    "@lordicon/react": "^1.11.0",
    "@tailwindcss/vite": "^4.2.2",
    "@tanstack/react-query": "^5.95.2",
    "axios": "^1.13.6",
    "class-variance-authority": "^0.7.1",
    "clsx": "^2.1.1",
    "framer-motion": "^12.40.0",
    "gsap": "^3.15.0",
    "lenis": "^1.3.23",
    "lottie-react": "^2.4.1",
    "lottie-web": "^5.13.0",
    "lucide-react": "^1.17.0",
    "radix-ui": "^1.5.0",
    "react": "^19.2.4",
    "react-dom": "^19.2.4",
    "react-hook-form": "^7.72.0",
    "react-router-dom": "^7.13.2",
    "shadcn": "^4.10.0",
    "tailwind-merge": "^3.6.0",
    "tailwindcss": "^4.2.2",
    "tw-animate-css": "^1.4.0",
    "zod": "^4.3.6",
    "zustand": "^5.0.12"
  },
  "devDependencies": {
    "@eslint/js": "^10.0.1",
    "@types/node": "^25.5.0",
    "@types/react": "^19.2.14",
    "@types/react-dom": "^19.2.3",
    "@vitejs/plugin-react": "^6.0.1",
    "eslint": "^10.1.0",
    "eslint-plugin-react-hooks": "^7.0.1",
    "eslint-plugin-react-refresh": "^0.5.2",
    "globals": "^17.4.0",
    "typescript": "^6.0.2",
    "typescript-eslint": "^8.57.2",
    "vite": "^8.0.2"
  },
  "name": "offpkg-vite-react",
  "private": true,
  "scripts": {
    "build": "tsc -b && vite build",
    "dev": "vite",
    "lint": "eslint .",
    "preview": "vite preview"
  },
  "type": "module",
  "version": "0.0.0"
}"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "public/favicon.svg".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/favicon.svg").to_vec()),
            },
            StackFile {
                path: "public/icons.svg".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/icons.svg").to_vec()),
            },
            StackFile {
                path: "src/App.css".into(),
                content: r##".counter {
  font-size: 16px;
  padding: 5px 10px;
  border-radius: 5px;
  color: var(--accent);
  background: var(--accent-bg);
  border: 2px solid transparent;
  transition: border-color 0.3s;
  margin-bottom: 24px;

  &:hover {
    border-color: var(--accent-border);
  }
  &:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 2px;
  }
}

.hero {
  position: relative;

  .base,
  .framework,
  .vite {
    inset-inline: 0;
    margin: 0 auto;
  }

  .base {
    width: 170px;
    position: relative;
    z-index: 0;
  }

  .framework,
  .vite {
    position: absolute;
  }

  .framework {
    z-index: 1;
    top: 34px;
    height: 28px;
    transform: perspective(2000px) rotateZ(300deg) rotateX(44deg) rotateY(39deg)
      scale(1.4);
  }

  .vite {
    z-index: 0;
    top: 107px;
    height: 26px;
    width: auto;
    transform: perspective(2000px) rotateZ(300deg) rotateX(40deg) rotateY(39deg)
      scale(0.8);
  }
}

#center {
  display: flex;
  flex-direction: column;
  gap: 25px;
  place-content: center;
  place-items: center;
  flex-grow: 1;

  @media (max-width: 1024px) {
    padding: 32px 20px 24px;
    gap: 18px;
  }
}

#next-steps {
  display: flex;
  border-top: 1px solid var(--border);
  text-align: left;

  & > div {
    flex: 1 1 0;
    padding: 32px;
    @media (max-width: 1024px) {
      padding: 24px 20px;
    }
  }

  .icon {
    margin-bottom: 16px;
    width: 22px;
    height: 22px;
  }

  @media (max-width: 1024px) {
    flex-direction: column;
    text-align: center;
  }
}

#docs {
  border-right: 1px solid var(--border);

  @media (max-width: 1024px) {
    border-right: none;
    border-bottom: 1px solid var(--border);
  }
}

#next-steps ul {
  list-style: none;
  padding: 0;
  display: flex;
  gap: 8px;
  margin: 32px 0 0;

  .logo {
    height: 18px;
  }

  a {
    color: var(--text-h);
    font-size: 16px;
    border-radius: 6px;
    background: var(--social-bg);
    display: flex;
    padding: 6px 12px;
    align-items: center;
    gap: 8px;
    text-decoration: none;
    transition: box-shadow 0.3s;

    &:hover {
      box-shadow: var(--shadow);
    }
    .button-icon {
      height: 18px;
      width: 18px;
    }
  }

  @media (max-width: 1024px) {
    margin-top: 20px;
    flex-wrap: wrap;
    justify-content: center;

    li {
      flex: 1 1 calc(50% - 8px);
    }

    a {
      width: 100%;
      justify-content: center;
      box-sizing: border-box;
    }
  }
}

#spacer {
  height: 88px;
  border-top: 1px solid var(--border);
  @media (max-width: 1024px) {
    height: 48px;
  }
}

.ticks {
  position: relative;
  width: 100%;

  &::before,
  &::after {
    content: '';
    position: absolute;
    top: -4.5px;
    border: 5px solid transparent;
  }

  &::before {
    left: 0;
    border-left-color: var(--border);
  }
  &::after {
    right: 0;
    border-right-color: var(--border);
  }
}
"##.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/App.tsx".into(),
                content: r#"import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import { RootLayout } from '@/layouts/RootLayout';
import { HomePage } from '@/pages/HomePage';
import { AboutPage } from '@/pages/AboutPage';
import { ContactPage } from '@/pages/ContactPage';
import { NotFoundPage } from '@/pages/NotFoundPage';
import './App.css';

const router = createBrowserRouter([
  {
    path: '/',
    element: <RootLayout />,
    children: [
      {
        index: true,
        element: <HomePage />,
      },
      {
        path: 'about',
        element: <AboutPage />,
      },
      {
        path: 'contact',
        element: <ContactPage />,
      },
      {
        path: '*',
        element: <NotFoundPage />,
      },
    ],
  },
]);

function App() {
  return <RouterProvider router={router} />;
}

export default App;



"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/api/axios.ts".into(),
                content: r#"import axios from 'axios';
import { logger } from '../lib/logger';

const api = axios.create({
  baseURL: import.meta.env.VITE_API_URL || 'https://api.example.com',
  headers: {
    'Content-Type': 'application/json',
  },
});

api.interceptors.request.use(
  (config) => {
    logger.info(`Request: ${config.method?.toUpperCase()} ${config.url}`);
    return config;
  },
  (error) => {
    logger.error('Request Error', error);
    return Promise.reject(error);
  }
);

api.interceptors.response.use(
  (response) => {
    logger.info(`Response: ${response.status} ${response.config.url}`);
    return response;
  },
  (error) => {
    logger.error('Response Error', error.response?.data || error.message);
    return Promise.reject(error);
  }
);

export default api;
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/assets/hero.png".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/hero.png").to_vec()),
            },
            StackFile {
                path: "src/assets/react.svg".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/react.svg").to_vec()),
            },
            StackFile {
                path: "src/assets/vite.svg".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/vite.svg").to_vec()),
            },
            StackFile {
                path: "src/components/Footer.tsx".into(),
                content: r##"import { Link } from 'react-router-dom';
import { LordIcon } from '@/components/LordIcon';
import { Layers, Heart } from 'lucide-react';

export function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="w-full border-t border-border/40 bg-background/50 backdrop-blur-md relative z-10 py-12 px-6 mt-12">
      <div className="max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-4 gap-8">
        
        {/* Brand Column */}
        <div className="md:col-span-2 space-y-4">
          <Link to="/" className="inline-flex items-center gap-2 text-xl font-heading font-extrabold tracking-tight text-primary hover:opacity-80 transition-opacity">
            <LordIcon src="https://cdn.lordicon.com/nocovwne.json" size={28} colors="primary:var(--color-primary),secondary:currentColor" />
            <span>OFFPKG</span>
          </Link>
          <p className="text-muted-foreground text-sm leading-relaxed max-w-sm font-body">
            The ultimate developer setup with Vite, React, GSAP, Framer Motion, and shadcn/ui. Build premium, highly-interactive web experiences offline.
          </p>
          <div className="flex items-center gap-4 pt-2">
            <a 
              href="https://github.com/aswin402/offpkg" 
              target="_blank" 
              rel="noreferrer" 
              className="w-9 h-9 rounded-xl border border-border/50 bg-card/40 flex items-center justify-center text-muted-foreground hover:text-foreground hover:border-primary/40 hover:shadow-md hover:shadow-primary/5 transition-all duration-300"
            >
              <svg viewBox="0 0 24 24" width="18" height="18" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round" className="w-4.5 h-4.5">
                <path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" />
                <path d="M9 18c-4.51 2-5-2-7-2" />
              </svg>
            </a>
            <a 
              href="#" 
              className="w-9 h-9 rounded-xl border border-border/50 bg-card/40 flex items-center justify-center text-muted-foreground hover:text-foreground hover:border-primary/40 hover:shadow-md hover:shadow-primary/5 transition-all duration-300"
            >
              <svg viewBox="0 0 24 24" width="18" height="18" stroke="currentColor" strokeWidth="2" fill="none" strokeLinecap="round" strokeLinejoin="round" className="w-4.5 h-4.5">
                <path d="M22 4s-.7 2.1-2 3.4c1.6 10-9.4 17.3-18 11.6 2.2.1 4.4-.6 6-2C3 15.5.5 9.6 3 5c2.2 2.6 5.6 4.1 9 4-.9-4.2 4-6.6 7-3.8 1.1 0 3-1.2 3-1.2z" />
              </svg>
            </a>
            <a 
              href="#" 
              className="w-9 h-9 rounded-xl border border-border/50 bg-card/40 flex items-center justify-center text-muted-foreground hover:text-foreground hover:border-primary/40 hover:shadow-md hover:shadow-primary/5 transition-all duration-300"
            >
              <Layers className="w-4.5 h-4.5" />
            </a>
          </div>
        </div>

        {/* Links Column */}
        <div className="space-y-4">
          <h4 className="font-heading font-bold text-sm uppercase tracking-wider text-foreground/80">Navigation</h4>
          <ul className="space-y-2.5 font-body text-sm">
            <li>
              <Link to="/" className="text-muted-foreground hover:text-primary transition-colors flex items-center gap-1.5 group">
                <span>Home</span>
              </Link>
            </li>
            <li>
              <Link to="/about" className="text-muted-foreground hover:text-primary transition-colors flex items-center gap-1.5 group">
                <span>About Stack</span>
              </Link>
            </li>
            <li>
              <Link to="/contact" className="text-muted-foreground hover:text-primary transition-colors flex items-center gap-1.5 group">
                <span>Contact US</span>
              </Link>
            </li>
          </ul>
        </div>

        {/* Stack Info Column */}
        <div className="space-y-4">
          <h4 className="font-heading font-bold text-sm uppercase tracking-wider text-foreground/80">Animations Stack</h4>
          <ul className="space-y-2 text-muted-foreground text-xs leading-relaxed font-body">
            <li>🟢 <strong className="text-foreground">GSAP:</strong> ScrollTrigger & Timeline Controls</li>
            <li>🟢 <strong className="text-foreground">Framer Motion:</strong> Entrance & Exit transitions</li>
            <li>🟢 <strong className="text-foreground">Lenis Scroll:</strong> Butter-smooth inertia physics</li>
            <li>🟢 <strong className="text-foreground">Lordicons:</strong> Animated kinetic JSON player</li>
          </ul>
        </div>

      </div>

      {/* Bottom Copyright bar */}
      <div className="max-w-6xl mx-auto border-t border-border/40 mt-10 pt-6 flex flex-col md:flex-row items-center justify-between gap-4 font-body text-xs text-muted-foreground">
        <p>© {currentYear} Offpkg Kinetic Template. All rights reserved.</p>
        <p className="flex items-center gap-1">
          Made with <Heart className="w-3.5 h-3.5 text-red-500 fill-red-500 animate-pulse" /> for high performance offline development.
        </p>
      </div>
    </footer>
  );
}
"##.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/LordIcon.tsx".into(),
                content: r##"import { useEffect, useRef, useState } from 'react';
import { Player } from '@lordicon/react';
import { useThemeStore } from '@/store/useThemeStore';

interface LordIconProps {
  src: string;
  size?: number;
  trigger?: 'hover' | 'click' | 'loop';
  colors?: string; // e.g. "primary:currentColor,secondary:var(--color-primary)"
  className?: string;
  delay?: number;
}

// Convert any browser-resolved CSS color string (oklch, color(), hsl, rgb) to hex format using a 1x1 canvas
function resolveToHex(colorStr: string, container: HTMLElement): string {
  // 1. Let the browser resolve the color (handles currentColor and CSS variables)
  const temp = document.createElement('div');
  temp.style.color = colorStr;
  container.appendChild(temp);
  const computedColor = window.getComputedStyle(temp).color;
  container.removeChild(temp);

  // 2. Use a canvas to convert the resolved color value to sRGB
  const canvas = document.createElement('canvas');
  canvas.width = 1;
  canvas.height = 1;
  const ctx = canvas.getContext('2d');
  if (!ctx) return '#000000';

  ctx.fillStyle = computedColor;
  ctx.fillRect(0, 0, 1, 1);

  const [r, g, b] = ctx.getImageData(0, 0, 1, 1).data;

  // 3. Format as Hex
  return '#' + ((1 << 24) + (r << 16) + (g << 8) + b).toString(16).slice(1);
}

export function LordIcon({ 
  src, 
  size = 32, 
  trigger = 'hover', 
  colors = 'primary:currentColor,secondary:var(--color-primary)', 
  className, 
  delay = 0 
}: LordIconProps) {
  const playerRef = useRef<Player>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const [iconData, setIconData] = useState<any>(null);
  const [isLoading, setIsLoading] = useState(true);
  const [resolvedColors, setResolvedColors] = useState<string | undefined>(undefined);
  
  // Hook into theme store to trigger re-renders on theme toggle
  const { theme } = useThemeStore();

  useEffect(() => {
    setIsLoading(true);
    fetch(src)
      .then((res) => {
        if (!res.ok) throw new Error(`HTTP error! status: ${res.status}`);
        return res.json();
      })
      .then((data) => {
        setIconData(data);
        setIsLoading(false);
      })
      .catch((err) => {
        console.error('Failed to load lordicon from url:', src, err);
        setIsLoading(false);
      });
  }, [src]);

  // Resolve CSS color variables and currentColors dynamically to HEX
  useEffect(() => {
    if (!colors || isLoading || !containerRef.current) {
      setResolvedColors(colors);
      return;
    }

    const container = containerRef.current;
    
    // We add a tiny delay to ensure ThemeProvider has updated the HTML class list and variables in the DOM tree
    const timer = setTimeout(() => {
      const parts = colors.split(',');
      const resolvedParts = parts.map((part) => {
        const [key, value] = part.split(':');
        if (!key || !value) return part;

        const hexColor = resolveToHex(value.trim(), container);
        return `${key}:${hexColor}`;
      });

      setResolvedColors(resolvedParts.join(','));
    }, 50); // 50ms delay is enough to let the DOM class rewrite commit

    return () => clearTimeout(timer);
  }, [colors, isLoading, theme]);

  useEffect(() => {
    if (!isLoading && iconData && trigger === 'loop') {
      const timer = setTimeout(() => {
        playerRef.current?.play();
      }, delay);
      return () => clearTimeout(timer);
    }
  }, [isLoading, iconData, trigger, delay]);

  const handleMouseEnter = () => {
    if (trigger === 'hover' && !isLoading && playerRef.current) {
      playerRef.current.playFromBeginning();
    }
  };

  const handleClick = () => {
    if (trigger === 'click' && !isLoading && playerRef.current) {
      playerRef.current.playFromBeginning();
    }
  };

  if (isLoading || !iconData) {
    return (
      <div 
        style={{ width: size, height: size }} 
        className={`inline-flex items-center justify-center rounded-full bg-muted/20 animate-pulse ${className || ''}`} 
      />
    );
  }

  return (
    <div
      ref={containerRef}
      onMouseEnter={handleMouseEnter}
      onClick={handleClick}
      style={{ width: size, height: size }}
      className={`inline-flex items-center justify-center cursor-pointer transition-transform duration-200 hover:scale-110 active:scale-95 ${className || ''}`}
    >
      <Player
        ref={playerRef}
        icon={iconData}
        size={size}
        colors={resolvedColors}
        onComplete={() => {
          if (trigger === 'loop' && playerRef.current) {
            playerRef.current.play();
          }
        }}
      />
    </div>
  );
}
"##.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/LottieAnimation.tsx".into(),
                content: r#"import { useEffect, useState } from 'react';
import Lottie from 'lottie-react';

interface LottieAnimationProps {
  src: string;
  className?: string;
  loop?: boolean;
}

// Handle Vite ESM/CommonJS default export wrapper mismatch
const LottieComponent = (Lottie as any).default || Lottie;

export function LottieAnimation({ src, className, loop = true }: LottieAnimationProps) {
  const [animationData, setAnimationData] = useState<any>(null);
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    setIsLoading(true);
    fetch(src)
      .then((res) => {
        if (!res.ok) throw new Error(`Failed to fetch Lottie JSON: ${res.status}`);
        return res.json();
      })
      .then((data) => {
        setAnimationData(data);
        setIsLoading(false);
      })
      .catch((err) => {
        console.error('Failed to load Lottie animation from URL:', src, err);
        setIsLoading(false);
      });
  }, [src]);

  if (isLoading || !animationData) {
    return (
      <div 
        className={`w-full h-full min-h-[200px] flex items-center justify-center rounded-2xl bg-muted/20 animate-pulse ${className || ''}`} 
      />
    );
  }

  return (
    <div className={className}>
      <LottieComponent animationData={animationData} loop={loop} />
    </div>
  );
}

"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/Navbar.tsx".into(),
                content: r#"import { Link } from 'react-router-dom';
import { ThemeToggleButton } from '@/components/ThemeToggleButton';
import { LordIcon } from '@/components/LordIcon';

export function Navbar() {
  return (
    <nav className="fixed top-0 left-0 right-0 h-16 border-b border-border/40 bg-background/80 backdrop-blur-md z-50 flex items-center justify-between px-6">
      <div className="flex items-center gap-8">
        <Link to="/" className="text-xl font-heading font-bold tracking-tight text-primary transition-opacity hover:opacity-80 flex items-center gap-2">
          <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/doc/logo.svg" alt="Offpkg Logo" className="w-7 h-7" />
          <span>OFFPKG</span>
        </Link>
        <div className="hidden md:flex items-center gap-6">
          <Link to="/" className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors flex items-center gap-2">
            <LordIcon src="https://cdn.lordicon.com/wmwqvixz.json" size={20} colors="primary:currentColor,secondary:currentColor" />
            <span>Home</span>
          </Link>
          <Link to="/about" className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors flex items-center gap-2">
            <LordIcon src="https://cdn.lordicon.com/xzalkbkz.json" size={20} colors="primary:currentColor,secondary:currentColor" />
            <span>About</span>
          </Link>
          <Link to="/contact" className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors flex items-center gap-2">
            <LordIcon src="https://cdn.lordicon.com/pithnlch.json" size={20} colors="primary:currentColor,secondary:currentColor" />
            <span>Contact</span>
          </Link>
        </div>
      </div>
      <ThemeToggleButton />
    </nav>
  );
}


"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/ThemeProvider.tsx".into(),
                content: r#"import React, { useEffect } from 'react';
import { useThemeStore } from '../store/useThemeStore';

interface ThemeProviderProps {
  children: React.ReactNode;
  inlineTheme?: Record<string, string>;
}

export function ThemeProvider({
  children,
  inlineTheme,
}: ThemeProviderProps) {
  const { theme } = useThemeStore();

  useEffect(() => {
    const root = window.document.documentElement;

    root.classList.remove('light', 'dark');

    if (theme === 'system') {
      const systemTheme = window.matchMedia('(prefers-color-scheme: dark)').matches
        ? 'dark'
        : 'light';

      root.classList.add(systemTheme);
      return;
    }

    root.classList.add(theme);
  }, [theme]);

  return (
    <div style={inlineTheme as React.CSSProperties}>
      {children}
    </div>
  );
}
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/ThemeToggleButton.tsx".into(),
                content: r#"import { Moon, Sun} from 'lucide-react';
import { useThemeStore } from '../store/useThemeStore';
import Button from './button';


export function ThemeToggleButton() {
  const { theme, setTheme } = useThemeStore();

  const cycleTheme = () => {
    if (theme === 'light') setTheme('dark');
    else setTheme('light');
  };

  return (
    <Button
      variant="outline"
      size="icon"
      onClick={cycleTheme}
      title={`Current: ${theme} • Click to cycle`}
      className="fixed top-4 right-4 h-10 w-10 rounded-full"
    >
      <Sun
        className={`h-[1.2rem] w-[1.2rem] transition-all ${
          theme === 'light'
            ? 'rotate-0 scale-100'
            : 'rotate-90 scale-0'
        }`}
      />
      <Moon
        className={`absolute h-[1.2rem] w-[1.2rem] transition-all ${
          theme === 'dark'
            ? 'rotate-0 scale-100'
            : 'rotate-90 scale-0'
        }`}
      />
      <span className="sr-only">Toggle theme</span>
    </Button>
  );
}"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/button.tsx".into(),
                content: r#"import { forwardRef, type ButtonHTMLAttributes } from 'react';

type ButtonVariant = 'primary' | 'secondary' | 'outline' | 'ghost';
type ButtonSize = 'default' | 'sm' | 'lg' | 'icon';

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: ButtonVariant;
  size?: ButtonSize;
}

const Button = forwardRef<HTMLButtonElement, ButtonProps>(
  ({ 
    children, 
    variant = 'primary', 
    size = 'default', 
    className = '', 
    ...props 
  }, ref) => {
    
    const baseClasses = "inline-flex items-center justify-center font-medium font-body rounded-button transition-all active:scale-[0.98] disabled:opacity-50 disabled:cursor-not-allowed focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2 focus-visible:ring-primary";

    const variantClasses = {
      primary: "bg-primary text-primary-foreground shadow-sm hover:shadow-md hover:bg-primary/90",
      secondary: "bg-secondary text-secondary-foreground hover:bg-secondary/80",
      outline: "border border-border bg-transparent hover:bg-muted hover:text-foreground",
      ghost: "hover:bg-muted hover:text-foreground",
    };

    const sizeClasses = {
      default: "px-4 py-2 text-sm",
      sm: "px-3 py-1.5 text-xs",
      lg: "px-6 py-3 text-base",
      icon: "h-10 w-10 p-0",
    };

    return (
      <button
        ref={ref}
        className={`${baseClasses} ${variantClasses[variant]} ${sizeClasses[size]} ${className}`}
        {...props}
      >
        {children}
      </button>
    );
  }
);

Button.displayName = "Button";

export default Button;"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/ui/accordion.tsx".into(),
                content: r#"import * as React from "react"
import { Accordion as AccordionPrimitive } from "radix-ui"

import { cn } from "@/lib/utils"
import { ChevronDownIcon, ChevronUpIcon } from "lucide-react"

function Accordion({
  className,
  ...props
}: React.ComponentProps<typeof AccordionPrimitive.Root>) {
  return (
    <AccordionPrimitive.Root
      data-slot="accordion"
      className={cn("flex w-full flex-col", className)}
      {...props}
    />
  )
}

function AccordionItem({
  className,
  ...props
}: React.ComponentProps<typeof AccordionPrimitive.Item>) {
  return (
    <AccordionPrimitive.Item
      data-slot="accordion-item"
      className={cn("not-last:border-b", className)}
      {...props}
    />
  )
}

function AccordionTrigger({
  className,
  children,
  ...props
}: React.ComponentProps<typeof AccordionPrimitive.Trigger>) {
  return (
    <AccordionPrimitive.Header className="flex">
      <AccordionPrimitive.Trigger
        data-slot="accordion-trigger"
        className={cn(
          "group/accordion-trigger relative flex flex-1 items-start justify-between rounded-lg border border-transparent py-2.5 text-left text-sm font-medium transition-all outline-none hover:underline focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:after:border-ring disabled:pointer-events-none disabled:opacity-50 **:data-[slot=accordion-trigger-icon]:ml-auto **:data-[slot=accordion-trigger-icon]:size-4 **:data-[slot=accordion-trigger-icon]:text-muted-foreground",
          className
        )}
        {...props}
      >
        {children}
        <ChevronDownIcon data-slot="accordion-trigger-icon" className="pointer-events-none shrink-0 group-aria-expanded/accordion-trigger:hidden" />
        <ChevronUpIcon data-slot="accordion-trigger-icon" className="pointer-events-none hidden shrink-0 group-aria-expanded/accordion-trigger:inline" />
      </AccordionPrimitive.Trigger>
    </AccordionPrimitive.Header>
  )
}

function AccordionContent({
  className,
  children,
  ...props
}: React.ComponentProps<typeof AccordionPrimitive.Content>) {
  return (
    <AccordionPrimitive.Content
      data-slot="accordion-content"
      className="overflow-hidden text-sm data-open:animate-accordion-down data-closed:animate-accordion-up"
      {...props}
    >
      <div
        className={cn(
          "h-(--radix-accordion-content-height) pt-0 pb-2.5 [&_a]:underline [&_a]:underline-offset-3 [&_a]:hover:text-foreground [&_p:not(:last-child)]:mb-4",
          className
        )}
      >
        {children}
      </div>
    </AccordionPrimitive.Content>
  )
}

export { Accordion, AccordionItem, AccordionTrigger, AccordionContent }
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/ui/button.tsx".into(),
                content: r#"import * as React from "react"
import { cva, type VariantProps } from "class-variance-authority"
import { Slot } from "radix-ui"

import { cn } from "@/lib/utils"

const buttonVariants = cva(
  "group/button inline-flex shrink-0 items-center justify-center rounded-lg border border-transparent bg-clip-padding text-sm font-medium whitespace-nowrap transition-all outline-none select-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 active:not-aria-[haspopup]:translate-y-px disabled:pointer-events-none disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-3 aria-invalid:ring-destructive/20 dark:aria-invalid:border-destructive/50 dark:aria-invalid:ring-destructive/40 [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
  {
    variants: {
      variant: {
        default: "bg-primary text-primary-foreground hover:bg-primary/80",
        outline:
          "border-border bg-background hover:bg-muted hover:text-foreground aria-expanded:bg-muted aria-expanded:text-foreground dark:border-input dark:bg-input/30 dark:hover:bg-input/50",
        secondary:
          "bg-secondary text-secondary-foreground hover:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)] aria-expanded:bg-secondary aria-expanded:text-secondary-foreground",
        ghost:
          "hover:bg-muted hover:text-foreground aria-expanded:bg-muted aria-expanded:text-foreground dark:hover:bg-muted/50",
        destructive:
          "bg-destructive/10 text-destructive hover:bg-destructive/20 focus-visible:border-destructive/40 focus-visible:ring-destructive/20 dark:bg-destructive/20 dark:hover:bg-destructive/30 dark:focus-visible:ring-destructive/40",
        link: "text-primary underline-offset-4 hover:underline",
      },
      size: {
        default:
          "h-8 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2",
        xs: "h-6 gap-1 rounded-[min(var(--radius-md),10px)] px-2 text-xs in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3",
        sm: "h-7 gap-1 rounded-[min(var(--radius-md),12px)] px-2.5 text-[0.8rem] in-data-[slot=button-group]:rounded-lg has-data-[icon=inline-end]:pr-1.5 has-data-[icon=inline-start]:pl-1.5 [&_svg:not([class*='size-'])]:size-3.5",
        lg: "h-9 gap-1.5 px-2.5 has-data-[icon=inline-end]:pr-2 has-data-[icon=inline-start]:pl-2",
        icon: "size-8",
        "icon-xs":
          "size-6 rounded-[min(var(--radius-md),10px)] in-data-[slot=button-group]:rounded-lg [&_svg:not([class*='size-'])]:size-3",
        "icon-sm":
          "size-7 rounded-[min(var(--radius-md),12px)] in-data-[slot=button-group]:rounded-lg",
        "icon-lg": "size-9",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
)

function Button({
  className,
  variant = "default",
  size = "default",
  asChild = false,
  ...props
}: React.ComponentProps<"button"> &
  VariantProps<typeof buttonVariants> & {
    asChild?: boolean
  }) {
  const Comp = asChild ? Slot.Root : "button"

  return (
    <Comp
      data-slot="button"
      data-variant={variant}
      data-size={size}
      className={cn(buttonVariants({ variant, size, className }))}
      {...props}
    />
  )
}

export { Button, buttonVariants }
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/ui/dialog.tsx".into(),
                content: r#"import * as React from "react"
import { Dialog as DialogPrimitive } from "radix-ui"

import { cn } from "@/lib/utils"
import { Button } from "@/components/ui/button"
import { XIcon } from "lucide-react"

function Dialog({
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Root>) {
  return <DialogPrimitive.Root data-slot="dialog" {...props} />
}

function DialogTrigger({
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Trigger>) {
  return <DialogPrimitive.Trigger data-slot="dialog-trigger" {...props} />
}

function DialogPortal({
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Portal>) {
  return <DialogPrimitive.Portal data-slot="dialog-portal" {...props} />
}

function DialogClose({
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Close>) {
  return <DialogPrimitive.Close data-slot="dialog-close" {...props} />
}

function DialogOverlay({
  className,
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Overlay>) {
  return (
    <DialogPrimitive.Overlay
      data-slot="dialog-overlay"
      className={cn(
        "fixed inset-0 isolate z-50 bg-black/10 duration-100 supports-backdrop-filter:backdrop-blur-xs data-open:animate-in data-open:fade-in-0 data-closed:animate-out data-closed:fade-out-0",
        className
      )}
      {...props}
    />
  )
}

function DialogContent({
  className,
  children,
  showCloseButton = true,
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Content> & {
  showCloseButton?: boolean
}) {
  return (
    <DialogPortal>
      <DialogOverlay />
      <DialogPrimitive.Content
        data-slot="dialog-content"
        className={cn(
          "fixed top-1/2 left-1/2 z-50 grid w-full max-w-[calc(100%-2rem)] -translate-x-1/2 -translate-y-1/2 gap-4 rounded-xl bg-popover p-4 text-sm text-popover-foreground ring-1 ring-foreground/10 duration-100 outline-none sm:max-w-sm data-open:animate-in data-open:fade-in-0 data-open:zoom-in-95 data-closed:animate-out data-closed:fade-out-0 data-closed:zoom-out-95",
          className
        )}
        {...props}
      >
        {children}
        {showCloseButton && (
          <DialogPrimitive.Close data-slot="dialog-close" asChild>
            <Button
              variant="ghost"
              className="absolute top-2 right-2"
              size="icon-sm"
            >
              <XIcon
              />
              <span className="sr-only">Close</span>
            </Button>
          </DialogPrimitive.Close>
        )}
      </DialogPrimitive.Content>
    </DialogPortal>
  )
}

function DialogHeader({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="dialog-header"
      className={cn("flex flex-col gap-2", className)}
      {...props}
    />
  )
}

function DialogFooter({
  className,
  showCloseButton = false,
  children,
  ...props
}: React.ComponentProps<"div"> & {
  showCloseButton?: boolean
}) {
  return (
    <div
      data-slot="dialog-footer"
      className={cn(
        "-mx-4 -mb-4 flex flex-col-reverse gap-2 rounded-b-xl border-t bg-muted/50 p-4 sm:flex-row sm:justify-end",
        className
      )}
      {...props}
    >
      {children}
      {showCloseButton && (
        <DialogPrimitive.Close asChild>
          <Button variant="outline">Close</Button>
        </DialogPrimitive.Close>
      )}
    </div>
  )
}

function DialogTitle({
  className,
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Title>) {
  return (
    <DialogPrimitive.Title
      data-slot="dialog-title"
      className={cn(
        "font-heading text-base leading-none font-medium",
        className
      )}
      {...props}
    />
  )
}

function DialogDescription({
  className,
  ...props
}: React.ComponentProps<typeof DialogPrimitive.Description>) {
  return (
    <DialogPrimitive.Description
      data-slot="dialog-description"
      className={cn(
        "text-sm text-muted-foreground *:[a]:underline *:[a]:underline-offset-3 *:[a]:hover:text-foreground",
        className
      )}
      {...props}
    />
  )
}

export {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogOverlay,
  DialogPortal,
  DialogTitle,
  DialogTrigger,
}
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/components/ui/tabs.tsx".into(),
                content: r#""use client"

import * as React from "react"
import { cva, type VariantProps } from "class-variance-authority"
import { Tabs as TabsPrimitive } from "radix-ui"

import { cn } from "@/lib/utils"

function Tabs({
  className,
  orientation = "horizontal",
  ...props
}: React.ComponentProps<typeof TabsPrimitive.Root>) {
  return (
    <TabsPrimitive.Root
      data-slot="tabs"
      data-orientation={orientation}
      className={cn(
        "group/tabs flex gap-2 data-horizontal:flex-col",
        className
      )}
      {...props}
    />
  )
}

const tabsListVariants = cva(
  "group/tabs-list inline-flex w-fit items-center justify-center rounded-lg p-[3px] text-muted-foreground group-data-horizontal/tabs:h-8 group-data-vertical/tabs:h-fit group-data-vertical/tabs:flex-col data-[variant=line]:rounded-none",
  {
    variants: {
      variant: {
        default: "bg-muted",
        line: "gap-1 bg-transparent",
      },
    },
    defaultVariants: {
      variant: "default",
    },
  }
)

function TabsList({
  className,
  variant = "default",
  ...props
}: React.ComponentProps<typeof TabsPrimitive.List> &
  VariantProps<typeof tabsListVariants>) {
  return (
    <TabsPrimitive.List
      data-slot="tabs-list"
      data-variant={variant}
      className={cn(tabsListVariants({ variant }), className)}
      {...props}
    />
  )
}

function TabsTrigger({
  className,
  ...props
}: React.ComponentProps<typeof TabsPrimitive.Trigger>) {
  return (
    <TabsPrimitive.Trigger
      data-slot="tabs-trigger"
      className={cn(
        "relative inline-flex h-[calc(100%-1px)] flex-1 items-center justify-center gap-1.5 rounded-md border border-transparent px-1.5 py-0.5 text-sm font-medium whitespace-nowrap text-foreground/60 transition-all group-data-vertical/tabs:w-full group-data-vertical/tabs:justify-start hover:text-foreground focus-visible:border-ring focus-visible:ring-[3px] focus-visible:ring-ring/50 focus-visible:outline-1 focus-visible:outline-ring disabled:pointer-events-none disabled:opacity-50 has-data-[icon=inline-end]:pr-1 has-data-[icon=inline-start]:pl-1 dark:text-muted-foreground dark:hover:text-foreground group-data-[variant=default]/tabs-list:data-active:shadow-sm group-data-[variant=line]/tabs-list:data-active:shadow-none [&_svg]:pointer-events-none [&_svg]:shrink-0 [&_svg:not([class*='size-'])]:size-4",
        "group-data-[variant=line]/tabs-list:bg-transparent group-data-[variant=line]/tabs-list:data-active:bg-transparent dark:group-data-[variant=line]/tabs-list:data-active:border-transparent dark:group-data-[variant=line]/tabs-list:data-active:bg-transparent",
        "data-active:bg-background data-active:text-foreground dark:data-active:border-input dark:data-active:bg-input/30 dark:data-active:text-foreground",
        "after:absolute after:bg-foreground after:opacity-0 after:transition-opacity group-data-horizontal/tabs:after:inset-x-0 group-data-horizontal/tabs:after:bottom-[-5px] group-data-horizontal/tabs:after:h-0.5 group-data-vertical/tabs:after:inset-y-0 group-data-vertical/tabs:after:-right-1 group-data-vertical/tabs:after:w-0.5 group-data-[variant=line]/tabs-list:data-active:after:opacity-100",
        className
      )}
      {...props}
    />
  )
}

function TabsContent({
  className,
  ...props
}: React.ComponentProps<typeof TabsPrimitive.Content>) {
  return (
    <TabsPrimitive.Content
      data-slot="tabs-content"
      className={cn("flex-1 text-sm outline-none", className)}
      {...props}
    />
  )
}

export { Tabs, TabsList, TabsTrigger, TabsContent, tabsListVariants }
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/hooks/useUser.ts".into(),
                content: r#"import { useQuery } from '@tanstack/react-query';
import api from '../api/axios';
import { UserSchema } from '../types/schema';

export const useUser = (userId: string) => {
  return useQuery({
    queryKey: ['user', userId],
    queryFn: async () => {
      const { data } = await api.get(`/users/${userId}`);
      return UserSchema.parse(data);
    },
    enabled: !!userId,
  });
};
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/index.css".into(),
                content: r#"@import url('https://fonts.googleapis.com/css2?family=Manrope:wght@400;500;600;700&family=Geist:wght@400;500;600;700&display=swap');
@import "tailwindcss";
@import "tw-animate-css";
@import "shadcn/tailwind.css";
@import "@fontsource-variable/inter";

@custom-variant dark (&:is(.dark *));

@theme {
  --color-background: oklch(var(--background));
  --color-foreground: oklch(var(--foreground));

  --color-primary: oklch(var(--primary));
  --color-primary-foreground: oklch(var(--primary-foreground));

  --color-secondary: oklch(var(--secondary));
  --color-secondary-foreground: oklch(var(--secondary-foreground));

  --color-muted: oklch(var(--muted));
  --color-muted-foreground: oklch(var(--muted-foreground));

  --color-accent: oklch(var(--accent));
  --color-accent-foreground: oklch(var(--accent-foreground));

  --color-destructive: oklch(var(--destructive));
  --color-destructive-foreground: oklch(var(--destructive-foreground));

  --color-border: oklch(var(--border));
  --color-input: oklch(var(--input));
  --color-ring: oklch(var(--ring));

  --radius-lg: 0.5rem;
  --radius-md: calc(0.5rem - 2px);
  --radius-sm: calc(0.5rem - 4px);

  --font-heading: 'Manrope', sans-serif;
  --font-body: 'Geist', sans-serif;
}

@layer base {
  :root {
    --background: 1 0 0;
    --foreground: 0.141 0.005 285.823;
    --primary: 0.59 0.201 273.444;
    --primary-foreground: 1 0 0;
    --secondary: 0.949 0.029 303.081;
    --secondary-foreground: 0.21 0.006 285.885;
    --muted: 0.963 0.023 308.198;
    --muted-foreground: 0.472 0.002 286.339;
    --accent: 0.949 0.029 303.081;
    --accent-foreground: 0.211 0.006 285.885;
    --destructive: 0.637 0.208 25.331;
    --destructive-foreground: 0.985 0 0;
    --border: 0.92 0.02 285; /* Adjusted for better visibility */
    --input: 0.92 0.02 285;
    --ring: 0.59 0.201 273.444;
    --radius: 0.5rem;
  }

  .dark {
    --background: 0.141 0.005 285.823;
    --foreground: 0.985 0 0;
    --primary: 0.665 0.179 278.961;
    --primary-foreground: 1 0 0;
    --secondary: 0.202 0.107 263.462;
    --secondary-foreground: 0.985 0 0;
    --muted: 0.167 0.112 264.144;
    --muted-foreground: 0.673 0 0;
    --accent: 0.202 0.107 263.462;
    --accent-foreground: 0.985 0 0;
    --destructive: 0.396 0.133 25.723;
    --destructive-foreground: 0.985 0 0;
    --border: 0.25 0.05 264; /* Adjusted for dark mode */
    --input: 0.25 0.05 264;
    --ring: 0.665 0.179 278.961;
  }
  * {
    @apply border-border outline-ring/50;
  }
  body {
    @apply bg-background text-foreground;
  }
  button:not(:disabled), [role="button"]:not(:disabled) {
    cursor: pointer;
  }
  html {
    @apply font-sans;
  }
}

@layer base {
  * {
    @apply border-border;
  }
  body {
    @apply bg-background text-foreground font-body;
  }
  h1, h2, h3, h4, h5, h6 {
    @apply font-heading font-bold;
  }
}

@theme inline {
  --font-heading: var(--font-sans);
  --font-sans: 'Inter Variable', sans-serif;
  --color-sidebar-ring: var(--sidebar-ring);
  --color-sidebar-border: var(--sidebar-border);
  --color-sidebar-accent-foreground: var(--sidebar-accent-foreground);
  --color-sidebar-accent: var(--sidebar-accent);
  --color-sidebar-primary-foreground: var(--sidebar-primary-foreground);
  --color-sidebar-primary: var(--sidebar-primary);
  --color-sidebar-foreground: var(--sidebar-foreground);
  --color-sidebar: var(--sidebar);
  --color-chart-5: var(--chart-5);
  --color-chart-4: var(--chart-4);
  --color-chart-3: var(--chart-3);
  --color-chart-2: var(--chart-2);
  --color-chart-1: var(--chart-1);
  --color-ring: var(--ring);
  --color-input: var(--input);
  --color-border: var(--border);
  --color-destructive: var(--destructive);
  --color-accent-foreground: var(--accent-foreground);
  --color-accent: var(--accent);
  --color-muted-foreground: var(--muted-foreground);
  --color-muted: var(--muted);
  --color-secondary-foreground: var(--secondary-foreground);
  --color-secondary: var(--secondary);
  --color-primary-foreground: var(--primary-foreground);
  --color-primary: var(--primary);
  --color-popover-foreground: var(--popover-foreground);
  --color-popover: var(--popover);
  --color-card-foreground: var(--card-foreground);
  --color-card: var(--card);
  --color-foreground: var(--foreground);
  --color-background: var(--background);
  --radius-sm: calc(var(--radius) * 0.6);
  --radius-md: calc(var(--radius) * 0.8);
  --radius-lg: var(--radius);
  --radius-xl: calc(var(--radius) * 1.4);
  --radius-2xl: calc(var(--radius) * 1.8);
  --radius-3xl: calc(var(--radius) * 2.2);
  --radius-4xl: calc(var(--radius) * 2.6);
}

:root {
  --background: oklch(1 0 0);
  --foreground: oklch(0.145 0 0);
  --card: oklch(1 0 0);
  --card-foreground: oklch(0.145 0 0);
  --popover: oklch(1 0 0);
  --popover-foreground: oklch(0.145 0 0);
  --primary: oklch(0.205 0 0);
  --primary-foreground: oklch(0.985 0 0);
  --secondary: oklch(0.97 0 0);
  --secondary-foreground: oklch(0.205 0 0);
  --muted: oklch(0.97 0 0);
  --muted-foreground: oklch(0.556 0 0);
  --accent: oklch(0.97 0 0);
  --accent-foreground: oklch(0.205 0 0);
  --destructive: oklch(0.577 0.245 27.325);
  --border: oklch(0.922 0 0);
  --input: oklch(0.922 0 0);
  --ring: oklch(0.708 0 0);
  --chart-1: oklch(0.87 0 0);
  --chart-2: oklch(0.556 0 0);
  --chart-3: oklch(0.439 0 0);
  --chart-4: oklch(0.371 0 0);
  --chart-5: oklch(0.269 0 0);
  --radius: 0.625rem;
  --sidebar: oklch(0.985 0 0);
  --sidebar-foreground: oklch(0.145 0 0);
  --sidebar-primary: oklch(0.205 0 0);
  --sidebar-primary-foreground: oklch(0.985 0 0);
  --sidebar-accent: oklch(0.97 0 0);
  --sidebar-accent-foreground: oklch(0.205 0 0);
  --sidebar-border: oklch(0.922 0 0);
  --sidebar-ring: oklch(0.708 0 0);
}

.dark {
  --background: oklch(0.145 0 0);
  --foreground: oklch(0.985 0 0);
  --card: oklch(0.205 0 0);
  --card-foreground: oklch(0.985 0 0);
  --popover: oklch(0.205 0 0);
  --popover-foreground: oklch(0.985 0 0);
  --primary: oklch(0.922 0 0);
  --primary-foreground: oklch(0.205 0 0);
  --secondary: oklch(0.269 0 0);
  --secondary-foreground: oklch(0.985 0 0);
  --muted: oklch(0.269 0 0);
  --muted-foreground: oklch(0.708 0 0);
  --accent: oklch(0.269 0 0);
  --accent-foreground: oklch(0.985 0 0);
  --destructive: oklch(0.704 0.191 22.216);
  --border: oklch(1 0 0 / 10%);
  --input: oklch(1 0 0 / 15%);
  --ring: oklch(0.556 0 0);
  --chart-1: oklch(0.87 0 0);
  --chart-2: oklch(0.556 0 0);
  --chart-3: oklch(0.439 0 0);
  --chart-4: oklch(0.371 0 0);
  --chart-5: oklch(0.269 0 0);
  --sidebar: oklch(0.205 0 0);
  --sidebar-foreground: oklch(0.985 0 0);
  --sidebar-primary: oklch(0.488 0.243 264.376);
  --sidebar-primary-foreground: oklch(0.985 0 0);
  --sidebar-accent: oklch(0.269 0 0);
  --sidebar-accent-foreground: oklch(0.985 0 0);
  --sidebar-border: oklch(1 0 0 / 10%);
  --sidebar-ring: oklch(0.556 0 0);
}"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/layouts/RootLayout.tsx".into(),
                content: r#"import { useEffect } from 'react';
import { Outlet } from 'react-router-dom';
import { Navbar } from '@/components/Navbar';
import { Footer } from '@/components/Footer';
import { ThemeProvider } from '@/components/ThemeProvider';
import Lenis from 'lenis';
import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

// Register ScrollTrigger
gsap.registerPlugin(ScrollTrigger);

export function RootLayout() {
  useEffect(() => {
    // Initialize Lenis
    const lenis = new Lenis({
      autoRaf: true, // Let Lenis handle its own RAF, or we can sync it with GSAP
    });

    // Synchronize ScrollTrigger with Lenis
    lenis.on('scroll', ScrollTrigger.update);

    return () => {
      lenis.destroy();
    };
  }, []);

  return (
    <ThemeProvider>
      <div className="min-h-screen flex flex-col bg-background text-foreground transition-colors duration-300">
        <Navbar />
        <main className="pt-16 flex-grow">
          <Outlet />
        </main>
        <Footer />
      </div>
    </ThemeProvider>
  );
}

"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/lib/logger.ts".into(),
                content: r##"type LogLevel = "info" | "warn" | "error" | "debug"

const isDev = import.meta.env.DEV

const styles: Record<LogLevel, string> = {
  info: "color: #3b82f6; font-weight: 600;",
  warn: "color: #f59e0b; font-weight: 600;",
  error: "color: #ef4444; font-weight: 600;",
  debug: "color: #10b981; font-weight: 600;",
}

const formatMessage = (level: LogLevel, message: string) => {
  const prefix = `[WEB] ${level.toUpperCase()}`
  return [`%c${prefix} %c${message}`, styles[level], "color: inherit; font-weight: normal;"]
}

function log(level: LogLevel, message: string, data?: unknown) {
  if (!isDev && level === "debug") return

  const [prompt, style, reset] = formatMessage(level, message)

  if (data === undefined) {
    if (level === "error") console.error(prompt, style, reset)
    else if (level === "warn") console.warn(prompt, style, reset)
    else console.log(prompt, style, reset)
    return
  }

  // Handle data with grouping for a cleaner console
  console.groupCollapsed(prompt, style, reset)
  
  if (data instanceof Error) {
    console.error(data.message)
    if (data.stack) console.debug(data.stack)
  } else {
    console.dir(data)
  }
  
  console.groupEnd()
}

export const logger = {
  info: (msg: string, data?: unknown) => log("info", msg, data),
  warn: (msg: string, data?: unknown) => log("warn", msg, data),
  error: (msg: string, data?: unknown) => log("error", msg, data),
  debug: (msg: string, data?: unknown) => log("debug", msg, data),
}"##.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/lib/utils.ts".into(),
                content: r#"import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/main.tsx".into(),
                content: r#"import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './App.tsx'
import { QueryProvider } from '@/providers/QueryProvider.tsx'

createRoot(document.getElementById('root')!).render(
  <StrictMode>
    <QueryProvider>
      <App />
    </QueryProvider>
  </StrictMode>,
)
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/pages/AboutPage.tsx".into(),
                content: r#"import { motion } from 'framer-motion';
import { LordIcon } from '@/components/LordIcon';
import { Button } from '@/components/ui/button';
import { Link } from 'react-router-dom';
import { ChevronLeft, ArrowRight, Shield, Award, Users } from 'lucide-react';

export function AboutPage() {
  return (
    <div className="relative min-h-[calc(100vh-4rem)] bg-background text-foreground py-16 px-6 overflow-hidden">
      {/* Background decoration */}
      <div className="absolute top-1/4 right-0 w-80 h-80 bg-purple-500/5 rounded-full blur-[100px] pointer-events-none" />
      <div className="absolute bottom-1/4 left-0 w-80 h-80 bg-primary/5 rounded-full blur-[100px] pointer-events-none" />

      <div className="max-w-4xl mx-auto relative z-10">
        {/* Back Link */}
        <Link to="/" className="inline-flex items-center gap-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors mb-12 group">
          <ChevronLeft className="w-4 h-4 group-hover:-translate-x-1 transition-transform" />
          <span>Back to Home</span>
        </Link>

        {/* Title */}
        <div className="flex items-center gap-4 mb-8">
          <div className="p-3 bg-primary/10 rounded-2xl text-primary border border-primary/20">
            <LordIcon src="https://cdn.lordicon.com/xzalkbkz.json" size={36} colors="primary:currentColor" />
          </div>
          <div>
            <h1 className="text-4xl md:text-5xl font-heading font-extrabold tracking-tight">About The Kinetic Stack</h1>
            <p className="text-muted-foreground font-body text-base mt-1">Unleashing high-performance motion design on the web.</p>
          </div>
        </div>

        {/* Body content */}
        <div className="space-y-12 font-body">
          {/* Mission */}
          <section className="bg-card/40 border border-border/40 rounded-3xl p-8 backdrop-blur-sm">
            <h2 className="text-2xl font-heading font-bold mb-4 flex items-center gap-2">
              <Award className="w-5 h-5 text-primary" />
              <span>Our Creative Vision</span>
            </h2>
            <p className="text-muted-foreground leading-relaxed">
              We believe websites should feel alive, responsive, and tactile. By combining the industry standards of animation, we create high-fidelity user experiences that don't just look beautiful but feel incredibly tactile and premium.
            </p>
          </section>

          {/* Cards Grid */}
          <section className="grid md:grid-cols-3 gap-6">
            <motion.div 
              whileHover={{ y: -4 }}
              className="p-6 rounded-2xl border border-border/40 bg-card/30 flex flex-col gap-3"
            >
              <div className="w-10 h-10 rounded-xl bg-purple-500/10 text-purple-500 border border-purple-500/20 flex items-center justify-center">
                <Shield className="w-5 h-5" />
              </div>
              <h3 className="font-heading font-bold text-lg">Optimized Code</h3>
              <p className="text-muted-foreground text-sm leading-relaxed">Dynamic script loading and tree-shaken animation layers keep things extremely fast.</p>
            </motion.div>

            <motion.div 
              whileHover={{ y: -4 }}
              className="p-6 rounded-2xl border border-border/40 bg-card/30 flex flex-col gap-3"
            >
              <div className="w-10 h-10 rounded-xl bg-cyan-500/10 text-cyan-500 border border-cyan-500/20 flex items-center justify-center">
                <Users className="w-5 h-5" />
              </div>
              <h3 className="font-heading font-bold text-lg">Accessible First</h3>
              <p className="text-muted-foreground text-sm leading-relaxed">Radix UI primitives ensure that our design templates are fully keyboard accessible and screen-reader compliant.</p>
            </motion.div>

            <motion.div 
              whileHover={{ y: -4 }}
              className="p-6 rounded-2xl border border-border/40 bg-card/30 flex flex-col gap-3"
            >
              <div className="w-10 h-10 rounded-xl bg-green-500/10 text-green-500 border border-green-500/20 flex items-center justify-center">
                <LordIcon src="https://cdn.lordicon.com/nocovwne.json" size={20} colors="primary:currentColor" />
              </div>
              <h3 className="font-heading font-bold text-lg">Interactive Assets</h3>
              <p className="text-muted-foreground text-sm leading-relaxed">Lordicon vector files scale to any size, support interactive triggers, and respond to theme changes.</p>
            </motion.div>
          </section>

          {/* Call to action */}
          <div className="flex flex-col items-center justify-center text-center p-8 border border-border/40 rounded-3xl bg-gradient-to-r from-primary/10 via-purple-500/5 to-transparent">
            <h3 className="font-heading font-bold text-xl mb-2">Ready to test the motions?</h3>
            <p className="text-muted-foreground text-sm max-w-md mb-6">Head back to our sandbox interface to test dragging, scrolling, and sequences.</p>
            <Link to="/">
              <Button className="rounded-xl px-5 gap-1.5 font-semibold">
                <span>Go to Sandbox</span>
                <ArrowRight className="w-4 h-4" />
              </Button>
            </Link>
          </div>
        </div>
      </div>
    </div>
  );
}
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/pages/ContactPage.tsx".into(),
                content: r#"import React, { useState } from 'react';
import { motion } from 'framer-motion';
import { LordIcon } from '@/components/LordIcon';
import { Button } from '@/components/ui/button';
import { Link } from 'react-router-dom';
import { ChevronLeft, Send, Sparkles } from 'lucide-react';

export function ContactPage() {
  const [formState, setFormState] = useState({ name: '', email: '', message: '' });
  const [submitted, setSubmitted] = useState(false);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (!formState.name || !formState.email || !formState.message) return;
    setSubmitted(true);
  };

  return (
    <div className="relative min-h-[calc(100vh-4rem)] bg-background text-foreground py-16 px-6 overflow-hidden">
      {/* Background decoration */}
      <div className="absolute top-1/4 left-0 w-80 h-80 bg-primary/5 rounded-full blur-[100px] pointer-events-none" />
      <div className="absolute bottom-1/4 right-0 w-80 h-80 bg-purple-500/5 rounded-full blur-[100px] pointer-events-none" />

      <div className="max-w-xl mx-auto relative z-10">
        {/* Back Link */}
        <Link to="/" className="inline-flex items-center gap-1.5 text-sm text-muted-foreground hover:text-foreground transition-colors mb-12 group">
          <ChevronLeft className="w-4 h-4 group-hover:-translate-x-1 transition-transform" />
          <span>Back to Home</span>
        </Link>

        {/* Title */}
        <div className="flex items-center gap-4 mb-8">
          <div className="p-3 bg-primary/10 rounded-2xl text-primary border border-primary/20">
            <LordIcon src="https://cdn.lordicon.com/pithnlch.json" size={36} colors="primary:currentColor" trigger="loop" delay={1500} />
          </div>
          <div>
            <h1 className="text-4xl font-heading font-extrabold tracking-tight">Get in Touch</h1>
            <p className="text-muted-foreground font-body text-base mt-1">Let's talk about building creative web animations.</p>
          </div>
        </div>

        {/* Contact Form or Success Screen */}
        <div className="bg-card/40 border border-border/40 rounded-3xl p-8 backdrop-blur-sm shadow-xl font-body">
          {!submitted ? (
            <form onSubmit={handleSubmit} className="space-y-6">
              <div className="space-y-2">
                <label className="text-sm font-semibold text-foreground/80">Full Name</label>
                <input
                  type="text"
                  required
                  placeholder="John Doe"
                  value={formState.name}
                  onChange={(e) => setFormState({ ...formState, name: e.target.value })}
                  className="w-full h-11 px-4 rounded-xl border border-border bg-background/50 focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all placeholder:text-muted-foreground/60 text-sm"
                />
              </div>

              <div className="space-y-2">
                <label className="text-sm font-semibold text-foreground/80">Email Address</label>
                <input
                  type="email"
                  required
                  placeholder="john@example.com"
                  value={formState.email}
                  onChange={(e) => setFormState({ ...formState, email: e.target.value })}
                  className="w-full h-11 px-4 rounded-xl border border-border bg-background/50 focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all placeholder:text-muted-foreground/60 text-sm"
                />
              </div>

              <div className="space-y-2">
                <label className="text-sm font-semibold text-foreground/80">Your Message</label>
                <textarea
                  required
                  rows={4}
                  placeholder="Tell us about your project or questions..."
                  value={formState.message}
                  onChange={(e) => setFormState({ ...formState, message: e.target.value })}
                  className="w-full p-4 rounded-xl border border-border bg-background/50 focus:border-primary focus:ring-1 focus:ring-primary outline-none transition-all placeholder:text-muted-foreground/60 text-sm resize-none"
                />
              </div>

              <Button
                type="submit"
                className="w-full h-12 rounded-xl font-semibold bg-gradient-to-r from-primary to-purple-600 text-primary-foreground hover:shadow-lg hover:shadow-primary/20 transition-all gap-2"
              >
                <Send className="w-4 h-4" />
                <span>Send Message</span>
              </Button>
            </form>
          ) : (
            <motion.div
              initial={{ scale: 0.95, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              className="text-center py-8 space-y-6"
            >
              <div className="inline-flex p-4 bg-green-500/10 rounded-full border border-green-500/20 text-green-500">
                <LordIcon src="https://cdn.lordicon.com/nocovwne.json" size={56} trigger="loop" colors="primary:currentColor" />
              </div>
              <div className="space-y-2">
                <h3 className="text-2xl font-heading font-extrabold flex items-center justify-center gap-1.5 text-foreground">
                  <Sparkles className="w-5 h-5 text-yellow-500" />
                  <span>Thank You, {formState.name}!</span>
                </h3>
                <p className="text-muted-foreground max-w-sm mx-auto text-sm leading-relaxed">
                  Your message has been received! We'll reach out to <span className="text-foreground font-semibold">{formState.email}</span> as soon as possible.
                </p>
              </div>
              <Button
                onClick={() => {
                  setSubmitted(false);
                  setFormState({ name: '', email: '', message: '' });
                }}
                variant="outline"
                className="rounded-xl px-5"
              >
                Send Another Message
              </Button>
            </motion.div>
          )}
        </div>
      </div>
    </div>
  );
}
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/pages/HomePage.tsx".into(),
                content: r##"import { useState, useEffect, useRef } from 'react';
import { motion } from 'framer-motion';
import reactLogo from '../assets/react.svg';
import viteLogo from '../assets/vite.svg';
import { Button } from '@/components/ui/button';
import { LordIcon } from '@/components/LordIcon';
import { LottieAnimation } from '@/components/LottieAnimation';
import { gsap } from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

// Register ScrollTrigger
gsap.registerPlugin(ScrollTrigger);

export function HomePage() {
  const [count, setCount] = useState(0);
  const [reducedMotion, setReducedMotion] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  // Check accessibility reduced-motion preference
  useEffect(() => {
    const mediaQuery = window.matchMedia('(prefers-reduced-motion: reduce)');
    setReducedMotion(mediaQuery.matches);
    
    const handleChange = (e: MediaQueryListEvent) => {
      setReducedMotion(e.matches);
    };
    
    mediaQuery.addEventListener('change', handleChange);
    return () => mediaQuery.removeEventListener('change', handleChange);
  }, []);

  // GSAP Animations
  useEffect(() => {
    if (reducedMotion) return;

    const ctx = gsap.context(() => {
      // 1. Hero Staggered Entrance Animation
      const tl = gsap.timeline({ defaults: { ease: 'power3.out' } });

      tl.fromTo('.hero-graphic', 
        { opacity: 0, scale: 0.85, y: 20 }, 
        { opacity: 1, scale: 1, y: 0, duration: 1.2 }
      );
      
      tl.fromTo('.hero-title', 
        { opacity: 0, y: 35 }, 
        { opacity: 1, y: 0, duration: 1.0 },
        '-=0.8'
      );

      tl.fromTo('.hero-desc', 
        { opacity: 0, y: 20 }, 
        { opacity: 1, y: 0, duration: 0.8 },
        '-=0.6'
      );

      tl.fromTo('.hero-cta', 
        { opacity: 0, y: 20 }, 
        { opacity: 1, y: 0, duration: 0.8 },
        '-=0.5'
      );

      // 2. ScrollTrigger Scroll Reveal for Cards Grid
      gsap.fromTo('.feature-card-wrapper',
        { opacity: 0, y: 50 },
        {
          opacity: 1,
          y: 0,
          duration: 0.8,
          stagger: 0.18,
          ease: 'power2.out',
          scrollTrigger: {
            trigger: '.features-section',
            start: 'top 85%',
            toggleActions: 'play none none none',
          }
        }
      );
    }, containerRef);

    return () => ctx.revert();
  }, [reducedMotion]);

  return (
    <div ref={containerRef} className="relative min-h-[calc(100vh-4rem)] flex flex-col justify-between overflow-hidden bg-background text-foreground">
      {/* Decorative ambient glowing background */}
      <div className="absolute top-1/4 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[550px] h-[550px] bg-primary/10 rounded-full blur-[140px] pointer-events-none" />

      {/* Main Hero Container */}
      <section className="flex-1 flex flex-col items-center justify-center p-8 max-w-4xl mx-auto w-full text-center relative z-10 pt-16">
        
        {/* Floating Logo / Lottie Hero Graphic */}
        <div className="hero-graphic relative mb-10 w-72 h-72 flex items-center justify-center bg-card/10 border border-border/20 rounded-full p-4 backdrop-blur-sm">
          {/* Lottie Web Design / Coding Illustration */}
          <LottieAnimation 
            src="https://assets.lottiefiles.com/packages/lf20_kkflmtur.json"
            className="w-full h-full relative z-10"
            loop={!reducedMotion}
          />
          
          {/* Floating framework logos with smooth animations */}
          <motion.img 
            src={reactLogo} 
            alt="React logo"
            animate={reducedMotion ? {} : { rotate: 360 }}
            transition={reducedMotion ? {} : { repeat: Infinity, duration: 18, ease: 'linear' }}
            className="framework absolute -top-2 right-4 w-12 h-12 drop-shadow-[0_0_15px_rgba(59,130,246,0.4)]"
          />
          <motion.img 
            src={viteLogo} 
            alt="Vite logo"
            animate={reducedMotion ? {} : { y: [0, -6, 0] }}
            transition={reducedMotion ? {} : { repeat: Infinity, duration: 4, ease: 'easeInOut' }}
            className="vite absolute -bottom-2 left-4 w-12 h-12 drop-shadow-[0_0_15px_rgba(245,158,11,0.4)]"
          />
        </div>

        {/* Title */}
        <h1 className="hero-title text-5xl md:text-6xl font-heading font-extrabold mb-6 tracking-tighter leading-tight">
          Get started{' '}
          <span className="bg-gradient-to-r from-primary via-purple-500 to-indigo-500 bg-clip-text text-transparent">
            Offpkg
          </span>{' '}
          <br /> Vite+React
        </h1>

        {/* Subtitle Description */}
        <p className="hero-desc text-muted-foreground text-lg md:text-xl leading-relaxed max-w-2xl mb-8 font-body">
          The ultimate developer setup with{' '}
          <span className="text-primary font-semibold">Tailwind v4, Zustand, Zod</span>, and{' '}
          <span className="text-primary font-semibold">React Query</span>. Enjoy butter-smooth inertia scrolling and interactive vector animations.
        </p>

        {/* Count trigger button */}
        <div className="hero-cta flex flex-col items-center gap-4">
          <Button
            size="lg"
            onClick={() => setCount((c) => c + 1)}
            className="h-14 px-8 rounded-2xl font-bold text-lg bg-primary text-primary-foreground shadow-lg shadow-primary/20 hover:shadow-primary/30 transition-all hover:scale-[1.04] active:scale-[0.98] gap-2.5"
          >
            <LordIcon 
              src="https://cdn.lordicon.com/lupuorrc.json" // Zap Star
              size={24} 
              trigger={reducedMotion ? 'click' : 'hover'}
              colors="primary:#ffffff,secondary:#ffffff"
            />
            <span>Count is {count}</span>
          </Button>
          <p className="text-sm text-muted-foreground font-mono bg-muted/40 border border-border/20 px-3 py-1 rounded-lg">
            Edit <code className="text-foreground font-semibold">src/pages/HomePage.tsx</code> to test HMR
          </p>
        </div>
      </section>

      {/* Decorative separator line */}
      <div className="h-px bg-gradient-to-r from-transparent via-border/40 to-transparent w-full" />

      {/* Features Grid Section */}
      <section className="features-section p-16 max-w-6xl mx-auto w-full relative z-10">
        <div className="grid md:grid-cols-3 gap-8">
          <div className="feature-card-wrapper">
            <FeatureCard
              title="Fast Refresh"
              desc="Lightning fast HMR provided by Vite 8 for an ultra-smooth dev experience."
              iconSrc="https://cdn.lordicon.com/lupuorrc.json" // Lightning Zap
              accentColor="text-yellow-500"
              reducedMotion={reducedMotion}
            />
          </div>
          <div className="feature-card-wrapper">
            <FeatureCard
              title="Type Safe"
              desc="Zod and TypeScript integration ensures your data is always valid."
              iconSrc="https://cdn.lordicon.com/sbnjyzil.json" // Shield
              accentColor="text-blue-500"
              reducedMotion={reducedMotion}
            />
          </div>
          <div className="feature-card-wrapper">
            <FeatureCard
              title="State Master"
              desc="Global state management simplified with Zustand stores."
              iconSrc="https://cdn.lordicon.com/nocovwne.json" // Box / Package
              accentColor="text-purple-500"
              reducedMotion={reducedMotion}
            />
          </div>
        </div>
      </section>
    </div>
  );
}

// Reusable Feature Card conforming to the "Feature Card Pattern"
interface FeatureCardProps {
  title: string;
  desc: string;
  iconSrc: string;
  accentColor: string;
  reducedMotion: boolean;
}

function FeatureCard({ title, desc, iconSrc, accentColor, reducedMotion }: FeatureCardProps) {
  return (
    <motion.div
      whileHover={{ y: reducedMotion ? 0 : -8 }}
      className="p-8 h-full rounded-3xl border border-border/40 bg-card/50 backdrop-blur-md text-card-foreground hover:border-primary/40 hover:shadow-xl hover:shadow-primary/5 transition-all duration-300 group cursor-default"
    >
      <div className={`mb-6 w-12 h-12 rounded-2xl bg-muted/20 border border-border/30 flex items-center justify-center transition-transform duration-300 group-hover:scale-110 ${accentColor}`}>
        <LordIcon 
          src={iconSrc} 
          size={32} 
          trigger={reducedMotion ? 'click' : 'hover'}
          colors="primary:currentColor,secondary:currentColor" 
        />
      </div>
      <h3 className="text-2xl font-heading font-extrabold mb-3 group-hover:text-primary transition-colors duration-200">
        {title}
      </h3>
      <p className="text-muted-foreground font-body leading-relaxed text-sm">
        {desc}
      </p>
    </motion.div>
  );
}
"##.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/pages/NotFoundPage.tsx".into(),
                content: r#"import { Link } from 'react-router-dom';
import { MoveLeft } from 'lucide-react';

export function NotFoundPage() {
  return (
    <div className="min-h-[calc(100vh-4rem)] flex flex-col items-center justify-center p-8 text-center animate-in fade-in duration-700">
      <div className="relative mb-8">
        <h1 className="text-[12rem] font-black leading-none tracking-tighter text-muted-foreground/10 select-none">
          404
        </h1>
        <div className="absolute inset-0 flex items-center justify-center">
          <p className="text-4xl font-heading font-black tracking-tight">PAGE NOT FOUND</p>
        </div>
      </div>
      
      <p className="text-xl text-muted-foreground mb-12 max-w-md mx-auto leading-relaxed">
        The page you are looking for doesn't exist or has been moved to another universe.
      </p>

      <Link
        to="/"
        className="flex items-center gap-2 bg-primary text-primary-foreground px-8 py-4 rounded-2xl font-bold transition-all hover:gap-4 hover:pr-10 hover:shadow-xl active:scale-95"
      >
        <MoveLeft className="w-5 h-5" /> Back to Home
      </Link>
    </div>
  );
}
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/providers/QueryProvider.tsx".into(),
                content: r#"import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import React from 'react';

const queryClient = new QueryClient({
  defaultOptions: {
    queries: {
      staleTime: 1000 * 60 * 5, // 5 minutes
      retry: 1,
      refetchOnWindowFocus: false,
    },
  },
});

export function QueryProvider({ children }: { children: React.ReactNode }) {
  return (
    <QueryClientProvider client={queryClient}>
      {children}
    </QueryClientProvider>
  );
}
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/store/useAppStore.ts".into(),
                content: r#"import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface AppState {
  user: { name: string; email: string } | null;
  setUser: (user: { name: string; email: string } | null) => void;
  isLoading: boolean;
  setIsLoading: (loading: boolean) => void;
}

export const useAppStore = create<AppState>()(
  persist(
    (set) => ({
      user: null,
      setUser: (user) => set({ user }),
      isLoading: false,
      setIsLoading: (isLoading) => set({ isLoading }),
    }),
    {
      name: 'app-storage',
    }
  )
);
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/store/useThemeStore.ts".into(),
                content: r#"import { create } from 'zustand';
import { persist } from 'zustand/middleware';

export type Theme = 'light' | 'dark' | 'system';

interface ThemeState {
  theme: Theme;
  setTheme: (theme: Theme) => void;
}

export const useThemeStore = create<ThemeState>()(
  persist(
    (set) => ({
      theme: 'system',
      setTheme: (theme) => set({ theme }),
    }),
    {
      name: 'theme-storage',
    }
  )
);
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "src/types/schema.ts".into(),
                content: r#"import { z } from 'zod';

export const UserSchema = z.object({
  id: z.string(),
  name: z.string().min(2, 'Name must be at least 2 characters'),
  email: z.string().email('Invalid email address'),
  role: z.enum(['admin', 'user', 'guest']),
});

export type User = z.infer<typeof UserSchema>;

export const LoginFormSchema = z.object({
  email: z.string().email(),
  password: z.string().min(6, 'Password must be at least 6 characters'),
});

export type LoginFormValues = z.infer<typeof LoginFormSchema>;
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "tsconfig.app.json".into(),
                content: r#"{
  "compilerOptions": {
    "tsBuildInfoFile": "./node_modules/.tmp/tsconfig.app.tsbuildinfo",
    "target": "ES2023",
    "useDefineForClassFields": true,
    "lib": ["ES2023", "DOM", "DOM.Iterable"],
    "module": "ESNext",
    "types": ["vite/client"],
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "verbatimModuleSyntax": true,
    "moduleDetection": "force",
    "noEmit": true,
    "jsx": "react-jsx",

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "erasableSyntaxOnly": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedSideEffectImports": true,
    "baseUrl": ".",
    "ignoreDeprecations": "6.0",
    "paths": {
      "@/*": ["./src/*"]
    }
  },
  "include": ["src"]
}
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "tsconfig.json".into(),
                content: r#"{
  "files": [],
  "references": [
    { "path": "./tsconfig.app.json" },
    { "path": "./tsconfig.node.json" }
  ]
}
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "tsconfig.node.json".into(),
                content: r#"{
  "compilerOptions": {
    "tsBuildInfoFile": "./node_modules/.tmp/tsconfig.node.tsbuildinfo",
    "target": "ES2023",
    "lib": ["ES2023"],
    "module": "ESNext",
    "types": ["node"],
    "skipLibCheck": true,

    /* Bundler mode */
    "moduleResolution": "bundler",
    "allowImportingTsExtensions": true,
    "verbatimModuleSyntax": true,
    "moduleDetection": "force",
    "noEmit": true,

    /* Linting */
    "strict": true,
    "noUnusedLocals": true,
    "noUnusedParameters": true,
    "erasableSyntaxOnly": true,
    "noFallthroughCasesInSwitch": true,
    "noUncheckedSideEffectImports": true
  },
  "include": ["vite.config.ts"]
}
"#.into(),
                binary_content: None,
            },
            StackFile {
                path: "vite.config.ts".into(),
                content: r#"import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'
import path from 'path'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    react(),
    tailwindcss(),
  ],
  server: {
    hmr: {
      overlay: true,
    },
  },
  resolve: {
    alias: {
      '@': path.resolve(__dirname, './src'),
    },
  },
})
"#.into(),
                binary_content: None,
            }
        ],
    }
}
