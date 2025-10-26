// Generic Web & Social Media Image Size Specifications
// By Leandro Santiago

use super::Size;

/// Social Media Profile Pictures
pub fn social_profile_pictures() -> Vec<Size> {
    vec![
        Size { width: 180, height: 180, name: "facebook-profile".to_string() },
        Size { width: 400, height: 400, name: "twitter-profile".to_string() },
        Size { width: 320, height: 320, name: "instagram-profile".to_string() },
        Size { width: 300, height: 300, name: "linkedin-profile".to_string() },
        Size { width: 800, height: 800, name: "youtube-profile".to_string() },
    ]
}

/// Social Media Cover/Banner Images
pub fn social_covers() -> Vec<Size> {
    vec![
        Size { width: 820, height: 312, name: "facebook-cover".to_string() },
        Size { width: 1500, height: 500, name: "twitter-header".to_string() },
        Size { width: 1584, height: 396, name: "linkedin-cover".to_string() },
        Size { width: 2560, height: 1440, name: "youtube-banner".to_string() },
    ]
}

/// Social Media Post Images
pub fn social_posts() -> Vec<Size> {
    vec![
        // Instagram
        Size { width: 1080, height: 1080, name: "instagram-square".to_string() },
        Size { width: 1080, height: 1350, name: "instagram-portrait".to_string() },
        Size { width: 1080, height: 608, name: "instagram-landscape".to_string() },
        Size { width: 1080, height: 1920, name: "instagram-story".to_string() },

        // Facebook
        Size { width: 1200, height: 630, name: "facebook-post".to_string() },
        Size { width: 1080, height: 1920, name: "facebook-story".to_string() },

        // Twitter
        Size { width: 1200, height: 675, name: "twitter-post".to_string() },

        // LinkedIn
        Size { width: 1200, height: 627, name: "linkedin-post".to_string() },

        // Pinterest
        Size { width: 1000, height: 1500, name: "pinterest-pin".to_string() },

        // TikTok
        Size { width: 1080, height: 1920, name: "tiktok-video".to_string() },
    ]
}

/// Web Favicons
pub fn favicons() -> Vec<Size> {
    vec![
        Size { width: 16, height: 16, name: "favicon-16".to_string() },
        Size { width: 32, height: 32, name: "favicon-32".to_string() },
        Size { width: 48, height: 48, name: "favicon-48".to_string() },
        Size { width: 64, height: 64, name: "favicon-64".to_string() },
        Size { width: 128, height: 128, name: "favicon-128".to_string() },
        Size { width: 256, height: 256, name: "favicon-256".to_string() },
    ]
}

/// Web Open Graph / Social Share Images
pub fn og_images() -> Vec<Size> {
    vec![
        Size { width: 1200, height: 630, name: "og-facebook".to_string() },
        Size { width: 1200, height: 627, name: "og-linkedin".to_string() },
        Size { width: 1200, height: 675, name: "og-twitter".to_string() },
        Size { width: 1200, height: 630, name: "og-general".to_string() },
    ]
}

/// Common Web Thumbnail Sizes
pub fn web_thumbnails() -> Vec<Size> {
    vec![
        Size { width: 150, height: 150, name: "thumb-150".to_string() },
        Size { width: 300, height: 300, name: "thumb-300".to_string() },
        Size { width: 400, height: 400, name: "thumb-400".to_string() },
        Size { width: 600, height: 600, name: "thumb-600".to_string() },
    ]
}

/// Common HD Resolutions
pub fn hd_resolutions() -> Vec<Size> {
    vec![
        Size { width: 1280, height: 720, name: "hd-720p".to_string() },
        Size { width: 1920, height: 1080, name: "full-hd-1080p".to_string() },
        Size { width: 2560, height: 1440, name: "qhd-1440p".to_string() },
        Size { width: 3840, height: 2160, name: "4k-uhd".to_string() },
    ]
}

/// Email Newsletter Images
pub fn email_newsletter() -> Vec<Size> {
    vec![
        Size { width: 600, height: 400, name: "email-header".to_string() },
        Size { width: 600, height: 200, name: "email-banner".to_string() },
        Size { width: 300, height: 300, name: "email-thumbnail".to_string() },
    ]
}

