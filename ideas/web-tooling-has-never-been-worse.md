---
title: Web tooling has never been worse
description: We live in an age of first-class tooling, so why can't I just build things?
author: Kayla Washburn
status: draft
accent_color: "#f35"
cover:
  avif: https://cdn.mckayla.cloud/-/6c613c6ccx5ee1/IMG_6006.avif
  default: https://cdn.mckayla.cloud/-/6c613c6ccx5ee1/IMG_6006.webp
---

- Publishing TypeScript on npm
- Running tests in TypeScript (slow start up time, issues with type checking)
- ESModules vs CommonJS
- Now that file extensions are required, do my imports need to use js or ts extensions?
  - TSC won't help you.
- Documentation comments (great in the editor, but, where else?)
- Integration testing (making sure it works vanilla, or with Webpack, ESBuild, Vite, Next.js, Storybook, etc.)
  - Compounds the ESModules vs CommonJS issue
  - Being able to just `import {} from "mylib";`
