# Android 8.5.1 decompiled screen map

This reference was assembled from the user-provided `Khan Academy Decompiled JS.zip` archive. It is a decompiled Metro/Hermes module extraction, not the pristine original React Native source checkout. The extracted modules are useful implementation evidence, but decompiler artifacts and unresolved module aliases must be checked before porting behavior.

## Home feed

- `module_1864_HomeScreenWithVisibilityTracking.js` implements the home screen container. It uses a vertical scroll view, refresh control, offline banner, theme background, and renders an ordered list of feature-flagged cards.
- `module_1866_placeLanguagePickerHigh.js` defines card ordering and language-dependent curriculum-picker placement.
- `module_1876_WelcomeCard.js` shows the sign-in upsell only when the user is signed out. It uses `HomeUpsellCard`, green background `#5BA26F`, title `explore_welcome_ready_to_start`, subtitle `explore_sign_in_prompt`, and navigates to `LoginOptions`.
- `module_1913_RawRecentLessonsCard.js` always registers the Recent Lessons card. It has loading, error, empty, and loaded states. Loaded lessons use title, parent title, icon, availability, and native navigation/browser fallback.
- `module_1926_RawCurriculumPickerCard.js` is the language/region upsell. Background `#F7F2EA`; button opens the curriculum picker; dismiss is persisted through an action.
- `module_1915_RawKhanKidsCard.js` is the Khan Academy Kids cross-promotion. Background `#D4EAF1`; title “Khan Academy Kids”; Android target is the Play Store package `org.khankids.android`.
- `module_1887_RawMyCoursesCard.js` and `module_1922_RawMyClassesCard.js` are signed-in home cards.
- `module_1880_StreakCardConnected.js` renders the streak card only when the `mobileStreakCard` feature is enabled and streak configuration allows it.
- `module_1919__.js` is the My Classes upsell card.

The source's home feed order is assembled from feature selectors: streak, hidden dummy upsell, urgent banner, signed-out welcome, language-dependent curriculum picker, My Classes upsell, My Classes, My Courses, Recent Lessons, a second mutually exclusive curriculum-picker placement, and Khan Academy Kids. Some cards are conditional on sign-in state, remote feature flags, dismissal flags, language, and system language preferences.

## Navigation and learning screens

- `module_1796_RootNavigation.js`: root stack, tab navigator, settings stack, login options, deep-link handling, and app-level initialization.
- `module_2652_ExploreStackNavigator.js`: Explore stack; combines Explore screen, browse screens, and content screens.
- `module_1708_browseScreens.js`: Unit, Course, and Domain screens.
- `module_2673_UnitScreen.js`: unit content list, mastery state, practice-vs-learn row selection, recommendations, and content navigation.
- `module_2687_UnitScreenLargeLayout.js`: wide layout uses a 298px left-hand list and a flexible right-hand list.
- `module_2681_LearnItemCell.js`: learn row uses title, content-type/progress icon, and navigation.
- `module_2678_PracticeItemCell.js`: practice row adds mastery level, points out of 100, and “Up next” treatment.
- `module_1929_MyClassesScreen.js`: classes list split into active and inactive sections, pull-to-refresh, class navigation.
- `module_1799_SettingsScreen.js`: settings screen entry.
- `module_2842_ActivityScreen.js` and `module_2845_ExerciseShell.js`: activity/exercise rendering.

## API and state clues

- Home mount dispatches streak data, home modules, recent lessons, and (for normal signed-in users) user courses.
- Pull-to-refresh refetches the same data plus the urgent banner.
- `module_0974_USER_CHANGE.js` contains user/course/home/recent-lesson Redux actions and selectors.
- `module_1003_graphqlFetch.js`, `module_0777__getGraphQLOperation.js`, and the many `*Query.js` modules expose GraphQL operation plumbing and operation definitions.

## Desktop porting notes

React Native primitives must be translated to accessible DOM controls and Tauri-safe navigation. Keep Android's conditional card ordering, data states, and API contracts rather than substituting static marketing content. Preserve the Android large-layout split for unit screens, but adapt the breakpoint to desktop window dimensions. Transition animations should be short and respect `prefers-reduced-motion`.

The current desktop Home screen has been updated to use the Android-derived signed-out feed order and card structure. The empty Recent Lessons state is intentionally the signed-out state; signed-in data loading and real API wiring remain separate work.
