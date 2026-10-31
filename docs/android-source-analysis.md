# Android reference: Khan Academy 8.5.1

- Application ID / namespace: org.khanacademy.android
- Version name: 8.5.1; version code: 104846
- Main activity: org.khanacademy.android.ui.library.MainActivity
- Main UI: app/src/main/assets/index.bundle (React Native bundle, approximately 6.2 MB)
- Seed data: app/src/main/assets/data/initial_learn_menus_en_json (gzip-compressed JSON) and localized variants
- Design assets: app/src/main/assets/fonts, app/src/main/res/drawable*, app/src/main/res/mipmap*, app/src/main/res/values*

The native activity is a React Native host. It creates the root frame layout and handles Android intents/deep links. The bundle contains app navigation, screen fixtures, localized strings, and GraphQL operation documents. Feature names visible in the bundle include Home, Explore, Courses, course/unit/topic navigation, Assignments, Search, Profile, Bookmarks, learning paths, and progress.

The desktop UI currently implements browse/navigation surfaces and uses canonical course URLs. It does not claim to implement authenticated progress, assignments, mastery, or the full interactive lesson player. Those require a deliberate port of the app's authenticated GraphQL flows and Android native services.
