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
RUN pnpm --filter @app/landing build

# Production
FROM node:22-alpine AS production
WORKDIR /app
COPY --from=build /app/apps/landing/.output ./.output
ENV HOST=0.0.0.0
ENV PORT=3000
EXPOSE 3000
CMD ["node", ".output/server/index.mjs"]
