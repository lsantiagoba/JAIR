// iOS App Store Image Size Specifications
// By Leandro Santiago

use super::Size;

/// iOS App Icons (Universal)
pub fn app_icons() -> Vec<Size> {
    vec![
        // iPhone
        Size { width: 120, height: 120, name: "iphone-60pt-2x".to_string() },
        Size { width: 180, height: 180, name: "iphone-60pt-3x".to_string() },

        // iPad
        Size { width: 76, height: 76, name: "ipad-76pt-1x".to_string() },
        Size { width: 152, height: 152, name: "ipad-76pt-2x".to_string() },

        // iPad Pro
        Size { width: 167, height: 167, name: "ipad-pro-83.5pt-2x".to_string() },

        // App Store
        Size { width: 1024, height: 1024, name: "app-store-1024pt".to_string() },
    ]
}

/// iOS Spotlight & Settings Icons
pub fn spotlight_settings_icons() -> Vec<Size> {
    vec![
        // Spotlight iPhone
        Size { width: 80, height: 80, name: "spotlight-iphone-40pt-2x".to_string() },
        Size { width: 120, height: 120, name: "spotlight-iphone-40pt-3x".to_string() },

        // Spotlight iPad
        Size { width: 40, height: 40, name: "spotlight-ipad-40pt-1x".to_string() },
        Size { width: 80, height: 80, name: "spotlight-ipad-40pt-2x".to_string() },

        // Settings iPhone
        Size { width: 58, height: 58, name: "settings-iphone-29pt-2x".to_string() },
        Size { width: 87, height: 87, name: "settings-iphone-29pt-3x".to_string() },

        // Settings iPad
        Size { width: 29, height: 29, name: "settings-ipad-29pt-1x".to_string() },
        Size { width: 58, height: 58, name: "settings-ipad-29pt-2x".to_string() },
    ]
}

/// iOS Notification Icons
pub fn notification_icons() -> Vec<Size> {
    vec![
        // iPhone
        Size { width: 40, height: 40, name: "notification-iphone-20pt-2x".to_string() },
        Size { width: 60, height: 60, name: "notification-iphone-20pt-3x".to_string() },

        // iPad
        Size { width: 20, height: 20, name: "notification-ipad-20pt-1x".to_string() },
        Size { width: 40, height: 40, name: "notification-ipad-20pt-2x".to_string() },
    ]
}

/// iOS App Store Screenshots - iPhone
pub fn screenshots_iphone() -> Vec<Size> {
    vec![
        // iPhone 15 Pro Max / 14 Pro Max / 13 Pro Max / 12 Pro Max
        Size { width: 1290, height: 2796, name: "iphone-6.7-portrait".to_string() },
        Size { width: 2796, height: 1290, name: "iphone-6.7-landscape".to_string() },

        // iPhone 15 / 14 / 13 / 12 / 11 Pro Max / XS Max
        Size { width: 1242, height: 2688, name: "iphone-6.5-portrait".to_string() },
        Size { width: 2688, height: 1242, name: "iphone-6.5-landscape".to_string() },

        // iPhone 14 Plus / 13 / 12 / 11 / XR
        Size { width: 1242, height: 2208, name: "iphone-5.5-portrait".to_string() },
        Size { width: 2208, height: 1242, name: "iphone-5.5-landscape".to_string() },
    ]
}

/// iOS App Store Screenshots - iPad
pub fn screenshots_ipad() -> Vec<Size> {
    vec![
        // iPad Pro 12.9" (3rd, 4th, 5th, 6th gen)
        Size { width: 2048, height: 2732, name: "ipad-12.9-portrait".to_string() },
        Size { width: 2732, height: 2048, name: "ipad-12.9-landscape".to_string() },

        // iPad Pro 11" / iPad Air
        Size { width: 1668, height: 2388, name: "ipad-11-portrait".to_string() },
        Size { width: 2388, height: 1668, name: "ipad-11-landscape".to_string() },
    ]
}

/// Apple Watch App Icons
pub fn watch_icons() -> Vec<Size> {
    vec![
        Size { width: 48, height: 48, name: "watch-24pt-2x-notification".to_string() },
        Size { width: 55, height: 55, name: "watch-27.5pt-2x-notification".to_string() },
        Size { width: 58, height: 58, name: "watch-29pt-2x-settings".to_string() },
        Size { width: 87, height: 87, name: "watch-29pt-3x-settings".to_string() },
        Size { width: 80, height: 80, name: "watch-40pt-2x-home".to_string() },
        Size { width: 88, height: 88, name: "watch-44pt-2x-home".to_string() },
        Size { width: 92, height: 92, name: "watch-46pt-2x-home".to_string() },
        Size { width: 100, height: 100, name: "watch-50pt-2x-home".to_string() },
        Size { width: 172, height: 172, name: "watch-86pt-2x-short-look".to_string() },
        Size { width: 196, height: 196, name: "watch-98pt-2x-short-look".to_string() },
        Size { width: 216, height: 216, name: "watch-108pt-2x-short-look".to_string() },
        Size { width: 1024, height: 1024, name: "watch-app-store".to_string() },
    ]
}

