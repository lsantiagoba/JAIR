// Android PlayStore Image Size Specifications
// By Leandro Santiago

use super::Size;

/// Android Launcher Icon Sizes (Legacy)
pub fn launcher_icons() -> Vec<Size> {
    vec![
        Size { width: 36, height: 36, name: "ldpi".to_string() },
        Size { width: 48, height: 48, name: "mdpi".to_string() },
        Size { width: 72, height: 72, name: "hdpi".to_string() },
        Size { width: 96, height: 96, name: "xhdpi".to_string() },
        Size { width: 144, height: 144, name: "xxhdpi".to_string() },
        Size { width: 192, height: 192, name: "xxxhdpi".to_string() },
    ]
}

/// Android Adaptive Icon Sizes (Modern - requires foreground and background layers)
pub fn adaptive_icons() -> Vec<Size> {
    vec![
        Size { width: 81, height: 81, name: "mdpi-adaptive".to_string() },
        Size { width: 108, height: 108, name: "hdpi-adaptive".to_string() },
        Size { width: 162, height: 162, name: "xhdpi-adaptive".to_string() },
        Size { width: 216, height: 216, name: "xxhdpi-adaptive".to_string() },
        Size { width: 324, height: 324, name: "xxxhdpi-adaptive".to_string() },
    ]
}

/// Google Play Store Feature Graphic
pub fn feature_graphic() -> Vec<Size> {
    vec![
        Size { width: 1024, height: 500, name: "feature-graphic".to_string() },
    ]
}

/// Google Play Store Icon (High-res)
pub fn store_icon() -> Vec<Size> {
    vec![
        Size { width: 512, height: 512, name: "store-icon".to_string() },
    ]
}

/// Google Play Store Screenshots - Phone
pub fn screenshots_phone() -> Vec<Size> {
    vec![
        // Minimum 320px, maximum 3840px
        // Recommended 16:9 aspect ratio
        Size { width: 1080, height: 1920, name: "phone-portrait".to_string() },
        Size { width: 1920, height: 1080, name: "phone-landscape".to_string() },
    ]
}

/// Google Play Store Screenshots - 7-inch Tablet
pub fn screenshots_tablet_7() -> Vec<Size> {
    vec![
        Size { width: 1200, height: 1920, name: "tablet-7-portrait".to_string() },
        Size { width: 1920, height: 1200, name: "tablet-7-landscape".to_string() },
    ]
}

/// Google Play Store Screenshots - 10-inch Tablet
pub fn screenshots_tablet_10() -> Vec<Size> {
    vec![
        Size { width: 1600, height: 2560, name: "tablet-10-portrait".to_string() },
        Size { width: 2560, height: 1600, name: "tablet-10-landscape".to_string() },
    ]
}

/// Google Play Store Promo Graphic
pub fn promo_graphic() -> Vec<Size> {
    vec![
        Size { width: 180, height: 120, name: "promo-graphic".to_string() },
    ]
}

/// TV Banner (for Android TV apps)
pub fn tv_banner() -> Vec<Size> {
    vec![
        Size { width: 1280, height: 720, name: "tv-banner".to_string() },
    ]
}

/// All PlayStore Graphics
pub fn all_playstore_graphics() -> Vec<Size> {
    let mut sizes = Vec::new();
    sizes.extend(store_icon());
    sizes.extend(feature_graphic());
    sizes.extend(promo_graphic());
    sizes
}

/// All Launcher Icons (Legacy + Adaptive)
pub fn all_launcher_icons() -> Vec<Size> {
    let mut sizes = Vec::new();
    sizes.extend(launcher_icons());
    sizes.extend(adaptive_icons());
    sizes
}

/// All Screenshot Sizes
pub fn all_screenshots() -> Vec<Size> {
    let mut sizes = Vec::new();
    sizes.extend(screenshots_phone());
    sizes.extend(screenshots_tablet_7());
    sizes.extend(screenshots_tablet_10());
    sizes
}

/// Complete Android Asset Pack
pub fn complete_android_pack() -> Vec<Size> {
    let mut sizes = Vec::new();
    sizes.extend(launcher_icons());
    sizes.extend(adaptive_icons());
    sizes.extend(store_icon());
    sizes.extend(feature_graphic());
    sizes.extend(screenshots_phone());
    sizes.extend(promo_graphic());
    sizes
}

/// Preset categories for UI selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AndroidPreset {
    LauncherIcons,
    AdaptiveIcons,
    StoreIcon,
    FeatureGraphic,
    ScreenshotsPhone,
    ScreenshotsTablet7,
    ScreenshotsTablet10,
    PromoGraphic,
    TvBanner,
    AllPlayStoreGraphics,
    AllLauncherIcons,
    AllScreenshots,
    CompleteAndroidPack,
}

impl AndroidPreset {
    pub fn get_sizes(&self) -> Vec<Size> {
        match self {
            AndroidPreset::LauncherIcons => launcher_icons(),
            AndroidPreset::AdaptiveIcons => adaptive_icons(),
            AndroidPreset::StoreIcon => store_icon(),
            AndroidPreset::FeatureGraphic => feature_graphic(),
            AndroidPreset::ScreenshotsPhone => screenshots_phone(),
            AndroidPreset::ScreenshotsTablet7 => screenshots_tablet_7(),
            AndroidPreset::ScreenshotsTablet10 => screenshots_tablet_10(),
            AndroidPreset::PromoGraphic => promo_graphic(),
            AndroidPreset::TvBanner => tv_banner(),
            AndroidPreset::AllPlayStoreGraphics => all_playstore_graphics(),
            AndroidPreset::AllLauncherIcons => all_launcher_icons(),
            AndroidPreset::AllScreenshots => all_screenshots(),
            AndroidPreset::CompleteAndroidPack => complete_android_pack(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            AndroidPreset::LauncherIcons => "Launcher Icons (Legacy)",
            AndroidPreset::AdaptiveIcons => "Adaptive Icons",
            AndroidPreset::StoreIcon => "Play Store Icon (512x512)",
            AndroidPreset::FeatureGraphic => "Feature Graphic (1024x500)",
            AndroidPreset::ScreenshotsPhone => "Phone Screenshots",
            AndroidPreset::ScreenshotsTablet7 => "7-inch Tablet Screenshots",
            AndroidPreset::ScreenshotsTablet10 => "10-inch Tablet Screenshots",
            AndroidPreset::PromoGraphic => "Promo Graphic (180x120)",
            AndroidPreset::TvBanner => "TV Banner (1280x720)",
            AndroidPreset::AllPlayStoreGraphics => "All Play Store Graphics",
            AndroidPreset::AllLauncherIcons => "All Launcher Icons",
            AndroidPreset::AllScreenshots => "All Screenshots",
            AndroidPreset::CompleteAndroidPack => "Complete Android Pack",
        }
    }

    pub fn all() -> Vec<AndroidPreset> {
        vec![
            AndroidPreset::CompleteAndroidPack,
            AndroidPreset::AllPlayStoreGraphics,
            AndroidPreset::AllLauncherIcons,
            AndroidPreset::AllScreenshots,
            AndroidPreset::LauncherIcons,
            AndroidPreset::AdaptiveIcons,
            AndroidPreset::StoreIcon,
            AndroidPreset::FeatureGraphic,
            AndroidPreset::ScreenshotsPhone,
            AndroidPreset::ScreenshotsTablet7,
            AndroidPreset::ScreenshotsTablet10,
            AndroidPreset::PromoGraphic,
            AndroidPreset::TvBanner,
        ]
    }
}
