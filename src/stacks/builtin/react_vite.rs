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
                content: r###"# <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/docs/logo.svg" alt="offpkg Logo" width="36" height="36" align="center"/> Offpkg Vite+React Template 🚀

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
          <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/docs/logo.svg" alt="Offpkg Logo" className="w-6 h-6" />
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
                content: r###"# <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/docs/logo.svg" alt="offpkg Logo" width="36" height="36" align="center"/> Offpkg Vite+React Template 🚀

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
          <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/docs/logo.svg" alt="Offpkg Logo" className="w-6 h-6" />
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
                content: r####"# <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/docs/logo.svg" alt="offpkg Logo" width="36" height="36" align="center"/> Offpkg Vite+React Kinetic Template 🚀

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
          <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/docs/logo.svg" alt="Offpkg Logo" className="w-7 h-7" />
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