/// Blog Post Images
pub fn blog_images() -> Vec<Size> {
    vec![
        Size { width: 1200, height: 630, name: "blog-featured".to_string() },
        Size { width: 800, height: 600, name: "blog-inline".to_string() },
        Size { width: 400, height: 300, name: "blog-thumbnail".to_string() },
    ]
}

/// E-commerce Product Images
pub fn ecommerce_products() -> Vec<Size> {
    vec![
        Size { width: 2000, height: 2000, name: "product-zoom".to_string() },
        Size { width: 1000, height: 1000, name: "product-large".to_string() },
        Size { width: 500, height: 500, name: "product-medium".to_string() },
        Size { width: 250, height: 250, name: "product-thumbnail".to_string() },
        Size { width: 100, height: 100, name: "product-mini".to_string() },
    ]
}

/// Complete Social Media Pack
pub fn complete_social_pack() -> Vec<Size> {
    let mut sizes = Vec::new();
    sizes.extend(social_profile_pictures());
    sizes.extend(social_covers());
    sizes.extend(social_posts());
    sizes
}

/// Complete Web Pack
pub fn complete_web_pack() -> Vec<Size> {
    let mut sizes = Vec::new();
    sizes.extend(favicons());
    sizes.extend(og_images());
    sizes.extend(web_thumbnails());
    sizes.extend(blog_images());
    sizes
}

/// Generic Preset categories for UI selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenericPreset {
    SocialProfilePictures,
    SocialCovers,
    SocialPosts,
    Favicons,
    OGImages,
    WebThumbnails,
    HDResolutions,
    EmailNewsletter,
    BlogImages,
    EcommerceProducts,
    CompleteSocialPack,
    CompleteWebPack,
}

impl GenericPreset {
    pub fn get_sizes(&self) -> Vec<Size> {
        match self {
            GenericPreset::SocialProfilePictures => social_profile_pictures(),
            GenericPreset::SocialCovers => social_covers(),
            GenericPreset::SocialPosts => social_posts(),
            GenericPreset::Favicons => favicons(),
            GenericPreset::OGImages => og_images(),
            GenericPreset::WebThumbnails => web_thumbnails(),
            GenericPreset::HDResolutions => hd_resolutions(),
            GenericPreset::EmailNewsletter => email_newsletter(),
            GenericPreset::BlogImages => blog_images(),
            GenericPreset::EcommerceProducts => ecommerce_products(),
            GenericPreset::CompleteSocialPack => complete_social_pack(),
            GenericPreset::CompleteWebPack => complete_web_pack(),
        }
    }

    pub fn name(&self) -> &str {
        match self {
            GenericPreset::SocialProfilePictures => "Social Media Profile Pictures",
            GenericPreset::SocialCovers => "Social Media Covers/Banners",
            GenericPreset::SocialPosts => "Social Media Posts",
            GenericPreset::Favicons => "Web Favicons",
            GenericPreset::OGImages => "Open Graph / Social Share Images",
            GenericPreset::WebThumbnails => "Web Thumbnails",
            GenericPreset::HDResolutions => "HD Resolutions (720p - 4K)",
            GenericPreset::EmailNewsletter => "Email Newsletter Images",
            GenericPreset::BlogImages => "Blog Post Images",
            GenericPreset::EcommerceProducts => "E-commerce Product Images",
            GenericPreset::CompleteSocialPack => "Complete Social Media Pack",
            GenericPreset::CompleteWebPack => "Complete Web Pack",
        }
    }

    pub fn all() -> Vec<GenericPreset> {
        vec![
            GenericPreset::CompleteSocialPack,
            GenericPreset::CompleteWebPack,
            GenericPreset::SocialProfilePictures,
            GenericPreset::SocialCovers,
            GenericPreset::SocialPosts,
            GenericPreset::Favicons,
            GenericPreset::OGImages,
            GenericPreset::WebThumbnails,
            GenericPreset::HDResolutions,
            GenericPreset::EmailNewsletter,
            GenericPreset::BlogImages,
            GenericPreset::EcommerceProducts,
        ]
    }
}
