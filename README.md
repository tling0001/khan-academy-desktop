# Khan Academy Desktop

Cross-platform desktop app built with Tauri v2, React, and TypeScript. Targets Windows, macOS, and Linux.

The supplied Android reference is Khan Academy 8.5.1. Its main interface is a React Native bundle at app/src/main/assets/index.bundle; the native MainActivity is the React Native host. The archive also contains the localized learn-menu seed data, fonts, icons, and Android resources.

The initial implementation includes responsive Home, Courses, Explore, Assignments, and Profile screens; course search and filtering; course details; and cross-platform GitHub Actions builds. Account-backed progress and assignment state are not fabricated; course launch currently opens the canonical Khan Academy course page.

Development: npm install && npm run tauri dev. Builds run in GitHub Actions, not locally.