/// macOS App Icons
pub fn macos_icons() -> Vec<Size> {
    vec![
        Size { width: 16, height: 16, name: "mac-16pt-1x".to_string() },
        Size { width: 32, height: 32, name: "mac-16pt-2x".to_string() },
        Size { width: 32, height: 32, name: "mac-32pt-1x".to_string() },
        Size { width: 64, height: 64, name: "mac-32pt-2x".to_string() },
        Size { width: 128, height: 128, name: "mac-128pt-1x".to_string() },
        Size { width: 256, height: 256, name: "mac-128pt-2x".to_string() },
        Size { width: 256, height: 256, name: "mac-256pt-1x".to_string() },
        Size { width: 512, height: 512, name: "mac-256pt-2x".to_string() },
        Size { width: 512, height: 512, name: "mac-512pt-1x".to_string() },
        Size { width: 1024, height: 1024, name: "mac-512pt-2x".to_string() },
    ]
}

/// All iOS App Icons
pub fn all_ios_icons() -> Vec<Size> {
    let mut sizes = Vec::new();
    sizes.extend(app_icons());
    sizes.extend(spotlight_settings_icons());
    sizes.extend(notification_icons());
    sizes
}

/// All iOS Screenshots
pub fn all_ios_screenshots() -> Vec<Size> {
    let mut sizes = Vec::new();
    sizes.extend(screenshots_iphone());
    sizes.extend(screenshots_ipad());
    sizes
}

/// Complete iOS Pack
pub fn complete_ios_pack() -> Vec<Size> {
    let mut sizes = Vec::new();
    sizes.extend(app_icons());
    sizes.extend(spotlight_settings_icons());
    sizes.extend(notification_icons());
    sizes.extend(screenshots_iphone());
    sizes.extend(screenshots_ipad());
    sizes
}

/// Complete Apple Ecosystem Pack (iOS + watchOS + macOS)
pub fn complete_apple_pack() -> Vec<Size> {
    let mut sizes = Vec::new();
    sizes.extend(app_icons());
    sizes.extend(spotlight_settings_icons());
    sizes.extend(notification_icons());
    sizes.extend(watch_icons());
    sizes.extend(macos_icons());
    sizes
}

/// iOS Preset categories for UI selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IOSPreset {
    AppIcons,
    SpotlightSettings,
    NotificationIcons,
    ScreenshotsIPhone,
    ScreenshotsIPad,
    WatchIcons,
    MacOSIcons,
    AllIOSIcons,
    AllIOSScreenshots,
    CompleteIOSPack,
    CompleteApplePack,
}

impl IOSPreset {
    pub fn get_sizes(&self) -> Vec<Size> {
        match self {
            IOSPreset::AppIcons => app_icons(),
            IOSPreset::SpotlightSettings => spotlight_settings_icons(),
            IOSPreset::NotificationIcons => notification_icons(),
            IOSPreset::ScreenshotsIPhone => screenshots_iphone(),
            IOSPreset::ScreenshotsIPad => screenshots_ipad(),
            IOSPreset::WatchIcons => watch_icons(),
            IOSPreset::MacOSIcons => macos_icons(),
            IOSPreset::AllIOSIcons => all_ios_icons(),
            IOSPreset::AllIOSScreenshots => all_ios_screenshots(),
            IOSPreset::CompleteIOSPack => complete_ios_pack(),
            IOSPreset::CompleteApplePack => complete_apple_pack(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            IOSPreset::AppIcons => "App Icons",
            IOSPreset::SpotlightSettings => "Spotlight & Settings Icons",
            IOSPreset::NotificationIcons => "Notification Icons",
            IOSPreset::ScreenshotsIPhone => "iPhone Screenshots",
            IOSPreset::ScreenshotsIPad => "iPad Screenshots",
            IOSPreset::WatchIcons => "Apple Watch Icons",
            IOSPreset::MacOSIcons => "macOS Icons",
            IOSPreset::AllIOSIcons => "All iOS Icons",
            IOSPreset::AllIOSScreenshots => "All iOS Screenshots",
            IOSPreset::CompleteIOSPack => "Complete iOS Pack",
            IOSPreset::CompleteApplePack => "Complete Apple Pack (iOS + Watch + Mac)",
        }
    }

    pub fn all() -> Vec<IOSPreset> {
        vec![
            IOSPreset::CompleteApplePack,
            IOSPreset::CompleteIOSPack,
            IOSPreset::AllIOSIcons,
            IOSPreset::AllIOSScreenshots,
            IOSPreset::AppIcons,
            IOSPreset::SpotlightSettings,
            IOSPreset::NotificationIcons,
            IOSPreset::ScreenshotsIPhone,
            IOSPreset::ScreenshotsIPad,
            IOSPreset::WatchIcons,
            IOSPreset::MacOSIcons,
        ]
    }
}
