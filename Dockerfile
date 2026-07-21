FROM node:22-alpine AS base
RUN corepack enable && corepack prepare pnpm@latest --activate
WORKDIR /app

# Install dependencies
FROM base AS deps
COPY pnpm-lock.yaml pnpm-workspace.yaml package.json .npmrc ./
COPY apps/landing/package.json ./apps/landing/
RUN pnpm install --frozen-lockfile --filter @app/landing...

# Build
FROM base AS build
COPY --from=deps /app/node_modules ./node_modules
COPY --from=deps /app/apps/landing/node_modules ./apps/landing/node_modules
COPY pnpm-lock.yaml pnpm-workspace.yaml package.json .npmrc ./
COPY apps/landing/ ./apps/landing/
# vite build only: the build script also runs tsc, which needs devDependencies
# that are pruned in the production stage anyway. Types are checked in CI.
RUN pnpm --filter @app/landing exec vite build

# Production — the server bundle keeps react and tanstack as external imports,
# so production node_modules are required (srvx ships among them).
FROM base AS production
ENV NODE_ENV=production
ENV PORT=3000
COPY pnpm-lock.yaml pnpm-workspace.yaml package.json .npmrc ./
COPY apps/landing/package.json ./apps/landing/
RUN pnpm install --frozen-lockfile --prod --filter @app/landing...
COPY --from=build /app/apps/landing/dist ./apps/landing/dist
WORKDIR /app/apps/landing
EXPOSE 3000
# --dir is required: srvx otherwise resolves --static relative to the entry dir.
CMD ["pnpm", "exec", "srvx", "serve", "--prod", "--port", "3000", "--dir", ".", "--entry", "dist/server/server.js", "--static", "dist/client"]
