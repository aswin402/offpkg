use crate::stacks::{Stack, StackFile};

pub fn next_template() -> Stack {
    Stack {
        name: "next-template".into(),
        runtime: "bun".into(),
        description: "Upgraded Next.js 16 + Bun + Tailwind CSS v4 + Prisma 7 + Professional Backend".into(),
        packages: vec![
            "@hookform/resolvers".into(),
            "@tanstack/react-query".into(),
            "axios".into(),
            "react-hook-form".into(),
            "zod".into(),
            "zustand".into(),
            "@prisma/client".into(),
            "bcryptjs".into(),
            "jsonwebtoken".into(),
            "superjson".into(),
            "next-themes".into(),
            "pino".into(),
            "ioredis".into(),
            "nodemailer".into(),
            "node-cron".into(),
            "@aws-sdk/client-s3".into(),
            "@aws-sdk/s3-request-presigner".into(),
            "cors".into(),
            "class-variance-authority".into(),
            "clsx".into(),
            "lucide-react".into(),
            "next".into(),
            "radix-ui".into(),
            "react".into(),
            "react-dom".into(),
            "shadcn".into(),
            "tailwind-merge".into(),
            "tw-animate-css".into(),
        ],
        dev_packages: vec![
            "prisma".into(),
            "@types/bcryptjs".into(),
            "@types/jsonwebtoken".into(),
            "typescript".into(),
            "@types/node".into(),
            "@types/react".into(),
            "@types/react-dom".into(),
            "eslint".into(),
            "eslint-config-next".into(),
            "tailwindcss".into(),
            "@tailwindcss/postcss".into(),
            "@types/pg".into(),
            "pino-pretty".into(),
            "@types/nodemailer".into(),
            "@types/node-cron".into(),
            "@types/cors".into(),
        ],
        transitive_packages: vec![],
        files: vec![
            StackFile {
                path: "next.config.ts".into(),
                content: r###"import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  logging: {
    incomingRequests: false,
  },
};

export default nextConfig;
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "next-env.d.ts".into(),
                content: r###"/// <reference types="next" />
/// <reference types="next/image-types/global" />
import "./.next/types/routes.d.ts";

// NOTE: This file should not be edited
// see https://nextjs.org/docs/app/api-reference/config/typescript for more information.
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "README.md".into(),
                content: r###"# <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/doc/logo.svg" alt="offpkg Logo" width="36" height="36" align="center"/> Offpkg Next.js Full-Stack Starter Template 🚀

Welcome to your upgraded, high-performance project template built with the latest modern web technologies. This project is optimized for speed, security, and developer experience.

## ✨ Features

- **Framework**: [Next.js 16 (App Router)](https://nextjs.org/) utilizing React Canary features (e.g., Server Components, Async Params, Server Functions).
- **Runtime & Bundler**: [Bun](https://bun.sh/) for ultra-fast package install, script runs, and hot reload.
- **Styling**: [Tailwind CSS v4](https://tailwindcss.com/) with native OKLCH colors and cascading layers.
- **ORM & Database**: [Prisma v7](https://www.prisma.io/) with native JavaScript driver adapters (`@prisma/adapter-pg` and `pg` pool) for a 90% smaller engine bundle.
- **UI System**: Pre-configured [Shadcn UI](https://ui.shadcn.com/) components.
- **State Management**: Hydration-safe [Zustand](https://docs.pmnd.rs/zustand) stores.
- **Data Fetching**: [TanStack Query v5 (React Query)](https://tanstack.com/query) client provider and cached queries.
- **Validation**: [Zod](https://zod.dev/) type-safe schemas.
- **Authentication**: Secure token-based session handling with `bcryptjs` password hashing, JSON Web Tokens (JWT), and HTTP-only cookies.
- **Structured Logging**: Dual-mode logger (colored server CLI logs + clean group-collapsed browser console entries).

---

## 📂 Documentation

Detailed manuals are available in the `docs/` directory:

1. [Database Setup & Prisma 7 Guide](docs/PRISMA.md) - Deep dive into database config, driver adapters, and schema structure.
2. [Architecture & Auth Layout](docs/ARCHITECTURE.md) - Explains folder hierarchy, global state, React Query hooks, and security flows.
3. [Structured Logging with Pino](docs/LOGGING.md) - High-performance structured logging.
4. [Redis & API Rate Limiting](docs/REDIS.md) - Setup for Redis caching and sliding-window rate limiters.
5. [Transactional SMTP Mailer](docs/MAILER.md) - Dispatching HTML emails using Nodemailer.
6. [Object File Storage (S3 & R2)](docs/STORAGE.md) - Object uploads and client presigned URLs.
7. [Scheduled Background Tasks (Cron)](docs/CRON.md) - Background cron registers utilizing Next.js instrumentation.

---

## 🛠️ Getting Started

### 1. Requirements
Ensure you have [Bun](https://bun.sh/) installed:
```bash
curl -fsSL https://bun.sh/install | bash
```

### 2. Installation
Install project dependencies:
```bash
bun install
```

### 3. Database & Environment Setup
Open `.env` in the root directory to confirm the default PostgreSQL database credentials match your docker setup:
```env
DATABASE_URL="postgresql://postgres:postgres@localhost:5432/offpkg_db?schema=public"
```

Spin up the local PostgreSQL database using Docker Compose:
```bash
docker-compose up -d
```

### 4. Running Database Migrations
Initialize database tables using Prisma CLI scripts:
```bash
bun run db:migrate
```

Re-generate client bindings and seed mock users/posts:
```bash
bun run db:generate
bun run db:seed
```

### 5. Running the Application
Spin up the hot-reload dev server:
```bash
bun run dev
```

Your app will be live at [http://localhost:3000](http://localhost:3000).

---

## 📦 Script Directory

All primary commands are run via Bun:

| Command | Action |
| :--- | :--- |
| `bun run dev` | Starts the Next.js development server |
| `bun run build` | Builds the production bundle |
| `bun run start` | Runs the built production bundle |
| `bun run lint` | Runs ESLint check |
| `bun run db:migrate` | Runs database migrations |
| `bun run db:generate` | Re-generates Prisma type-safe client |
| `bun run db:seed` | Resets database and seeds mock data |
| `bun run db:studio` | Opens interactive database panel in browser |

---

## 📜 License
MIT
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "tsconfig.json".into(),
                content: r###"{
  "compilerOptions": {
    "target": "ES2017",
    "lib": ["dom", "dom.iterable", "esnext"],
    "allowJs": true,
    "skipLibCheck": true,
    "strict": true,
    "noEmit": true,
    "esModuleInterop": true,
    "module": "esnext",
    "moduleResolution": "bundler",
    "resolveJsonModule": true,
    "isolatedModules": true,
    "jsx": "react-jsx",
    "incremental": true,
    "plugins": [
      {
        "name": "next"
      }
    ],
    "paths": {
      "@/*": ["./*"]
    }
  },
  "include": [
    "next-env.d.ts",
    "**/*.ts",
    "**/*.tsx",
    ".next/types/**/*.ts",
    ".next/dev/types/**/*.ts",
    "**/*.mts"
  ],
  "exclude": ["node_modules"]
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "components/ui/card.tsx".into(),
                content: r###"import * as React from "react"

import { cn } from "@/lib/utils"

function Card({
  className,
  size = "default",
  ...props
}: React.ComponentProps<"div"> & { size?: "default" | "sm" }) {
  return (
    <div
      data-slot="card"
      data-size={size}
      className={cn(
        "group/card flex flex-col gap-(--card-spacing) overflow-hidden rounded-xl bg-card py-(--card-spacing) text-sm text-card-foreground ring-1 ring-foreground/10 [--card-spacing:--spacing(4)] has-data-[slot=card-footer]:pb-0 has-[>img:first-child]:pt-0 data-[size=sm]:[--card-spacing:--spacing(3)] data-[size=sm]:has-data-[slot=card-footer]:pb-0 *:[img:first-child]:rounded-t-xl *:[img:last-child]:rounded-b-xl",
        className
      )}
      {...props}
    />
  )
}

function CardHeader({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-header"
      className={cn(
        "group/card-header @container/card-header grid auto-rows-min items-start gap-1 rounded-t-xl px-(--card-spacing) has-data-[slot=card-action]:grid-cols-[1fr_auto] has-data-[slot=card-description]:grid-rows-[auto_auto] [.border-b]:pb-(--card-spacing)",
        className
      )}
      {...props}
    />
  )
}

function CardTitle({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-title"
      className={cn(
        "font-heading text-base leading-snug font-medium group-data-[size=sm]/card:text-sm",
        className
      )}
      {...props}
    />
  )
}

function CardDescription({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-description"
      className={cn("text-sm text-muted-foreground", className)}
      {...props}
    />
  )
}

function CardAction({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-action"
      className={cn(
        "col-start-2 row-span-2 row-start-1 self-start justify-self-end",
        className
      )}
      {...props}
    />
  )
}

function CardContent({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-content"
      className={cn("px-(--card-spacing)", className)}
      {...props}
    />
  )
}

function CardFooter({ className, ...props }: React.ComponentProps<"div">) {
  return (
    <div
      data-slot="card-footer"
      className={cn(
        "flex items-center rounded-b-xl border-t bg-muted/50 p-(--card-spacing)",
        className
      )}
      {...props}
    />
  )
}

export {
  Card,
  CardHeader,
  CardFooter,
  CardTitle,
  CardAction,
  CardDescription,
  CardContent,
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "components/ui/button.tsx".into(),
                content: r###"import * as React from "react"
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
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "components/ui/input.tsx".into(),
                content: r###"import * as React from "react"
import { cn } from "@/lib/utils"

const Input = React.forwardRef<HTMLInputElement, React.ComponentProps<"input">>(
  ({ className, type, ...props }, ref) => {
    return (
      <input
        type={type}
        className={cn(
          "flex h-10 w-full rounded-md border border-input bg-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-sm file:font-medium placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50",
          className
        )}
        ref={ref}
        {...props}
      />
    )
  }
)
Input.displayName = "Input"

export { Input }
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "components/providers.tsx".into(),
                content: r###"'use client';

import React, { useEffect, useState } from 'react';
import { QueryClient, QueryClientProvider } from '@tanstack/react-query';
import { ThemeProvider as NextThemesProvider } from 'next-themes';
import { useAppStore } from '@/store/useAppStore';

export function Providers({ children }: { children: React.ReactNode }) {
  // Rehydrate Zustand store on mount (client side only) to prevent SSR hydration errors
  useEffect(() => {
    useAppStore.persist.rehydrate();
  }, []);

  const [queryClient] = useState(
    () =>
      new QueryClient({
        defaultOptions: {
          queries: {
            staleTime: 1000 * 60 * 5, // 5 minutes
            retry: 1,
            refetchOnWindowFocus: false,
          },
        },
      })
  );

  return (
    <QueryClientProvider client={queryClient}>
      <NextThemesProvider
        attribute="class"
        defaultTheme="system"
        enableSystem
        disableTransitionOnChange
      >
        {children}
      </NextThemesProvider>
    </QueryClientProvider>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "components/Navbar.tsx".into(),
                content: r###"'use client';

import React, { useState } from 'react';
import { useAuth } from '@/hooks/useAuth';
import { ThemeToggle } from './ThemeToggle';
import { Button } from './ui/button';
import { Input } from './ui/input';
import { LogOut, User as UserIcon, Shield, Loader2, Home, Newspaper } from 'lucide-react';
import { LoginFormSchema, RegisterFormSchema } from '@/types/schema';

export function Navbar() {
  const { user, login, register, logout, isLoggingIn, isRegistering, isLoggingOut } = useAuth();
  const [isAuthModalOpen, setIsAuthModalOpen] = useState(false);
  const [authMode, setAuthMode] = useState<'login' | 'register'>('login');
  
  // Form states
  const [email, setEmail] = useState('');
  const [password, setPassword] = useState('');
  const [name, setName] = useState('');
  const [error, setError] = useState<string | null>(null);

  const handleAuthSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setError(null);

    try {
      if (authMode === 'login') {
        const validated = LoginFormSchema.safeParse({ email, password });
        if (!validated.success) {
          setError(validated.error.issues[0].message);
          return;
        }
        await login({ email, password });
      } else {
        const validated = RegisterFormSchema.safeParse({ email, name, password });
        if (!validated.success) {
          setError(validated.error.issues[0].message);
          return;
        }
        await register({ email, name, password });
      }
      setIsAuthModalOpen(false);
      resetForm();
    } catch (err: any) {
      setError(err.response?.data?.error || 'Authentication failed. Please try again.');
    }
  };

  const resetForm = () => {
    setEmail('');
    setPassword('');
    setName('');
    setError(null);
  };

  const openAuth = (mode: 'login' | 'register') => {
    setAuthMode(mode);
    setError(null);
    setIsAuthModalOpen(true);
  };

  return (
    <>
      <nav className="fixed top-0 left-0 right-0 h-16 border-b border-border/40 bg-background/80 backdrop-blur-md z-40 flex items-center justify-between px-6 transition-colors duration-300">
        <div className="flex items-center gap-8">
          <a href="#" className="text-xl font-bold tracking-tight text-primary hover:opacity-80 transition-opacity flex items-center gap-2">
            <img src="https://raw.githubusercontent.com/aswin402/offpkg/main/doc/logo.svg" alt="Offpkg Logo" className="w-6 h-6" />
            <span>OFFPKG</span>
            <span className="text-xs bg-primary/10 text-primary px-2 py-0.5 rounded-full ml-1 font-normal">NEXT</span>
          </a>
          <div className="hidden md:flex items-center gap-6">
            <a href="#hero" className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors flex items-center gap-1.5">
              <Home className="w-4 h-4" /> Home
            </a>
            <a href="#posts" className="text-sm font-medium text-muted-foreground hover:text-foreground transition-colors flex items-center gap-1.5">
              <Newspaper className="w-4 h-4" /> Feed
            </a>
          </div>
        </div>

        <div className="flex items-center gap-4">
          <ThemeToggle />

          {user ? (
            <div className="flex items-center gap-4">
              <div className="hidden sm:flex flex-col items-end text-xs">
                <span className="font-semibold text-foreground flex items-center gap-1">
                  {user.role === 'ADMIN' && <Shield className="w-3.5 h-3.5 text-red-500" />}
                  {user.name || 'User'}
                </span>
                <span className="text-muted-foreground">{user.email}</span>
              </div>
              <Button 
                variant="outline" 
                size="sm" 
                onClick={() => logout()}
                disabled={isLoggingOut}
                className="flex items-center gap-2"
              >
                {isLoggingOut ? <Loader2 className="w-4 h-4 animate-spin" /> : <LogOut className="w-4 h-4" />}
                Logout
              </Button>
            </div>
          ) : (
            <div className="flex items-center gap-2">
              <Button variant="ghost" size="sm" onClick={() => openAuth('login')}>
                Sign In
              </Button>
              <Button size="sm" onClick={() => openAuth('register')}>
                Sign Up
              </Button>
            </div>
          )}
        </div>
      </nav>

      {/* Auth Modal overlay */}
      {isAuthModalOpen && (
        <div className="fixed inset-0 bg-black/60 backdrop-blur-sm z-50 flex items-center justify-center p-4 animate-in fade-in duration-200">
          <div className="bg-card text-card-foreground border border-border w-full max-w-md p-8 rounded-3xl shadow-2xl relative animate-in zoom-in-95 duration-200">
            <button
              onClick={() => { setIsAuthModalOpen(false); resetForm(); }}
              className="absolute top-4 right-4 text-muted-foreground hover:text-foreground text-xl font-semibold w-8 h-8 rounded-full flex items-center justify-center hover:bg-muted"
            >
              ×
            </button>
            <h3 className="text-2xl font-bold mb-2">
              {authMode === 'login' ? 'Welcome Back' : 'Create an Account'}
            </h3>
            <p className="text-muted-foreground text-sm mb-6">
              {authMode === 'login' 
                ? 'Sign in to access database seeding, posts, and server actions.' 
                : 'Sign up to create your profile and share posts in the feed.'
              }
            </p>

            <form onSubmit={handleAuthSubmit} className="space-y-4">
              {error && (
                <div className="p-3 bg-destructive/10 text-destructive text-sm rounded-xl border border-destructive/20 font-medium">
                  {error}
                </div>
              )}

              {authMode === 'register' && (
                <div className="space-y-1.5">
                  <label className="text-xs font-semibold text-muted-foreground">Full Name</label>
                  <Input
                    type="text"
                    placeholder="Aswin Dev"
                    value={name}
                    onChange={(e) => setName(e.target.value)}
                    required
                  />
                </div>
              )}

              <div className="space-y-1.5">
                <label className="text-xs font-semibold text-muted-foreground">Email Address</label>
                <Input
                  type="email"
                  placeholder="name@example.com"
                  value={email}
                  onChange={(e) => setEmail(e.target.value)}
                  required
                />
              </div>

              <div className="space-y-1.5">
                <label className="text-xs font-semibold text-muted-foreground">Password</label>
                <Input
                  type="password"
                  placeholder="••••••••"
                  value={password}
                  onChange={(e) => setPassword(e.target.value)}
                  required
                />
              </div>

              <Button type="submit" className="w-full h-11 font-semibold text-base mt-2" disabled={isLoggingIn || isRegistering}>
                {isLoggingIn || isRegistering ? (
                  <Loader2 className="w-5 h-5 animate-spin mr-2" />
                ) : null}
                {authMode === 'login' ? 'Sign In' : 'Sign Up'}
              </Button>
            </form>

            <div className="mt-6 text-center text-sm text-muted-foreground">
              {authMode === 'login' ? (
                <>
                  Don't have an account?{' '}
                  <button 
                    onClick={() => openAuth('register')} 
                    className="text-primary font-semibold hover:underline"
                  >
                    Sign Up
                  </button>
                </>
              ) : (
                <>
                  Already have an account?{' '}
                  <button 
                    onClick={() => openAuth('login')} 
                    className="text-primary font-semibold hover:underline"
                  >
                    Sign In
                  </button>
                </>
              )}
            </div>
          </div>
        </div>
      )}
    </>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "components/ThemeToggle.tsx".into(),
                content: r###"'use client';

import { useTheme } from 'next-themes';
import { Sun, Moon } from 'lucide-react';
import { Button } from './ui/button';
import { useEffect, useState } from 'react';

export function ThemeToggle() {
  const { setTheme, resolvedTheme } = useTheme();
  const [mounted, setMounted] = useState(false);

  useEffect(() => {
    setMounted(true);
  }, []);

  if (!mounted) {
    return <div className="w-10 h-10 rounded-md bg-muted/40" />;
  }

  return (
    <Button
      variant="ghost"
      size="icon"
      onClick={() => setTheme(resolvedTheme === 'dark' ? 'light' : 'dark')}
      className="w-10 h-10 rounded-full hover:bg-accent hover:text-accent-foreground"
      title="Toggle Theme"
    >
      {resolvedTheme === 'dark' ? (
        <Sun className="h-5 w-5 text-yellow-400 transition-all" />
      ) : (
        <Moon className="h-5 w-5 text-indigo-600 transition-all" />
      )}
      <span className="sr-only">Toggle theme</span>
    </Button>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "eslint.config.mjs".into(),
                content: r###"import { defineConfig, globalIgnores } from "eslint/config";
import nextVitals from "eslint-config-next/core-web-vitals";
import nextTs from "eslint-config-next/typescript";

const eslintConfig = defineConfig([
  ...nextVitals,
  ...nextTs,
  // Override default ignores of eslint-config-next.
  globalIgnores([
    // Default ignores of eslint-config-next:
    ".next/**",
    "out/**",
    "build/**",
    "next-env.d.ts",
  ]),
]);

export default eslintConfig;
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "app/globals.css".into(),
                content: r###"@import "tailwindcss";
@import "tw-animate-css";
@import "shadcn/tailwind.css";

@custom-variant dark (&:is(.dark *));

@theme inline {
  --color-background: var(--background);
  --color-foreground: var(--foreground);
  --font-sans: var(--font-sans);
  --font-mono: var(--font-geist-mono);
  --font-heading: var(--font-sans);
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
}

@layer base {
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
}"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "app/layout.tsx".into(),
                content: r###"import type { Metadata } from "next";
import { Geist, Geist_Mono, Inter } from "next/font/google";
import "./globals.css";
import { cn } from "@/lib/utils";
import { Providers } from "@/components/providers";

const inter = Inter({ subsets: ['latin'], variable: '--font-sans' });

const geistSans = Geist({
  variable: "--font-geist-sans",
  subsets: ["latin"],
});

const geistMono = Geist_Mono({
  variable: "--font-geist-mono",
  subsets: ["latin"],
});

export const metadata: Metadata = {
  title: "Offpkg Next.js Template",
  description: "Modern full-stack starter template with Next.js, Bun, Tailwind CSS v4, Shadcn UI, and Prisma",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html
      lang="en"
      suppressHydrationWarning
      className={cn(
        "h-full",
        "antialiased",
        geistSans.variable,
        geistMono.variable,
        "font-sans",
        inter.variable
      )}
    >
      <body className="min-h-full flex flex-col bg-background text-foreground transition-colors duration-300">
        <Providers>
          {children}
        </Providers>
      </body>
    </html>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "app/page.tsx".into(),
                content: r###"'use client';

import React, { useState } from 'react';
import { useAuth } from '@/hooks/useAuth';
import { usePosts } from '@/hooks/usePosts';
import { Navbar } from '@/components/Navbar';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Card, CardHeader, CardTitle, CardDescription, CardContent, CardFooter } from '@/components/ui/card';
import { 
  Zap, 
  ShieldCheck, 
  Package, 
  Database, 
  Flame,
  ArrowRight,
  Loader2,
  Lock,
  PlusCircle,
  RefreshCw,
  Sparkles,
  BookOpen
} from 'lucide-react';
import apiClient from '@/lib/api-client';
import { useQueryClient } from '@tanstack/react-query';

export default function Home() {
  const { user } = useAuth();
  const { posts, isLoading: postsLoading, createPost, isCreating } = usePosts();
  const queryClient = useQueryClient();

  const [postTitle, setPostTitle] = useState('');
  const [postContent, setPostContent] = useState('');
  const [postError, setPostError] = useState<string | null>(null);
  
  const [seeding, setSeeding] = useState(false);
  const [seedStatus, setSeedStatus] = useState<string | null>(null);

  const handleCreatePost = async (e: React.FormEvent) => {
    e.preventDefault();
    setPostError(null);

    const token = typeof window !== 'undefined' ? localStorage.getItem('auth-token') : null;
    
    if (!token) {
      setPostError('You must be signed in to post.');
      return;
    }

    try {
      await createPost({
        title: postTitle,
        content: postContent,
        published: true,
        token
      });
      setPostTitle('');
      setPostContent('');
    } catch (err: any) {
      setPostError(err.response?.data?.error || 'Failed to create post.');
    }
  };

  const handleSeedDatabase = async () => {
    setSeeding(true);
    setSeedStatus(null);
    try {
      const { data } = await apiClient.post<{ success: boolean; message: string }>('/api/db/seed');
      if (data.success) {
        setSeedStatus('Database seeded successfully!');
        queryClient.invalidateQueries({ queryKey: ['posts'] });
      }
    } catch (err: any) {
      setSeedStatus(err.response?.data?.error || 'Seeding failed.');
    } finally {
      setSeeding(false);
    }
  };

  return (
    <div className="min-h-screen flex flex-col bg-background text-foreground transition-colors duration-300">
      <Navbar />

      <main className="flex-1 pt-16">
        {/* Hero Section */}
        <section id="hero" className="relative min-h-[70vh] flex flex-col items-center justify-center p-8 text-center overflow-hidden border-b border-border/40">
          <div className="absolute -inset-10 bg-gradient-to-tr from-primary/10 via-transparent to-accent/15 blur-3xl opacity-60 rounded-full" />
          <div className="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 w-[500px] h-[500px] bg-primary/5 rounded-full blur-3xl" />
          
          <div className="relative z-10 max-w-3xl space-y-6">
            <div className="inline-flex items-center gap-2 px-3 py-1.5 rounded-full bg-primary/10 text-primary text-xs font-semibold tracking-wide border border-primary/20 animate-pulse">
              <Sparkles className="w-3.5 h-3.5" /> Upgraded Fullstack Template
            </div>
            
            <h1 className="text-5xl md:text-7xl font-bold tracking-tight leading-none bg-gradient-to-r from-foreground via-foreground/90 to-muted-foreground bg-clip-text text-transparent">
              Next.js 16 + Bun <br />
              <span className="bg-gradient-to-r from-primary to-purple-600 bg-clip-text text-transparent">Prisma 7 & Tailwind v4</span>
            </h1>
            
            <p className="text-muted-foreground text-lg md:text-xl max-w-2xl mx-auto leading-relaxed">
              The ultimate high-performance production template. Blazing fast, client-hydrate safe, 
              type-safe validations, secure JWT auth, and relational database integrations.
            </p>

            <div className="flex flex-wrap items-center justify-center gap-4 pt-4">
              <a href="#feed">
                <Button size="lg" className="h-12 px-6 rounded-xl font-medium shadow-md shadow-primary/25 hover:shadow-lg transition-all flex items-center gap-2 group">
                  Explore Feed <ArrowRight className="w-4 h-4 group-hover:translate-x-1 transition-transform" />
                </Button>
              </a>
              <a href="#features">
                <Button size="lg" variant="outline" className="h-12 px-6 rounded-xl font-medium border-border/60 hover:bg-accent/40">
                  Tech Stack
                </Button>
              </a>
            </div>
          </div>
        </section>

        {/* Features Grid */}
        <section id="features" className="py-20 px-6 max-w-6xl mx-auto space-y-12">
          <div className="text-center space-y-3">
            <h2 className="text-3xl font-bold tracking-tight">Core Architecture</h2>
            <p className="text-muted-foreground max-w-xl mx-auto">
              Pre-configured tools combined to give you the most efficient developer experience.
            </p>
          </div>

          <div className="grid sm:grid-cols-2 lg:grid-cols-4 gap-6">
            <Card className="hover:border-primary/40 transition-all duration-300">
              <CardHeader className="space-y-2">
                <div className="w-10 h-10 rounded-xl bg-orange-500/10 flex items-center justify-center">
                  <Flame className="w-5 h-5 text-orange-500" />
                </div>
                <CardTitle>Bun Runtime</CardTitle>
                <CardDescription>Lightning fast bundler and package manager for zero startup overhead.</CardDescription>
              </CardHeader>
            </Card>

            <Card className="hover:border-primary/40 transition-all duration-300">
              <CardHeader className="space-y-2">
                <div className="w-10 h-10 rounded-xl bg-blue-500/10 flex items-center justify-center">
                  <Zap className="w-5 h-5 text-blue-500" />
                </div>
                <CardTitle>Next.js 16</CardTitle>
                <CardDescription>Advanced App Router with React Canary & Async Route Params support.</CardDescription>
              </CardHeader>
            </Card>

            <Card className="hover:border-primary/40 transition-all duration-300">
              <CardHeader className="space-y-2">
                <div className="w-10 h-10 rounded-xl bg-purple-500/10 flex items-center justify-center">
                  <Database className="w-5 h-5 text-purple-500" />
                </div>
                <CardTitle>Prisma 7 Client</CardTitle>
                <CardDescription>Next-gen ORM with Rust-free native JS driver adapters and PostgreSQL.</CardDescription>
              </CardHeader>
            </Card>

            <Card className="hover:border-primary/40 transition-all duration-300">
              <CardHeader className="space-y-2">
                <div className="w-10 h-10 rounded-xl bg-emerald-500/10 flex items-center justify-center">
                  <ShieldCheck className="w-5 h-5 text-emerald-500" />
                </div>
                <CardTitle>Zod & Zustand</CardTitle>
                <CardDescription>Type-safe form schemas combined with hydrate-safe global Zustand state.</CardDescription>
              </CardHeader>
            </Card>
          </div>
        </section>

        {/* Database & Interactive Actions */}
        <section className="py-12 border-t border-b border-border/40 bg-muted/30">
          <div className="max-w-4xl mx-auto px-6 grid md:grid-cols-2 gap-8 items-center">
            <div className="space-y-4">
              <h3 className="text-2xl font-bold tracking-tight flex items-center gap-2">
                <Database className="w-6 h-6 text-primary" /> Dev Sandbox Tools
              </h3>
              <p className="text-muted-foreground leading-relaxed">
                Reset your database, wipe existing collections, and seed beautiful dummy user and post 
                relations directly via this API wrapper trigger. Make testing HMR and queries instantly visual.
              </p>
              
              <div className="flex items-center gap-4">
                <Button 
                  onClick={handleSeedDatabase} 
                  disabled={seeding}
                  className="flex items-center gap-2 rounded-xl"
                >
                  {seeding ? <Loader2 className="w-4 h-4 animate-spin" /> : <RefreshCw className="w-4 h-4" />}
                  Seed Mock Database
                </Button>
                {seedStatus && (
                  <span className="text-sm font-medium text-primary animate-pulse">{seedStatus}</span>
                )}
              </div>
            </div>
            
            <Card className="bg-card/40 backdrop-blur-sm">
              <CardHeader>
                <CardTitle className="flex items-center gap-2 text-base">
                  <BookOpen className="w-4 h-4 text-purple-500" /> CLI Seeding Shortcut
                </CardTitle>
              </CardHeader>
              <CardContent className="space-y-2 text-xs font-mono bg-muted/60 p-4 rounded-xl mx-4 border text-muted-foreground">
                <div># Seeding directly via Bun CLI:</div>
                <div className="text-foreground">bun run db:seed</div>
                <div className="mt-4"># Spin up Prisma Studio to view database:</div>
                <div className="text-foreground">bun run db:studio</div>
              </CardContent>
            </Card>
          </div>
        </section>

        {/* Feed & Post Form Section */}
        <section id="feed" className="py-20 max-w-5xl mx-auto px-6 grid md:grid-cols-3 gap-8">
          
          {/* Post Form Panel (Left) */}
          <div className="md:col-span-1 space-y-6">
            <h3 className="text-xl font-bold tracking-tight">Post Panel</h3>
            {user ? (
              <Card className="border-primary/20 bg-card/50">
                <CardHeader>
                  <CardTitle className="text-base flex items-center gap-2">
                    <PlusCircle className="w-4 h-4 text-primary" /> Create a Post
                  </CardTitle>
                  <CardDescription>Share your thoughts on fullstack development.</CardDescription>
                </CardHeader>
                <form onSubmit={handleCreatePost}>
                  <CardContent className="space-y-4">
                    {postError && (
                      <div className="p-3 bg-destructive/10 text-destructive text-xs rounded-lg font-medium border border-destructive/20">
                        {postError}
                      </div>
                    )}
                    <div className="space-y-1.5">
                      <label className="text-xs font-semibold text-muted-foreground">Title</label>
                      <Input
                        type="text"
                        placeholder="Next.js 16 is amazing!"
                        value={postTitle}
                        onChange={(e) => setPostTitle(e.target.value)}
                        required
                      />
                    </div>
                    <div className="space-y-1.5">
                      <label className="text-xs font-semibold text-muted-foreground">Content</label>
                      <textarea
                        className="flex min-h-[100px] w-full rounded-md border border-input bg-background px-3 py-2 text-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:cursor-not-allowed disabled:opacity-50"
                        placeholder="Write your post here..."
                        value={postContent}
                        onChange={(e) => setPostContent(e.target.value)}
                        required
                      />
                    </div>
                  </CardContent>
                  <CardFooter className="bg-transparent border-none">
                    <Button type="submit" className="w-full" disabled={isCreating}>
                      {isCreating ? <Loader2 className="w-4 h-4 animate-spin mr-2" /> : null}
                      Post to Feed
                    </Button>
                  </CardFooter>
                </form>
              </Card>
            ) : (
              <Card className="border-dashed bg-muted/20">
                <CardHeader className="text-center p-6 space-y-4">
                  <div className="mx-auto w-12 h-12 rounded-full bg-muted flex items-center justify-center">
                    <Lock className="w-5 h-5 text-muted-foreground" />
                  </div>
                  <div className="space-y-1">
                    <CardTitle className="text-base">Authenticated Only</CardTitle>
                    <CardDescription className="text-xs">
                      Sign in or create an account to post thoughts directly to the database.
                    </CardDescription>
                  </div>
                </CardHeader>
              </Card>
            )}
          </div>

          {/* Feed List Panel (Right) */}
          <div className="md:col-span-2 space-y-6">
            <div className="flex items-center justify-between">
              <h3 className="text-xl font-bold tracking-tight">Active Feed</h3>
              <span className="text-xs px-2 py-0.5 bg-muted text-muted-foreground rounded-full border">
                {posts.length} {posts.length === 1 ? 'post' : 'posts'}
              </span>
            </div>

            {postsLoading ? (
              <div className="flex flex-col items-center justify-center py-20 space-y-3">
                <Loader2 className="w-8 h-8 animate-spin text-primary" />
                <p className="text-muted-foreground text-sm">Querying database posts...</p>
              </div>
            ) : posts.length === 0 ? (
              <Card className="bg-muted/10 border-dashed py-20 text-center">
                <CardContent className="space-y-2">
                  <p className="text-muted-foreground">No posts found in database.</p>
                  <p className="text-xs text-muted-foreground/80">Click the 'Seed Mock Database' button above to populate.</p>
                </CardContent>
              </Card>
            ) : (
              <div className="space-y-4">
                {posts.map((post) => (
                  <Card key={post.id} className="hover:shadow-md transition-shadow">
                    <CardHeader className="pb-2">
                      <div className="flex justify-between items-start">
                        <CardTitle className="text-lg font-semibold leading-snug">{post.title}</CardTitle>
                        <span className="text-[10px] bg-secondary/80 text-secondary-foreground font-semibold px-2 py-0.5 rounded-full border border-secondary">
                          {post.views} views
                        </span>
                      </div>
                      <CardDescription className="text-xs">
                        Posted by <span className="font-semibold text-foreground">{post.author?.name || 'Anonymous'}</span> ({post.author?.email})
                      </CardDescription>
                    </CardHeader>
                    <CardContent>
                      <p className="text-muted-foreground leading-relaxed whitespace-pre-line text-sm">
                        {post.content}
                      </p>
                    </CardContent>
                    <CardFooter className="py-2 text-[10px] text-muted-foreground flex justify-between bg-muted/20 border-t">
                      <span>ID: {post.id}</span>
                      <span>{new Date(post.createdAt).toLocaleDateString()}</span>
                    </CardFooter>
                  </Card>
                ))}
              </div>
            )}
          </div>
        </section>
      </main>

      <footer className="py-8 border-t border-border/40 bg-muted/10 text-center text-xs text-muted-foreground">
        <p>© 2026 Offpkg. Next.js 16 + Prisma 7 Template Starter.</p>
      </footer>
    </div>
  );
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "app/api/db/seed/route.ts".into(),
                content: r###"import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma';
import bcrypt from 'bcryptjs';
import { logger } from '@/lib/logger';
import { withLogging } from '@/lib/api-logger';

async function seedHandler(request: NextRequest) {
  if (process.env.NODE_ENV === 'production') {
    return NextResponse.json(
      { success: false, error: 'Seeding API disabled in production' },
      { status: 403 }
    );
  }

  try {
    logger.info('Database seeding requested via API...');

    await prisma.post.deleteMany();
    await prisma.user.deleteMany();

    const adminPassword = await bcrypt.hash('admin123', 10);
    const userPassword = await bcrypt.hash('user123', 10);

    const admin = await prisma.user.create({
      data: {
        email: 'admin@offpkg.com',
        name: 'Admin User',
        password: adminPassword,
        role: 'ADMIN',
      },
    });

    const user1 = await prisma.user.create({
      data: {
        email: 'user1@offpkg.com',
        name: 'Aswin Dev',
        password: userPassword,
        role: 'USER',
      },
    });

    const user2 = await prisma.user.create({
      data: {
        email: 'user2@offpkg.com',
        name: 'Jane Smith',
        password: userPassword,
        role: 'USER',
      },
    });

    await prisma.post.createMany({
      data: [
        {
          title: 'Getting Started with Next.js 16 and Prisma 7',
          content: 'Next.js 16 and Prisma 7 provide an incredibly powerful combo for full-stack React applications. By combining Next.js Server Actions with Prisma driver adapters, you can build blazing fast, edge-ready applications.',
          published: true,
          authorId: user1.id,
          views: 125,
        },
        {
          title: 'Building Beautiful Interfaces with Tailwind CSS v4',
          content: 'Tailwind CSS v4 introduces a streamlined engine, CSS-first configuration, and native cascading layers. It makes managing design systems a breeze without the bloat of traditional CSS setups.',
          published: true,
          authorId: user1.id,
          views: 348,
        },
        {
          title: 'The Future of State Management with Zustand',
          content: 'Zustand is a small, fast, and scalable bear-bones state-management solution. It has a comfy API based on hooks, is not opinionated, and doesn\'t wrap your app in providers.',
          published: true,
          authorId: user2.id,
          views: 99,
        },
        {
          title: 'Next.js 16 Asynchronous Route Parameters',
          content: 'In Next.js 16, page and route parameters (params) are now resolved as Promises. You must await params before reading their values to ensure compatibility and runtime speed.',
          published: true,
          authorId: admin.id,
          views: 42,
        },
      ],
    });

    logger.info('Database seeded successfully via API!');
    return NextResponse.json({ success: true, message: 'Database seeded successfully' });
  } catch (error: any) {
    logger.error('Error seeding database via API:', error);
    return NextResponse.json(
      { success: false, error: error.message || 'Failed to seed database' },
      { status: 500 }
    );
  }
}

export const POST = withLogging(seedHandler);
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "app/api/posts/route.ts".into(),
                content: r###"import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma';
import { CreatePostSchema } from '@/types/schema';
import { verifyToken } from '@/lib/jwt';
import { logger } from '@/lib/logger';
import { withLogging } from '@/lib/api-logger';

async function getHandler(request: NextRequest) {
  try {
    const { searchParams } = new URL(request.url);
    const limit = parseInt(searchParams.get('limit') || '10', 10);
    
    const posts = await prisma.post.findMany({
      take: limit,
      orderBy: { createdAt: 'desc' },
      include: {
        author: {
          select: {
            id: true,
            name: true,
            email: true,
            role: true,
          },
        },
      },
    });

    return NextResponse.json({ success: true, data: posts });
  } catch (error: any) {
    logger.error('Error fetching posts in API:', error);
    return NextResponse.json(
      { success: false, error: 'Failed to fetch posts' },
      { status: 500 }
    );
  }
}

async function postHandler(request: NextRequest) {
  try {
    const authHeader = request.headers.get('Authorization');
    let userId: string | null = null;
    
    if (authHeader && authHeader.startsWith('Bearer ')) {
      const token = authHeader.split(' ')[1];
      const payload = verifyToken(token);
      if (payload) {
        userId = payload.userId;
      }
    }
    
    if (!userId) {
      return NextResponse.json(
        { success: false, error: 'Unauthorized' },
        { status: 401 }
      );
    }

    const body = await request.json();
    const validatedData = CreatePostSchema.safeParse(body);

    if (!validatedData.success) {
      return NextResponse.json(
        { success: false, errors: validatedData.error.flatten().fieldErrors },
        { status: 400 }
      );
    }

    const post = await prisma.post.create({
      data: {
        title: validatedData.data.title,
        content: validatedData.data.content,
        published: validatedData.data.published,
        authorId: userId,
      },
      include: {
        author: {
          select: {
            id: true,
            name: true,
            email: true,
          },
        },
      },
    });

    return NextResponse.json({ success: true, data: post }, { status: 201 });
  } catch (error: any) {
    logger.error('Error creating post in API:', error);
    return NextResponse.json(
      { success: false, error: 'Internal Server Error' },
      { status: 500 }
    );
  }
}

export const GET = withLogging(getHandler);
export const POST = withLogging(postHandler);
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "app/api/auth/register/route.ts".into(),
                content: r###"import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma';
import { RegisterFormSchema } from '@/types/schema';
import { hashPassword } from '@/lib/auth-utils';
import { signToken } from '@/lib/jwt';
import { logger } from '@/lib/logger';
import { withLogging } from '@/lib/api-logger';

async function registerHandler(request: NextRequest) {
  try {
    const body = await request.json();
    const validatedData = RegisterFormSchema.safeParse(body);

    if (!validatedData.success) {
      return NextResponse.json(
        { success: false, errors: validatedData.error.flatten().fieldErrors },
        { status: 400 }
      );
    }

    const { email, name, password } = validatedData.data;

    const existingUser = await prisma.user.findUnique({
      where: { email },
    });

    if (existingUser) {
      return NextResponse.json(
        { success: false, error: 'User with this email already exists' },
        { status: 409 }
      );
    }

    const hashedPassword = await hashPassword(password);

    const user = await prisma.user.create({
      data: {
        email,
        name,
        password: hashedPassword,
        role: 'USER',
      },
    });

    const token = signToken({
      userId: user.id,
      email: user.email,
      role: user.role,
    });

    const responseUser = {
      id: user.id,
      email: user.email,
      name: user.name,
      role: user.role,
    };

    const response = NextResponse.json(
      { success: true, data: responseUser, token },
      { status: 201 }
    );

    response.cookies.set('token', token, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'strict',
      maxAge: 60 * 60 * 24 * 7,
      path: '/',
    });

    return response;
  } catch (error: any) {
    logger.error('Registration Error:', error);
    return NextResponse.json(
      { success: false, error: 'Internal Server Error' },
      { status: 500 }
    );
  }
}

export const POST = withLogging(registerHandler);
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "app/api/auth/me/route.ts".into(),
                content: r###"import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma';
import { verifyToken } from '@/lib/jwt';
import { logger } from '@/lib/logger';
import { withLogging } from '@/lib/api-logger';

async function getHandler(request: NextRequest) {
  try {
    let token = request.cookies.get('token')?.value;

    if (!token) {
      const authHeader = request.headers.get('Authorization');
      if (authHeader && authHeader.startsWith('Bearer ')) {
        token = authHeader.split(' ')[1];
      }
    }

    if (!token) {
      return NextResponse.json(
        { success: false, error: 'Unauthorized' },
        { status: 401 }
      );
    }

    const payload = verifyToken(token);

    if (!payload) {
      return NextResponse.json(
        { success: false, error: 'Unauthorized or expired token' },
        { status: 401 }
      );
    }

    const user = await prisma.user.findUnique({
      where: { id: payload.userId },
      select: {
        id: true,
        email: true,
        name: true,
        role: true,
        createdAt: true,
      },
    });

    if (!user) {
      return NextResponse.json(
        { success: false, error: 'User not found' },
        { status: 404 }
      );
    }

    return NextResponse.json({ success: true, data: user });
  } catch (error: any) {
    logger.error('Auth check error:', error);
    return NextResponse.json(
      { success: false, error: 'Internal Server Error' },
      { status: 500 }
    );
  }
}

async function postHandler() {
  const response = NextResponse.json({ success: true, message: 'Logged out successfully' });
  response.cookies.set('token', '', {
    httpOnly: true,
    expires: new Date(0),
    path: '/',
  });
  return response;
}

export const GET = withLogging(getHandler);
export const POST = withLogging(postHandler);
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "app/api/auth/login/route.ts".into(),
                content: r###"import { NextRequest, NextResponse } from 'next/server';
import { prisma } from '@/lib/prisma';
import { LoginFormSchema } from '@/types/schema';
import { comparePassword } from '@/lib/auth-utils';
import { signToken } from '@/lib/jwt';
import { logger } from '@/lib/logger';
import { withLogging } from '@/lib/api-logger';

async function loginHandler(request: NextRequest) {
  try {
    const body = await request.json();
    const validatedData = LoginFormSchema.safeParse(body);

    if (!validatedData.success) {
      return NextResponse.json(
        { success: false, errors: validatedData.error.flatten().fieldErrors },
        { status: 400 }
      );
    }

    const { email, password } = validatedData.data;

    const user = await prisma.user.findUnique({
      where: { email },
    });

    if (!user) {
      return NextResponse.json(
        { success: false, error: 'Invalid email or password' },
        { status: 401 }
      );
    }

    const isValidPassword = await comparePassword(password, user.password);

    if (!isValidPassword) {
      return NextResponse.json(
        { success: false, error: 'Invalid email or password' },
        { status: 401 }
      );
    }

    const token = signToken({
      userId: user.id,
      email: user.email,
      role: user.role,
    });

    const responseUser = {
      id: user.id,
      email: user.email,
      name: user.name,
      role: user.role,
    };

    const response = NextResponse.json(
      { success: true, data: responseUser, token },
      { status: 200 }
    );

    response.cookies.set('token', token, {
      httpOnly: true,
      secure: process.env.NODE_ENV === 'production',
      sameSite: 'strict',
      maxAge: 60 * 60 * 24 * 7,
      path: '/',
    });

    return response;
  } catch (error: any) {
    logger.error('Login Error:', error);
    return NextResponse.json(
      { success: false, error: 'Internal Server Error' },
      { status: 500 }
    );
  }
}

export const POST = withLogging(loginHandler);
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "app/favicon.ico".into(),
                content: "".into(),
                binary_content: Some(include_bytes!("assets/next_favicon.ico").to_vec()),
            },
            StackFile {
                path: "components.json".into(),
                content: r###"{
  "$schema": "https://ui.shadcn.com/schema.json",
  "style": "radix-nova",
  "rsc": true,
  "tsx": true,
  "tailwind": {
    "config": "",
    "css": "app/globals.css",
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
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "package.json".into(),
                content: r###"{
  "name": "offpkg_next",
  "version": "0.1.0",
  "private": true,
  "scripts": {
    "dev": "next dev",
    "build": "next build",
    "start": "next start",
    "lint": "eslint",
    "db:migrate": "prisma migrate dev",
    "db:generate": "prisma generate",
    "db:seed": "prisma db seed",
    "db:studio": "prisma studio"
  },
  "dependencies": {
    "@aws-sdk/client-s3": "^3.1063.0",
    "@aws-sdk/s3-request-presigner": "^3.1063.0",
    "@hookform/resolvers": "^5.4.0",
    "@prisma/adapter-pg": "^7.8.0",
    "@prisma/client": "^7.8.0",
    "@tanstack/react-query": "^5.101.0",
    "axios": "^1.17.0",
    "bcryptjs": "^3.0.3",
    "class-variance-authority": "^0.7.1",
    "clsx": "^2.1.1",
    "cors": "^2.8.6",
    "ioredis": "^5.11.1",
    "jsonwebtoken": "^9.0.3",
    "lucide-react": "^1.17.0",
    "next": "16.2.7",
    "next-themes": "^0.4.6",
    "node-cron": "^4.2.1",
    "nodemailer": "^8.0.10",
    "pg": "^8.21.0",
    "pino": "^10.3.1",
    "radix-ui": "^1.5.0",
    "react": "19.2.4",
    "react-dom": "19.2.4",
    "react-hook-form": "^7.77.0",
    "shadcn": "^4.10.0",
    "superjson": "^2.2.6",
    "tailwind-merge": "^3.6.0",
    "tw-animate-css": "^1.4.0",
    "zod": "^4.4.3",
    "zustand": "^5.0.14"
  },
  "devDependencies": {
    "@tailwindcss/postcss": "^4",
    "@types/bcryptjs": "^3.0.0",
    "@types/cors": "^2.8.19",
    "@types/jsonwebtoken": "^9.0.10",
    "@types/node": "^20",
    "@types/node-cron": "^3.0.11",
    "@types/nodemailer": "^8.0.0",
    "@types/pg": "^8.20.0",
    "@types/react": "^19",
    "@types/react-dom": "^19",
    "eslint": "^9",
    "eslint-config-next": "16.2.7",
    "pino-pretty": "^13.1.3",
    "prisma": "^7.8.0",
    "tailwindcss": "^4",
    "typescript": "^5"
  },
  "ignoreScripts": [
    "sharp",
    "unrs-resolver"
  ],
  "trustedDependencies": [
    "sharp",
    "unrs-resolver"
  ],
  "prisma": {
    "seed": "bun prisma/seed.ts"
  }
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: ".gitignore".into(),
                content: r###"# See https://help.github.com/articles/ignoring-files/ for more about ignoring files.

# dependencies
/node_modules
/.pnp
.pnp.*
.yarn/*
!.yarn/patches
!.yarn/plugins
!.yarn/releases
!.yarn/versions

# testing
/coverage

# next.js
/.next/
/out/

# production
/build

# misc
.DS_Store
*.pem

# debug
npm-debug.log*
yarn-debug.log*
yarn-error.log*
.pnpm-debug.log*

# env files (can opt-in for committing if needed)
.env*

# vercel
.vercel

# typescript
*.tsbuildinfo
next-env.d.ts

/lib/generated/prisma
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/s3.ts".into(),
                content: r###"import { S3Client, PutObjectCommand, GetObjectCommand } from '@aws-sdk/client-s3';
import { getSignedUrl } from '@aws-sdk/s3-request-presigner';
import { pinoLogger } from './pino-logger';

const S3_REGION = process.env.S3_REGION || 'us-east-1';
const S3_BUCKET = process.env.S3_BUCKET || 'my-app-bucket';
const S3_ACCESS_KEY_ID = process.env.S3_ACCESS_KEY_ID || '';
const S3_SECRET_ACCESS_KEY = process.env.S3_SECRET_ACCESS_KEY || '';
const S3_ENDPOINT = process.env.S3_ENDPOINT; // Supports custom endpoints like MinIO / Cloudflare R2

const s3Client = new S3Client({
  region: S3_REGION,
  credentials: S3_ACCESS_KEY_ID && S3_SECRET_ACCESS_KEY ? {
    accessKeyId: S3_ACCESS_KEY_ID,
    secretAccessKey: S3_SECRET_ACCESS_KEY,
  } : undefined,
  endpoint: S3_ENDPOINT,
});

export { s3Client };

interface UploadParams {
  key: string;
  body: Buffer | Uint8Array;
  contentType: string;
}

export async function uploadToS3({ key, body, contentType }: UploadParams) {
  try {
    const command = new PutObjectCommand({
      Bucket: S3_BUCKET,
      Key: key,
      Body: body,
      ContentType: contentType,
    });
    
    await s3Client.send(command);
    pinoLogger.info(`Successfully uploaded object to S3: ${key}`);
    return { key, bucket: S3_BUCKET };
  } catch (error) {
    pinoLogger.error(error, `S3 upload error for key "${key}"`);
    throw error;
  }
}

export async function getPresignedDownloadUrl(key: string, expiresInSeconds = 3600) {
  try {
    const command = new GetObjectCommand({
      Bucket: S3_BUCKET,
      Key: key,
    });
    
    const url = await getSignedUrl(s3Client, command, { expiresIn: expiresInSeconds });
    return url;
  } catch (error) {
    pinoLogger.error(error, `S3 presigned download URL error for key "${key}"`);
    throw error;
  }
}

export async function getPresignedUploadUrl(key: string, contentType: string, expiresInSeconds = 3600) {
  try {
    const command = new PutObjectCommand({
      Bucket: S3_BUCKET,
      Key: key,
      ContentType: contentType,
    });
    
    const url = await getSignedUrl(s3Client, command, { expiresIn: expiresInSeconds });
    return url;
  } catch (error) {
    pinoLogger.error(error, `S3 presigned upload URL error for key "${key}"`);
    throw error;
  }
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/pino-logger.ts".into(),
                content: r###"import pino from 'pino';

const isDev = process.env.NODE_ENV !== 'production';

export const pinoLogger = pino({
  level: process.env.LOG_LEVEL || 'info',
  transport: isDev
    ? {
        target: 'pino-pretty',
        options: {
          colorize: true,
          translateTime: 'SYS:standard',
          ignore: 'pid,hostname',
        },
      }
    : undefined,
});
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/prisma.ts".into(),
                content: r###"import { PrismaClient } from './generated/prisma/client';
import { PrismaPg } from '@prisma/adapter-pg';
import pg from 'pg';

const { Pool } = pg;

const globalForPrisma = globalThis as unknown as {
  prisma: PrismaClient | undefined;
  pool: pg.Pool | undefined;
};

const connectionString = process.env.DATABASE_URL;

if (!connectionString && process.env.NODE_ENV !== 'production') {
  console.warn('Warning: DATABASE_URL is not set. Database operations will fail.');
}

export let prisma: PrismaClient;

if (process.env.NODE_ENV === 'production') {
  const pool = new Pool({ connectionString });
  const adapter = new PrismaPg(pool);
  prisma = new PrismaClient({ adapter });
} else {
  if (!globalForPrisma.prisma) {
    const pool = new Pool({ connectionString });
    const adapter = new PrismaPg(pool);
    globalForPrisma.pool = pool;
    globalForPrisma.prisma = new PrismaClient({ adapter });
  }
  prisma = globalForPrisma.prisma;
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/auth-utils.ts".into(),
                content: r###"import bcrypt from 'bcryptjs';

export async function hashPassword(password: string): Promise<string> {
  return bcrypt.hash(password, 10);
}

export async function comparePassword(password: string, hash: string): Promise<boolean> {
  return bcrypt.compare(password, hash);
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/utils.ts".into(),
                content: r###"import { clsx, type ClassValue } from "clsx"
import { twMerge } from "tailwind-merge"

export function cn(...inputs: ClassValue[]) {
  return twMerge(clsx(inputs))
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/jwt.ts".into(),
                content: r###"import jwt from 'jsonwebtoken';

const JWT_SECRET = process.env.JWT_SECRET || 'super-secret-key-change-me';

interface TokenPayload {
  userId: string;
  email: string;
  role: string;
}

export function signToken(payload: TokenPayload, expiresIn: string = '7d'): string {
  return jwt.sign(payload, JWT_SECRET, { expiresIn: expiresIn as any });
}

export function verifyToken(token: string): TokenPayload | null {
  try {
    return jwt.verify(token, JWT_SECRET) as TokenPayload;
  } catch {
    return null;
  }
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/mailer.ts".into(),
                content: r###"import nodemailer from 'nodemailer';
import { pinoLogger } from './pino-logger';

const SMTP_HOST = process.env.SMTP_HOST || 'smtp.mailtrap.io';
const SMTP_PORT = parseInt(process.env.SMTP_PORT || '2525', 10);
const SMTP_USER = process.env.SMTP_USER || '';
const SMTP_PASS = process.env.SMTP_PASS || '';
const MAIL_FROM = process.env.MAIL_FROM || 'noreply@offpkg.com';

const transporter = nodemailer.createTransport({
  host: SMTP_HOST,
  port: SMTP_PORT,
  secure: SMTP_PORT === 465,
  auth: SMTP_USER && SMTP_PASS ? {
    user: SMTP_USER,
    pass: SMTP_PASS,
  } : undefined,
});

interface SendEmailParams {
  to: string;
  subject: string;
  html: string;
  text?: string;
}

export async function sendEmail({ to, subject, html, text }: SendEmailParams) {
  try {
    const info = await transporter.sendMail({
      from: MAIL_FROM,
      to,
      subject,
      text: text || html.replace(/<[^>]*>/g, ''),
      html,
    });

    pinoLogger.info(`Email sent successfully: ${info.messageId}`);
    return { success: true, messageId: info.messageId };
  } catch (error) {
    pinoLogger.error(error, 'Failed to send email');
    throw error;
  }
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/api-client.ts".into(),
                content: r###"import axios from 'axios';
import { logger } from './logger';

const apiClient = axios.create({
  baseURL: typeof window !== 'undefined' ? '' : process.env.NEXT_PUBLIC_API_URL || 'http://localhost:3000',
  headers: {
    'Content-Type': 'application/json',
  },
});

apiClient.interceptors.request.use(
  (config) => {
    logger.info(`Request: ${config.method?.toUpperCase()} ${config.url}`);
    return config;
  },
  (error) => {
    logger.error('Request Error', error);
    return Promise.reject(error);
  }
);

apiClient.interceptors.response.use(
  (response) => {
    logger.info(`Response: ${response.status} ${response.config.url}`);
    return response;
  },
  (error) => {
    logger.error('Response Error', error.response?.data || error.message);
    return Promise.reject(error);
  }
);

export default apiClient;
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/logger.ts".into(),
                content: r###"type LogLevel = "info" | "warn" | "error" | "debug"

const isDev = process.env.NODE_ENV !== "production"

const styles: Record<LogLevel, string> = {
  info: "color: #3b82f6; font-weight: 600;",
  warn: "color: #f59e0b; font-weight: 600;",
  error: "color: #ef4444; font-weight: 600;",
  debug: "color: #10b981; font-weight: 600;",
}

const formatMessage = (level: LogLevel, message: string) => {
  const prefix = `[APP] ${level.toUpperCase()}`
  const isBrowser = typeof window !== 'undefined'
  if (isBrowser) {
    return [`%c${prefix} %c${message}`, styles[level], "color: inherit; font-weight: normal;"]
  }
  // Server-side terminal color logging
  const colors: Record<LogLevel, string> = {
    info: "\x1b[36m", // Cyan
    warn: "\x1b[33m", // Yellow
    error: "\x1b[31m", // Red
    debug: "\x1b[32m", // Green
  }
  const reset = "\x1b[0m"
  return [`${colors[level]}${prefix}${reset} ${message}`]
}

function log(level: LogLevel, message: string, data?: unknown) {
  if (!isDev && level === "debug") return

  const formatted = formatMessage(level, message)

  if (data === undefined) {
    if (typeof window !== 'undefined') {
      const [prompt, style, reset] = formatted
      if (level === "error") console.error(prompt, style, reset)
      else if (level === "warn") console.warn(prompt, style, reset)
      else console.log(prompt, style, reset)
    } else {
      const [prompt] = formatted
      if (level === "error") console.error(prompt)
      else if (level === "warn") console.warn(prompt)
      else console.log(prompt)
    }
    return
  }

  // Handle data with grouping for a cleaner console
  if (typeof window !== 'undefined') {
    const [prompt, style, reset] = formatted
    console.groupCollapsed(prompt, style, reset)
    if (data instanceof Error) {
      console.error(data.message)
      if (data.stack) console.debug(data.stack)
    } else {
      console.dir(data)
    }
    console.groupEnd()
  } else {
    const [prompt] = formatted
    console.log(prompt, data)
  }
}

export const logger = {
  info: (msg: string, data?: unknown) => log("info", msg, data),
  warn: (msg: string, data?: unknown) => log("warn", msg, data),
  error: (msg: string, data?: unknown) => log("error", msg, data),
  debug: (msg: string, data?: unknown) => log("debug", msg, data),
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/api-logger.ts".into(),
                content: r###"import { NextRequest, NextResponse } from 'next/server';

export function withLogging(
  handler: (request: NextRequest, context?: any) => Promise<NextResponse>
) {
  return async (request: NextRequest, context?: any) => {
    const method = request.method;
    const path = request.nextUrl.pathname;
    
    // Output incoming request log in cyan
    console.log(`\x1b[36m→\x1b[0m ${method} ${path}`);
    
    const start = performance.now();
    try {
      const response = await handler(request, context);
      const duration = Math.round(performance.now() - start);
      
      const statusColor = response.status >= 400 ? '\x1b[31m' : '\x1b[32m';
      console.log(`\x1b[35m←\x1b[0m ${method} ${path} ${statusColor}${response.status}\x1b[0m ${duration}ms`);
      return response;
    } catch (error) {
      const duration = Math.round(performance.now() - start);
      console.log(`\x1b[31m←\x1b[0m ${method} ${path} \x1b[31m500\x1b[0m ${duration}ms`);
      throw error;
    }
  };
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/cors.ts".into(),
                content: r###"import { NextResponse } from 'next/server';

export function corsHeaders(origin = '*') {
  return {
    'Access-Control-Allow-Origin': origin,
    'Access-Control-Allow-Methods': 'GET, POST, PUT, DELETE, OPTIONS',
    'Access-Control-Allow-Headers': 'Content-Type, Authorization',
    'Access-Control-Max-Age': '86400', // 24 hours
  };
}

export function handleOptions() {
  return new NextResponse(null, {
    status: 204,
    headers: corsHeaders(),
  });
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/cron.ts".into(),
                content: r###"import cron from 'node-cron';
import { pinoLogger } from './pino-logger';

export function initCronJobs() {
  pinoLogger.info('Initializing background cron jobs...');

  // Example task: Runs every hour
  cron.schedule('0 * * * *', () => {
    pinoLogger.info('Cron Job [Hourly]: Running system health checks & log cleanup...');
  });

  // Example task: Runs every midnight (00:00)
  cron.schedule('0 0 * * *', () => {
    pinoLogger.info('Cron Job [Daily]: Running database optimization & backup hooks...');
  });

  pinoLogger.info('Background cron jobs registered successfully.');
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/redis.ts".into(),
                content: r###"import Redis from 'ioredis';
import { pinoLogger } from './pino-logger';

const REDIS_URL = process.env.REDIS_URL || 'redis://localhost:6379';

const globalForRedis = globalThis as unknown as {
  redis: Redis | undefined;
};

export let redis: Redis;

if (process.env.NODE_ENV === 'production') {
  redis = new Redis(REDIS_URL);
} else {
  if (!globalForRedis.redis) {
    globalForRedis.redis = new Redis(REDIS_URL, {
      maxRetriesPerRequest: 3,
    });
    pinoLogger.info('Initialized Redis connection (dev)');
  }
  redis = globalForRedis.redis;
}

redis.on('error', (err) => {
  pinoLogger.error(err, 'Redis connection error');
});
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/rate-limiter.ts".into(),
                content: r###"import { redis } from './redis';
import { pinoLogger } from './pino-logger';

interface RateLimitResult {
  success: boolean;
  limit: number;
  remaining: number;
  reset: number;
}

const memoryCache = new Map<string, { tokens: number; lastRefill: number }>();

export async function rateLimit(
  key: string,
  limit = 60,
  windowSeconds = 60
): Promise<RateLimitResult> {
  const now = Math.floor(Date.now() / 1000);
  const redisKey = `ratelimit:${key}`;

  try {
    const pipeline = redis.pipeline();
    pipeline.incr(redisKey);
    pipeline.ttl(redisKey);
    const results = await pipeline.exec();

    if (results) {
      const count = results[0][1] as number;
      const ttl = results[1][1] as number;

      if (count === 1) {
        await redis.expire(redisKey, windowSeconds);
      }

      const isAllowed = count <= limit;
      return {
        success: isAllowed,
        limit,
        remaining: Math.max(0, limit - count),
        reset: now + (ttl > 0 ? ttl : windowSeconds),
      };
    }
  } catch (error) {
    pinoLogger.warn(error, 'Redis rate limiting failed. Falling back to in-memory limiting.');
  }

  // Fallback: In-memory sliding window rate limiter
  const bucket = memoryCache.get(key) || { tokens: limit, lastRefill: now };
  const refillRate = limit / windowSeconds;
  const elapsed = now - bucket.lastRefill;
  
  const currentTokens = Math.min(limit, bucket.tokens + elapsed * refillRate);
  
  if (currentTokens >= 1) {
    memoryCache.set(key, {
      tokens: currentTokens - 1,
      lastRefill: now,
    });
    return {
      success: true,
      limit,
      remaining: Math.floor(currentTokens - 1),
      reset: now + windowSeconds,
    };
  } else {
    memoryCache.set(key, {
      tokens: currentTokens,
      lastRefill: now,
    });
    return {
      success: false,
      limit,
      remaining: 0,
      reset: now + windowSeconds,
    };
  }
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "prisma.config.ts".into(),
                content: r###"// This file was generated by Prisma, and assumes you have installed the following:
// npm install --save-dev prisma dotenv
import "dotenv/config";
import { defineConfig } from "prisma/config";

export default defineConfig({
  schema: "prisma/schema.prisma",
  migrations: {
    path: "prisma/migrations",
    seed: "bun prisma/seed.ts",
  },
  datasource: {
    url: process.env["DATABASE_URL"],
  },
});
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "store/useAppStore.ts".into(),
                content: r###"import { create } from 'zustand';
import { persist } from 'zustand/middleware';

interface UserInfo {
  id: string;
  name: string | null;
  email: string;
  role: 'ADMIN' | 'USER' | 'GUEST';
}

interface AppState {
  user: UserInfo | null;
  setUser: (user: UserInfo | null) => void;
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
      skipHydration: true, // Safe for Next.js SSR hydration
    }
  )
);
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "public/vercel.svg".into(),
                content: r###"<svg fill="none" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1155 1000"><path d="m577.3 0 577.4 1000H0z" fill="#fff"/></svg>"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "public/window.svg".into(),
                content: r###"<svg fill="none" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><path fill-rule="evenodd" clip-rule="evenodd" d="M1.5 2.5h13v10a1 1 0 0 1-1 1h-11a1 1 0 0 1-1-1zM0 1h16v11.5a2.5 2.5 0 0 1-2.5 2.5h-11A2.5 2.5 0 0 1 0 12.5zm3.75 4.5a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5M7 4.75a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0m1.75.75a.75.75 0 1 0 0-1.5.75.75 0 0 0 0 1.5" fill="#666"/></svg>"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "public/next.svg".into(),
                content: r###"<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 394 80"><path fill="#000" d="M262 0h68.5v12.7h-27.2v66.6h-13.6V12.7H262V0ZM149 0v12.7H94v20.4h44.3v12.6H94v21h55v12.6H80.5V0h68.7zm34.3 0h-17.8l63.8 79.4h17.9l-32-39.7 32-39.6h-17.9l-23 28.6-23-28.6zm18.3 56.7-9-11-27.1 33.7h17.8l18.3-22.7z"/><path fill="#000" d="M81 79.3 17 0H0v79.3h13.6V17l50.2 62.3H81Zm252.6-.4c-1 0-1.8-.4-2.5-1s-1.1-1.6-1.1-2.6.3-1.8 1-2.5 1.6-1 2.6-1 1.8.3 2.5 1a3.4 3.4 0 0 1 .6 4.3 3.7 3.7 0 0 1-3 1.8zm23.2-33.5h6v23.3c0 2.1-.4 4-1.3 5.5a9.1 9.1 0 0 1-3.8 3.5c-1.6.8-3.5 1.3-5.7 1.3-2 0-3.7-.4-5.3-1s-2.8-1.8-3.7-3.2c-.9-1.3-1.4-3-1.4-5h6c.1.8.3 1.6.7 2.2s1 1.2 1.6 1.5c.7.4 1.5.5 2.4.5 1 0 1.8-.2 2.4-.6a4 4 0 0 0 1.6-1.8c.3-.8.5-1.8.5-3V45.5zm30.9 9.1a4.4 4.4 0 0 0-2-3.3 7.5 7.5 0 0 0-4.3-1.1c-1.3 0-2.4.2-3.3.5-.9.4-1.6 1-2 1.6a3.5 3.5 0 0 0-.3 4c.3.5.7.9 1.3 1.2l1.8 1 2 .5 3.2.8c1.3.3 2.5.7 3.7 1.2a13 13 0 0 1 3.2 1.8 8.1 8.1 0 0 1 3 6.5c0 2-.5 3.7-1.5 5.1a10 10 0 0 1-4.4 3.5c-1.8.8-4.1 1.2-6.8 1.2-2.6 0-4.9-.4-6.8-1.2-2-.8-3.4-2-4.5-3.5a10 10 0 0 1-1.7-5.6h6a5 5 0 0 0 3.5 4.6c1 .4 2.2.6 3.4.6 1.3 0 2.5-.2 3.5-.6 1-.4 1.8-1 2.4-1.7a4 4 0 0 0 .8-2.4c0-.9-.2-1.6-.7-2.2a11 11 0 0 0-2.1-1.4l-3.2-1-3.8-1c-2.8-.7-5-1.7-6.6-3.2a7.2 7.2 0 0 1-2.4-5.7 8 8 0 0 1 1.7-5 10 10 0 0 1 4.3-3.5c2-.8 4-1.2 6.4-1.2 2.3 0 4.4.4 6.2 1.2 1.8.8 3.2 2 4.3 3.4 1 1.4 1.5 3 1.5 5h-5.8z"/></svg>"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "public/file.svg".into(),
                content: r###"<svg fill="none" viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg"><path d="M14.5 13.5V5.41a1 1 0 0 0-.3-.7L9.8.29A1 1 0 0 0 9.08 0H1.5v13.5A2.5 2.5 0 0 0 4 16h8a2.5 2.5 0 0 0 2.5-2.5m-1.5 0v-7H8v-5H3v12a1 1 0 0 0 1 1h8a1 1 0 0 0 1-1M9.5 5V2.12L12.38 5zM5.13 5h-.62v1.25h2.12V5zm-.62 3h7.12v1.25H4.5zm.62 3h-.62v1.25h7.12V11z" clip-rule="evenodd" fill="#666" fill-rule="evenodd"/></svg>"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "public/globe.svg".into(),
                content: r###"<svg fill="none" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 16 16"><g clip-path="url(#a)"><path fill-rule="evenodd" clip-rule="evenodd" d="M10.27 14.1a6.5 6.5 0 0 0 3.67-3.45q-1.24.21-2.7.34-.31 1.83-.97 3.1M8 16A8 8 0 1 0 8 0a8 8 0 0 0 0 16m.48-1.52a7 7 0 0 1-.96 0H7.5a4 4 0 0 1-.84-1.32q-.38-.89-.63-2.08a40 40 0 0 0 3.92 0q-.25 1.2-.63 2.08a4 4 0 0 1-.84 1.31zm2.94-4.76q1.66-.15 2.95-.43a7 7 0 0 0 0-2.58q-1.3-.27-2.95-.43a18 18 0 0 1 0 3.44m-1.27-3.54a17 17 0 0 1 0 3.64 39 39 0 0 1-4.3 0 17 17 0 0 1 0-3.64 39 39 0 0 1 4.3 0m1.1-1.17q1.45.13 2.69.34a6.5 6.5 0 0 0-3.67-3.44q.65 1.26.98 3.1M8.48 1.5l.01.02q.41.37.84 1.31.38.89.63 2.08a40 40 0 0 0-3.92 0q.25-1.2.63-2.08a4 4 0 0 1 .85-1.32 7 7 0 0 1 .96 0m-2.75.4a6.5 6.5 0 0 0-3.67 3.44 29 29 0 0 1 2.7-.34q.31-1.83.97-3.1M4.58 6.28q-1.66.16-2.95.43a7 7 0 0 0 0 2.58q1.3.27 2.95.43a18 18 0 0 1 0-3.44m.17 4.71q-1.45-.12-2.69-.34a6.5 6.5 0 0 0 3.67 3.44q-.65-1.27-.98-3.1" fill="#666"/></g><defs><clipPath id="a"><path fill="#fff" d="M0 0h16v16H0z"/></clipPath></defs></svg>"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "hooks/useAuth.ts".into(),
                content: r###"import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import apiClient from '@/lib/api-client';
import { useAppStore } from '@/store/useAppStore';
import { LoginFormValues, RegisterFormValues, User } from '@/types/schema';
import { logger } from '@/lib/logger';

export const useAuth = () => {
  const queryClient = useQueryClient();
  const { setUser, user } = useAppStore();

  const meQuery = useQuery({
    queryKey: ['auth-me'],
    queryFn: async () => {
      try {
        const { data } = await apiClient.get<{ success: boolean; data: User }>('/api/auth/me');
        if (data.success) {
          setUser(data.data);
          return data.data;
        }
        return null;
      } catch (error) {
        logger.debug('No active session found');
        setUser(null);
        return null;
      }
    },
    retry: false,
  });

  const loginMutation = useMutation({
    mutationFn: async (credentials: LoginFormValues) => {
      const { data } = await apiClient.post<{ success: boolean; data: User; token: string }>('/api/auth/login', credentials);
      if (typeof window !== 'undefined' && data.token) {
        localStorage.setItem('auth-token', data.token);
      }
      return data;
    },
    onSuccess: (data) => {
      setUser(data.data);
      queryClient.invalidateQueries({ queryKey: ['auth-me'] });
    },
  });

  const registerMutation = useMutation({
    mutationFn: async (userData: RegisterFormValues) => {
      const { data } = await apiClient.post<{ success: boolean; data: User; token: string }>('/api/auth/register', userData);
      if (typeof window !== 'undefined' && data.token) {
        localStorage.setItem('auth-token', data.token);
      }
      return data;
    },
    onSuccess: (data) => {
      setUser(data.data);
      queryClient.invalidateQueries({ queryKey: ['auth-me'] });
    },
  });

  const logoutMutation = useMutation({
    mutationFn: async () => {
      await apiClient.post('/api/auth/me');
      if (typeof window !== 'undefined') {
        localStorage.removeItem('auth-token');
      }
    },
    onSuccess: () => {
      setUser(null);
      queryClient.setQueryData(['auth-me'], null);
      queryClient.invalidateQueries({ queryKey: ['auth-me'] });
    },
  });

  return {
    user,
    isLoading: meQuery.isLoading,
    login: loginMutation.mutateAsync,
    isLoggingIn: loginMutation.isPending,
    register: registerMutation.mutateAsync,
    isRegistering: registerMutation.isPending,
    logout: logoutMutation.mutateAsync,
    isLoggingOut: logoutMutation.isPending,
  };
};
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "hooks/usePosts.ts".into(),
                content: r###"import { useQuery, useMutation, useQueryClient } from '@tanstack/react-query';
import apiClient from '@/lib/api-client';
import { Post, CreatePostValues } from '@/types/schema';

export const usePosts = () => {
  const queryClient = useQueryClient();

  const postsQuery = useQuery({
    queryKey: ['posts'],
    queryFn: async () => {
      const { data } = await apiClient.get<{ success: boolean; data: (Post & { author: { name: string | null; email: string } })[] }>('/api/posts');
      return data.data;
    },
  });

  const createPostMutation = useMutation({
    mutationFn: async (newPost: CreatePostValues & { token: string }) => {
      const { token, ...postData } = newPost;
      const { data } = await apiClient.post<{ success: boolean; data: Post }>('/api/posts', postData, {
        headers: {
          Authorization: `Bearer ${token}`,
        },
      });
      return data.data;
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['posts'] });
    },
  });

  return {
    posts: postsQuery.data || [],
    isLoading: postsQuery.isLoading,
    error: postsQuery.error,
    createPost: createPostMutation.mutateAsync,
    isCreating: createPostMutation.isPending,
  };
};
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "AGENTS.md".into(),
                content: r###"<!-- BEGIN:nextjs-agent-rules -->
# This is NOT the Next.js you know

This version has breaking changes — APIs, conventions, and file structure may all differ from your training data. Read the relevant guide in `node_modules/next/dist/docs/` before writing any code. Heed deprecation notices.
<!-- END:nextjs-agent-rules -->
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "docs/LOGGING.md".into(),
                content: r###"# Structured Logging with Pino 🪵

This backend template integrates **Pino**, a high-performance, structured logging framework.

## 🚀 Why Pino?

- **Speed**: Pino is one of the fastest loggers in the Node.js ecosystem, with negligible latency impact.
- **Structured JSON**: Logs are printed as JSON in production, enabling easy ingestion by log managers (like Datadog, Logtail, Elasticsearch, or AWS CloudWatch).
- **Colorized Dev Mode**: Uses `pino-pretty` to print readable logs during local development.

---

## 🛠️ Usage Guide

Import the configured `pinoLogger` from `@/lib/pino-logger`:

```typescript
import { pinoLogger } from '@/lib/pino-logger';

// Standard logs
pinoLogger.info('App successfully initialized');
pinoLogger.warn('Rate limit threshold reached for IP: 127.0.0.1');

// Logs with metadata objects
pinoLogger.info({ userId: '123', action: 'CREATE_POST' }, 'User created a new post');

// Error logging
try {
  throw new Error('Database connection failed');
} catch (error) {
  pinoLogger.error(error, 'An unexpected error occurred during seeding');
}
```

---

## 🎛️ Configurations

Adjust your log settings in `.env`:
```env
# Supported: fatal, error, warn, info, debug, trace
LOG_LEVEL="info"
```
During production builds (`NODE_ENV=production`), `pino-pretty` is automatically bypassed to output raw JSON strings to `stdout` for optimal performance.
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "docs/CRON.md".into(),
                content: r###"# Background Task Scheduling with Node-Cron ⏰

This backend template supports scheduled cron tasks using **Node-Cron**, allowing you to execute database operations, backups, health checks, or log cleanup on specific time intervals.

## 🛠️ Usage Guide

### 1. Registering Tasks
Tasks are registered and managed inside `@/lib/cron.ts`. It utilizes standard 5-field cron syntax:
`* * * * *` (minute hour day-of-month month day-of-week).

```typescript
import cron from 'node-cron';
import { pinoLogger } from './pino-logger';

export function initCronJobs() {
  pinoLogger.info('Initializing background cron jobs...');

  // E.g. Run every hour
  cron.schedule('0 * * * *', () => {
    pinoLogger.info('Cron task: Running hourly database cleanups...');
  });
}
```

---

### 2. Startup Hooks in Next.js
To ensure background task runners register automatically on Next.js server boot, you should initialize them inside `instrumentation.ts` in the root of the project. Next.js 16 calls the `register()` function once when the runtime starts.

Create an `instrumentation.ts` file in your root folder:

```typescript
export async function register() {
  // Only register background schedulers on the server side
  if (process.env.NEXT_RUNTIME === 'nodejs') {
    const { initCronJobs } = await import('./lib/cron');
    initCronJobs();
  }
}
```

Make sure to enable instrumentation in your `next.config.ts`:
```typescript
import type { NextConfig } from 'next';

const nextConfig: NextConfig = {
  experimental: {
    instrumentationHook: true,
  },
};

export default nextConfig;
```
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "docs/STORAGE.md".into(),
                content: r###"# Object File Storage with S3 & Cloudflare R2 🪣

This backend template utilizes the official **AWS SDK v3 S3 client** (`@aws-sdk/client-s3` and `@aws-sdk/s3-request-presigner`) to connect to object storage buckets. This architecture is fully compatible with standard AWS S3, Cloudflare R2, MinIO, or DigitalOcean Spaces.

## 🛠️ Usage Guide

### 1. Direct Server-Side Upload
Upload files directly from your server actions or API routes:

```typescript
import { uploadToS3 } from '@/lib/s3';

const fileBuffer = Buffer.from('Hello File Storage');

await uploadToS3({
  key: 'uploads/docs/hello-world.txt',
  body: fileBuffer,
  contentType: 'text/plain',
});
```

---

### 2. Client-Side Uploads via Presigned URLs (Best Practice)
For optimal speed, large file uploads should bypass the Next.js server limits and be uploaded directly from the browser to the bucket using a secure, pre-signed upload URL.

#### Step A: Generate URL in Route Handler / Server Action
```typescript
import { getPresignedUploadUrl } from '@/lib/s3';

// Generate a URL valid for 10 minutes (600 seconds)
const uploadUrl = await getPresignedUploadUrl('user-avatars/user_1.png', 'image/png', 600);
```

#### Step B: Send file directly from Client Component
```typescript
const file = event.target.files[0];

await fetch(uploadUrl, {
  method: 'PUT',
  body: file,
  headers: {
    'Content-Type': file.type,
  },
});
```

---

## 🎛️ Configurations

Adjust the environment variables in `.env`:
```env
S3_REGION="us-east-1"
S3_BUCKET="my-app-bucket"
S3_ACCESS_KEY_ID="your-access-key-id"
S3_SECRET_ACCESS_KEY="your-secret-access-key"

# Optional: Add custom endpoints for MinIO / Cloudflare R2 / DigitalOcean Spaces
S3_ENDPOINT="https://xxxxxx.r2.cloudflarestorage.com"
```
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "docs/MAILER.md".into(),
                content: r###"# SMTP Mailer with Nodemailer ✉️

This backend starter templates uses **Nodemailer** to dispatch transactional emails (e.g., account verifications, password resets, welcome emails).

## 🛠️ Usage Guide

Import the `sendEmail` helper function from `@/lib/mailer`:

```typescript
import { sendEmail } from '@/lib/mailer';

try {
  await sendEmail({
    to: 'user@example.com',
    subject: 'Welcome to Offpkg Next.js Template!',
    html: '<h1>Account Created</h1><p>Your fullstack template account has been successfully set up.</p>',
    text: 'Account Created. Your fullstack template account has been successfully set up.', // Optional text-only fallback
  });
} catch (error) {
  // Handle mail dispatch error
}
```

---

## 🎛️ Configurations

Update mail settings in `.env` to connect with your SMTP provider (e.g., Mailtrap, SendGrid, Amazon SES, Resend):

```env
# SMTP connection options
SMTP_HOST="smtp.mailtrap.io"
SMTP_PORT=2525
SMTP_USER="your-smtp-username"
SMTP_PASS="your-smtp-password"

# Sender settings
MAIL_FROM="noreply@offpkg.com"
```
In case `SMTP_USER` and `SMTP_PASS` variables are omitted, Nodemailer will attempt to send emails over a local direct SMTP connection.
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "docs/PRISMA.md".into(),
                content: r###"# Prisma 7 Database Documentation 🗄️

This template uses **Prisma v7**, which introduces several architectural changes, specifically separation of concerns between CLI operations and runtime database connections.

## ⚠️ Key Differences in Prisma 7

### 1. No Connection URL in `schema.prisma`
In previous Prisma versions, you defined `url = env("DATABASE_URL")` directly inside the `datasource` block of `schema.prisma`. In Prisma 7, this has been deprecated.
The datasource block is now simplified:
```prisma
datasource db {
  provider = "postgresql"
}
```

### 2. Connection URL inside `prisma.config.ts`
All CLI commands (like migrations and DB pushes) pull the database configuration from `prisma.config.ts` in the root of the project:
```typescript
import "dotenv/config";
import { defineConfig, env } from "prisma/config";

export default defineConfig({
  schema: "prisma/schema.prisma",
  migrations: {
    path: "prisma/migrations",
  },
  datasource: {
    url: env("DATABASE_URL"),
  },
});
```

### 3. Native JS Driver Adapters at Runtime
Prisma 7 removes the bundled Rust query engine binary by default to keep package size minimal and make it serverless/edge-ready. 
Instead, it requires passing a **Driver Adapter** (e.g., node-postgres, serverless pg, or neon) when instantiating the client:
```typescript
import { PrismaClient } from './generated/prisma/client';
import { PrismaPg } from '@prisma/adapter-pg';
import pg from 'pg';

const pool = new pg.Pool({ connectionString: process.env.DATABASE_URL });
const adapter = new PrismaPg(pool);
const prisma = new PrismaClient({ adapter });
```

---

## 🛠️ PostgreSQL & Database Operations

This template provides a Docker-based PostgreSQL setup to get you up and running instantly.

### 1. Start PostgreSQL Container
Spin up a local PostgreSQL 16 server in the background:
```bash
docker-compose up -d
```
This starts a Postgres instance mapped to port `5432` with username `postgres`, password `postgres`, and database `offpkg_db`.

### 2. Run Database Migrations
Create database tables and schemas:
```bash
bun run db:migrate
```

### 3. Generate Type-Safe Client
Re-generate the custom Prisma Client under `lib/generated/prisma`:
```bash
bun run db:generate
```

### 4. Seed Database
Populate database with mock users and posts (with hashed passwords):
```bash
bun run db:seed
```

### 5. Open Prisma Studio
Explore and edit database tables in an interactive browser GUI:
```bash
bun run db:studio
```

"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "docs/ARCHITECTURE.md".into(),
                content: r###"# Template Architecture & Layout 🏗️

This project template is designed for building highly performant full-stack web applications with Next.js 16, TypeScript, Bun, and Prisma.

## 📂 Folder Structure

```text
├── app/                  # Next.js App Router routes & layouts
│   ├── actions/          # Next.js Server Actions (if any)
│   ├── api/              # API endpoints (auth, posts, seed)
│   ├── favicon.ico       # Favicon asset
│   ├── globals.css       # Global styles (Tailwind CSS v4 + tw-animate-css)
│   ├── layout.tsx        # Main application layout wrapping Providers
│   └── page.tsx          # Homepage UI feed and stack sandbox
├── components/           # Reusable UI & helper components
│   ├── ui/               # Lower-level Shadcn UI primitives (button, card, input)
│   ├── ThemeToggle.tsx   # SSR-safe client theme switcher button
│   ├── providers.tsx     # Consolidated context wrapper (Query, next-themes, Zustand)
│   └── Navbar.tsx        # Top navbar containing modal authorization
├── docs/                 # Architectural & configuration guides
├── hooks/                # React Query data fetching hooks (useAuth, usePosts)
├── lib/                  # Server-side & client-side utilities
│   ├── generated/prisma  # Custom target path for type-safe Prisma client
│   ├── api-client.ts     # Request interceptor Axios client
│   ├── auth-utils.ts     # Bcryptjs password hashing helpers
│   ├── jwt.ts            # Sign & verify tokens
│   ├── logger.ts         # Cross-environment terminal/console logger
│   ├── prisma.ts         # Singleton database connection client
│   └── utils.ts          # Tailwind cn utility function
├── prisma/               # Database schemas & seeds
├── store/                # Hydration-safe Zustand state stores
└── types/                # Zod schemas & shared TypeScript types
```

---

## 🔒 Authentication Flow

Authentication is handled via state-of-the-art secure JWTs.

1. **Registration/Login**: User posts credentials to `/api/auth/register` or `/api/auth/login`.
2. **Password Validation**: Passwords are hashed and checked using `bcryptjs` on the server.
3. **Session Issuing**: A JWT is generated containing the user info (`userId`, `email`, `role`) and sent back.
4. **Cookie & Storage Sync**:
   - The server sets the JWT as an `httpOnly` secure `sameSite=strict` cookie.
   - The client stores the JWT token in `localStorage` as fallback for authorization headers.
5. **State Sync**: The client hooks `useAuth` query `/api/auth/me` to fetch current user data and caches it into the **Zustand global store** `useAppStore` for instant page shell rendering.
6. **Log Out**: POSTing to `/api/auth/me` expires the browser cookie and wipes client memory stores.

---

## ⚛️ State Management & Data Fetching

### 1. Hydration-Safe Zustand
Next.js server-side renders (SSR) pages first. If Zustand loads persisted local storage state during SSR, a client-server mismatch warning triggers.
This template uses `skipHydration: true` on store definition, and manually triggers rehydration inside a client-side `useEffect` in the consolidated provider layout wrapper:
```typescript
useEffect(() => {
  useAppStore.persist.rehydrate();
}, []);
```

### 2. React Query Integration
Data fetching is managed by TanStack Query. It handles queries, mutations, cache invalidation, loading states, and API errors smoothly. The custom hooks (`useAuth` and `usePosts`) abstract API routing from the component layer.
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "docs/REDIS.md".into(),
                content: r###"# Redis Cache & Rate Limiting 🚀

This template integrates **Redis** (via the `ioredis` package) to support high-performance caching, key-value storage, and API rate limiting.

## 🛠️ Usage Guide

### 1. Direct Redis Access
Import the shared singleton connection from `@/lib/redis`:

```typescript
import { redis } from '@/lib/redis';

// Set values with custom Expiration (TTL)
await redis.set('cache:users:list', JSON.stringify(users), 'EX', 3600); // 1 hour TTL

// Get values
const cachedData = await redis.get('cache:users:list');
if (cachedData) {
  const users = JSON.parse(cachedData);
}

// Delete key
await redis.del('cache:users:list');
```

---

## 🛡️ API Rate Limiting

The template provides a custom sliding-window token bucket rate limiter in `@/lib/rate-limiter`. It includes an automatic **in-memory memory cache fallback** in case Redis is not active, making it fully functional in local dev environments even without Redis.

### Usage in Route Handlers
```typescript
import { NextRequest, NextResponse } from 'next/server';
import { rateLimit } from '@/lib/rate-limiter';

export async function GET(request: NextRequest) {
  const ip = request.headers.get('x-forwarded-for') || '127.0.0.1';
  
  // Limit to 30 requests per minute per IP
  const { success, limit, remaining, reset } = await rateLimit(`ip:${ip}`, 30, 60);

  if (!success) {
    return NextResponse.json(
      { error: 'Too Many Requests' },
      { 
        status: 429,
        headers: {
          'X-RateLimit-Limit': limit.toString(),
          'X-RateLimit-Remaining': remaining.toString(),
          'X-RateLimit-Reset': reset.toString(),
        }
      }
    );
  }

  // Handle standard route business logic...
}
```

---

## 🎛️ Configurations

Update connection settings in `.env`:
```env
REDIS_URL="redis://localhost:6379"
```
To run a local Redis instance using Docker, add the Redis service to your `docker-compose.yml`.
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "instrumentation.ts".into(),
                content: r###"export async function register() {
  if (process.env.NEXT_RUNTIME === 'nodejs') {
    const { initCronJobs } = await import('./lib/cron');
    initCronJobs();
  }
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "prisma/schema.prisma".into(),
                content: r###"// This is your Prisma schema file,
// learn more about it in the docs: https://pris.ly/d/prisma-schema

generator client {
  provider = "prisma-client"
  output   = "../lib/generated/prisma"
}

datasource db {
  provider = "postgresql"
}

enum Role {
  ADMIN
  USER
  GUEST
}

model User {
  id        String   @id @default(uuid())
  email     String   @unique
  name      String?
  password  String   // Hashed password
  role      Role     @default(USER)
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
  posts     Post[]
}

model Post {
  id        String   @id @default(uuid())
  title     String
  content   String?
  published Boolean  @default(false)
  views     Int      @default(0)
  createdAt DateTime @default(now())
  updatedAt DateTime @updatedAt
  authorId  String
  author    User     @relation(fields: [authorId], references: [id], onDelete: Cascade)
}
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "prisma/seed.ts".into(),
                content: r###"import { PrismaClient } from '../lib/generated/prisma/client';
import { PrismaPg } from '@prisma/adapter-pg';
import pg from 'pg';
import bcrypt from 'bcryptjs';

const { Pool } = pg;
const connectionString = process.env.DATABASE_URL;

if (!connectionString) {
  console.error("DATABASE_URL is required to seed the database.");
  process.exit(1);
}

const pool = new Pool({ connectionString });
const adapter = new PrismaPg(pool);
const prisma = new PrismaClient({ adapter });

async function main() {
  console.log('🌱 Seeding database...');

  // Clean the database
  await prisma.post.deleteMany();
  await prisma.user.deleteMany();

  // Create Users
  const adminPassword = await bcrypt.hash('admin123', 10);
  const userPassword = await bcrypt.hash('user123', 10);

  const admin = await prisma.user.create({
    data: {
      email: 'admin@offpkg.com',
      name: 'Admin User',
      password: adminPassword,
      role: 'ADMIN',
    },
  });

  const user1 = await prisma.user.create({
    data: {
      email: 'user1@offpkg.com',
      name: 'Aswin Dev',
      password: userPassword,
      role: 'USER',
    },
  });

  const user2 = await prisma.user.create({
    data: {
      email: 'user2@offpkg.com',
      name: 'Jane Smith',
      password: userPassword,
      role: 'USER',
    },
  });

  // Create Posts
  await prisma.post.createMany({
    data: [
      {
        title: 'Getting Started with Next.js 16 and Prisma 7',
        content: 'Next.js 16 and Prisma 7 provide an incredibly powerful combo for full-stack React applications. By combining Next.js Server Actions with Prisma driver adapters, you can build blazing fast, edge-ready applications.',
        published: true,
        authorId: user1.id,
        views: 120,
      },
      {
        title: 'Building Beautiful Interfaces with Tailwind CSS v4',
        content: 'Tailwind CSS v4 introduces a streamlined engine, CSS-first configuration, and native cascading layers. It makes managing design systems a breeze without the bloat of traditional CSS setups.',
        published: true,
        authorId: user1.id,
        views: 340,
      },
      {
        title: 'The Future of State Management with Zustand',
        content: 'Zustand is a small, fast, and scalable bear-bones state-management solution. It has a comfy API based on hooks, is not opinionated, and doesn\'t wrap your app in providers.',
        published: true,
        authorId: user2.id,
        views: 95,
      },
      {
        title: 'Draft post',
        content: 'This is a draft post that is not published yet.',
        published: false,
        authorId: admin.id,
      },
    ],
  });

  console.log('✅ Database seeded successfully!');
}

main()
  .catch((e) => {
    console.error('❌ Error seeding database:', e);
    process.exit(1);
  })
  .finally(async () => {
    await pool.end();
  });
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "types/schema.ts".into(),
                content: r###"import { z } from 'zod';

export const RoleEnum = z.enum(['ADMIN', 'USER', 'GUEST']);
export type Role = z.infer<typeof RoleEnum>;

export const UserSchema = z.object({
  id: z.string().uuid(),
  email: z.string().email('Invalid email address'),
  name: z.string().min(2, 'Name must be at least 2 characters').nullable(),
  role: RoleEnum,
  createdAt: z.string().or(z.date()),
  updatedAt: z.string().or(z.date()),
});

export type User = z.infer<typeof UserSchema>;

export const PostSchema = z.object({
  id: z.string().uuid(),
  title: z.string().min(3, 'Title must be at least 3 characters'),
  content: z.string().min(5, 'Content must be at least 5 characters').nullable(),
  published: z.boolean().default(false),
  views: z.number().int().nonnegative().default(0),
  createdAt: z.string().or(z.date()),
  updatedAt: z.string().or(z.date()),
  authorId: z.string().uuid(),
});

export type Post = z.infer<typeof PostSchema>;

export const LoginFormSchema = z.object({
  email: z.string().email('Invalid email address'),
  password: z.string().min(6, 'Password must be at least 6 characters'),
});

export type LoginFormValues = z.infer<typeof LoginFormSchema>;

export const RegisterFormSchema = z.object({
  email: z.string().email('Invalid email address'),
  name: z.string().min(2, 'Name must be at least 2 characters'),
  password: z.string().min(6, 'Password must be at least 6 characters'),
});

export type RegisterFormValues = z.infer<typeof RegisterFormSchema>;

export const CreatePostSchema = z.object({
  title: z.string().min(3, 'Title must be at least 3 characters'),
  content: z.string().min(5, 'Content must be at least 5 characters'),
  published: z.boolean().default(false),
});

export type CreatePostValues = z.infer<typeof CreatePostSchema>;
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "docker-compose.yml".into(),
                content: r###"version: '3.8'

services:
  postgres:
    image: postgres:16-alpine
    container_name: offpkg-postgres
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: offpkg_db
    ports:
      - '5434:5432'
    volumes:
      - postgres_data:/var/lib/postgresql/data
    restart: always

volumes:
  postgres_data:
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "postcss.config.mjs".into(),
                content: r###"const config = {
  plugins: {
    "@tailwindcss/postcss": {},
  },
};

export default config;
"###.into(),
                binary_content: None,
            },
            StackFile {
                path: "CLAUDE.md".into(),
                content: r###"@AGENTS.md
"###.into(),
                binary_content: None,
            },
        ],
    }
}
