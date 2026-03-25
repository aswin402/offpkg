use crate::stacks::{Stack, StackFile};

pub fn flutter_riverpod(app_name: Option<&str>) -> Stack {
    let name = app_name.unwrap_or("my_app").to_string();

    // Nice title: "MyApp", "TodoApp", etc.
    let title = app_name
        .map(|s| {
            let mut t = s.to_string();
            if let Some(first) = t.get_mut(0..1) {
                first.make_ascii_uppercase();
            }
            t
        })
        .unwrap_or_else(|| "MyApp".to_string());

    Stack {
        name: if name.is_empty() { "flutter-riverpod".into() } else { format!("flutter-riverpod-{}", name).into() },
        runtime: "flutter".into(),
        description: "Universal Flutter + Riverpod 2/3 + GoRouter + Material 3 (Custom Color Extensions) + Logger + Google Fonts".into(),
        packages: vec![
            "flutter_riverpod".into(),
            "go_router".into(),
            "logger".into(),
            "google_fonts".into(),
        ],
        dev_packages: vec![],
        transitive_packages: vec![],
        files: vec![
            // ==================== MAIN ====================
            StackFile {
                path: "lib/main.dart".into(),
                content: format!(
                    r##"import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'core/router/app_router.dart';
import 'core/theme/app_themes.dart';
import 'core/theme/theme_provider.dart';

void main() {{
  runApp(
    const ProviderScope(
      child: MyApp(),
    ),
  );
}}

class MyApp extends ConsumerWidget {{
  const MyApp({{super.key}});

  @override
  Widget build(BuildContext context, WidgetRef ref) {{
    final router = ref.watch(routerProvider);
    final themeMode = ref.watch(themeProvider);

    return MaterialApp.router(
      title: "{title}",
      debugShowCheckedModeBanner: false,
      themeMode: themeMode,
      darkTheme: AppTheme.dark,
      theme: AppTheme.light,
      routerConfig: router,
    );
  }}
}}
"##,
                    title = title
                )
                .into(),
                binary_content: None,
            },

            // ==================== CORE ====================
            StackFile {
                path: "lib/core/extension/context_extension.dart".into(),
                content: r##"import 'package:flutter/material.dart';
import '../theme/app_color_extension.dart';

extension ThemeContext on BuildContext {
  ThemeData get theme => Theme.of(this);
  TextTheme get textTheme => theme.textTheme;
  ColorScheme get colors => theme.colorScheme;
  ColorScheme get cs => colors;
  AppColorExtension get ac => theme.extension<AppColorExtension>()!;
}
"##.into(),
                binary_content: None,
            },

            StackFile {
                path: "lib/core/layout/app_shell.dart".into(),
                content: r##"import 'package:flutter/material.dart';
import '../../shared/widgets/app_footer/app_footer.dart';
import '../../shared/widgets/app_top_navbar/app_top_navbar.dart';

class AppShell extends StatelessWidget {
  final Widget child;
  const AppShell({super.key, required this.child});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: const AppTopNavbar(),
      body: child,
      bottomNavigationBar: const AppFooter(),
    );
  }
}
"##.into(),
                binary_content: None,
            },

            StackFile {
                path: "lib/core/logger/app_logger.dart".into(),
                content: r##"import 'package:flutter/foundation.dart';
import 'package:logger/logger.dart';

class AppLogger {
  static final Logger _logger = Logger(
    printer: PrettyPrinter(
      methodCount: 1,
      errorMethodCount: 8,
      lineLength: 120,
      colors: true,
      printEmojis: true,
      dateTimeFormat: DateTimeFormat.onlyTimeAndSinceStart,
    ),
    level: kReleaseMode ? Level.warning : Level.debug,
  );

  static void debug(String message) => _logger.d(message);
  static void info(String message) => _logger.i(message);
  static void warning(String message) => _logger.w(message);
  static void error(String message, [dynamic error, StackTrace? stackTrace]) =>
      _logger.e(message, error: error, stackTrace: stackTrace);
}
"##.into(),
                binary_content: None,
            },

            // ==================== ROUTER ====================
            StackFile {
                path: "lib/core/router/app_router.dart".into(),
                content: r##"import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import '../../features/home/presentation/pages/home_page.dart';
import '../layout/app_shell.dart';
import 'app_routes.dart';

final routerProvider = Provider<GoRouter>((ref) {
  return GoRouter(
    initialLocation: AppRoutes.home,
    routes: [
      ShellRoute(
        builder: (context, state, child) => AppShell(child: child),
        routes: [
          GoRoute(
            path: AppRoutes.home,
            builder: (context, state) => const HomePage(),
          ),
          // Add more routes here as you create features
        ],
      ),
    ],
  );
});
"##.into(),
                binary_content: None,
            },

            StackFile {
                path: "lib/core/router/app_routes.dart".into(),
                content: r##"class AppRoutes {
  static const home = '/';
  // Example: static const profile = '/profile';
}
"##.into(),
                binary_content: None,
            },

            // ==================== THEME ====================
            StackFile {
                path: "lib/core/theme/app_colors.dart".into(),
                content: r##"// ... (paste your full app_colors.dart content here - LightColors + DarkColors)"##.into(), binary_content: None,
            },
            StackFile {
                path: "lib/core/theme/app_color_extension.dart".into(),
                content: r##"// ... (paste your full AppColorExtension with copyWith + lerp here)"##.into(), binary_content: None,
            },
            StackFile {
                path: "lib/core/theme/app_text_styles.dart".into(),
                content: r##"// ... (paste your Google Fonts setup here)"##.into(), binary_content: None,
            },
            StackFile {
                path: "lib/core/theme/app_themes.dart".into(),
                content: r##"import 'package:flutter/material.dart';
import 'light_theme.dart';
import 'dark_theme.dart';

class AppTheme {
  static ThemeData get light => lightTheme;
  static ThemeData get dark => darkTheme;
}
"##.into(),
                binary_content: None,
            },
            StackFile {
                path: "lib/core/theme/light_theme.dart".into(),
                content: r##"// ... (paste your full light_theme.dart here)"##.into(), binary_content: None,
            },
            StackFile {
                path: "lib/core/theme/dark_theme.dart".into(),
                content: r##"// ... (paste your full dark_theme.dart here)"##.into(), binary_content: None,
            },
            StackFile {
                path: "lib/core/theme/theme_provider.dart".into(),
                content: r##"import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

class ThemeNotifier extends Notifier<ThemeMode> {
  @override
  ThemeMode build() => ThemeMode.system;

  void setThemeMode(ThemeMode mode) => state = mode;
  void toggleTheme() => state = state == ThemeMode.light ? ThemeMode.dark : ThemeMode.light;
}

final themeProvider = NotifierProvider<ThemeNotifier, ThemeMode>(() => ThemeNotifier());
"##.into(),
                binary_content: None,
            },

            // Component themes (keep all your excellent component files)
            StackFile {
                path: "lib/core/theme/components/component_themes.dart".into(),
                content: r##"// ... (your component_themes.dart)"##.into(), binary_content: None,
            },
            StackFile {
                path: "lib/core/theme/components/appbar_theme.dart".into(),
                content: r##"// ... (your appbar_theme.dart)"##.into(), binary_content: None,
            },
            StackFile {
                path: "lib/core/theme/components/button_theme.dart".into(),
                content: r##"// ... (your button_theme.dart)"##.into(), binary_content: None,
            },
            StackFile {
                path: "lib/core/theme/components/input_theme.dart".into(),
                content: r##"// ... (your input_theme.dart)"##.into(), binary_content: None,
            },
            StackFile {
                path: "lib/core/theme/components/card_theme.dart".into(),
                content: r##"// ... (your card_theme.dart)"##.into(), binary_content: None,
            },
            StackFile {
                path: "lib/core/theme/components/bottom_nav_theme.dart".into(),
                content: r##"// ... (your bottom_nav_theme.dart)"##.into(), binary_content: None,
            },

            // ==================== FEATURES ====================
            StackFile {
                path: "lib/features/home/presentation/pages/home_page.dart".into(),
                content: r##"import 'package:flutter/material.dart';
import '../../../core/extension/context_extension.dart';

class HomePage extends StatelessWidget {
  const HomePage({super.key});

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: [
          Text("Welcome to the App", style: context.textTheme.headlineLarge),
          const SizedBox(height: 32),
          ElevatedButton(
            onPressed: () {},
            child: const Text("Get Started"),
          ),
        ],
      ),
    );
  }
}
"##.into(),
                binary_content: None,
            },

            // ==================== SHARED WIDGETS ====================
            StackFile {
                path: "lib/shared/widgets/app_footer/app_footer.dart".into(),
                content: r##"// ... (your app_footer.dart content)"##.into(), binary_content: None,
            },
            StackFile {
                path: "lib/shared/widgets/app_top_navbar/app_top_navbar.dart".into(),
                content: r##"// ... (your app_top_navbar.dart content)"##.into(), binary_content: None,
            },
            StackFile {
                path: "lib/shared/widgets/app_top_navbar/toggle_theme_button.dart".into(),
                content: r##"// ... (your toggle_theme_button.dart content)"##.into(), binary_content: None,
            },
        ],
    }
}
