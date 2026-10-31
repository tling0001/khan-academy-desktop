# Android reference: Khan Academy 8.5.1

## Source inventory

- Application ID / namespace: `org.khanacademy.android`
- Version name: `8.5.1`; version code: `104846`
- Main activity: `org.khanacademy.android.ui.library.MainActivity`
- Main UI: `app/src/main/assets/index.bundle` — a 6,175,052-byte Hermes bytecode bundle (Hermes bytecode version 96)
- Learning menu: `app/src/main/assets/data/initial_learn_menus_en_json` — gzip-compressed JSON, with localized/curriculum variants
- Launcher artwork: `app/src/main/res/mipmap-xxxhdpi/app_icon.png` (192 × 192)
- Other design resources: `app/src/main/assets/fonts`, `app/src/main/res/drawable*`, `app/src/main/res/mipmap*`, and `app/src/main/res/values*`

The Android activity is a React Native host. The APK archive contains the compiled Hermes bundle, not the original editable React Native component source. This means component trees and screen styles cannot simply be copied as source files; the bundle and Android resources have to be inspected and behavior reconstructed in the desktop React application.

## Data recovered from the Android package

The English learn-menu seed file contains 33 curriculum/localization entries. The default curriculum entry contains 17 menu groups and 174 course/topic links. The desktop app imports this catalog from `src/data/android-learn-menu.json` and preserves the 157 Android `courseId` values in `src/data/android-course-ids.json`; this replaces the earlier hand-maintained 12-item sample list. Course titles, relative paths, and available IDs come from the Android package.

The Android launcher image uses the Khan Academy green `#14BF96` and the white learner/leaf mark inside a green hexagon. The exact 192 × 192 Android launcher PNG is committed at `public/android-launcher.png` for the app chrome and `src-tauri/icons/android-launcher.png` as the source for Tauri's platform-specific PNG/ICO/ICNS assets. GitHub Actions runs the Tauri icon generator before packaging each platform.

## Porting status and constraints

- Implemented in the desktop shell: responsive navigation, search, course browsing, course detail navigation, explore, assignments and profile entry points, plus page entrance transitions.
- The course browser is now data-backed by the Android 8.5.1 learn-menu seed rather than a small illustrative catalog.
- The Android APK does not include readable original React component source. Its Hermes bytecode is useful for reverse engineering but is not a drop-in desktop source tree.
- Authenticated progress, teacher/class assignment flows, mastery state, and the interactive lesson/exercise player are not complete. They require tracing the relevant bundled application behavior and reproducing the authenticated network/API flows; they must not be represented by static placeholder data.
- The desktop app must continue to be built by GitHub Actions on Windows, Linux, and macOS. Local builds are intentionally not part of this workflow.
