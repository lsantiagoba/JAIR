/* MIT License
 *
 * Copyright (c) 2025 Leandro Santiago
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 *
 * SPDX-License-Identifier: MIT
 */

use gtk::prelude::*;
use adw::subclass::prelude::*;
use adw::prelude::*;
use gtk::{gio, glib};
use gettextrs::gettext;
use std::path::PathBuf;

mod imp {
    use super::*;

    use std::cell::RefCell;
    use std::path::PathBuf;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/codes/lsb/jair/window.ui")]
    pub struct JairWindow {
        // Template widgets
        #[template_child]
        pub header_bar: TemplateChild<adw::HeaderBar>,
        #[template_child]
        pub platform_combo: TemplateChild<adw::ComboRow>,
        #[template_child]
        pub preset_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        pub preset_combo: TemplateChild<adw::ComboRow>,
        #[template_child]
        pub custom_size_group: TemplateChild<adw::PreferencesGroup>,
        #[template_child]
        pub custom_width_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub custom_height_entry: TemplateChild<gtk::Entry>,
        #[template_child]
        pub images_list: TemplateChild<gtk::ListBox>,
        #[template_child]
        pub add_images_btn: TemplateChild<gtk::Button>,
        #[template_child]
        pub clear_images_btn: TemplateChild<gtk::Button>,
        #[template_child]
        pub format_switch: TemplateChild<gtk::Switch>,
        #[template_child]
        pub format_label: TemplateChild<gtk::Label>,
        #[template_child]
        pub process_btn: TemplateChild<gtk::Button>,
        #[template_child]
        pub progress_bar: TemplateChild<gtk::ProgressBar>,
        #[template_child]
        pub status_label: TemplateChild<gtk::Label>,

        // Data
        pub selected_images: RefCell<Vec<PathBuf>>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for JairWindow {
        const NAME: &'static str = "JairWindow";
        type Type = super::JairWindow;
        type ParentType = adw::ApplicationWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for JairWindow {}
    impl WidgetImpl for JairWindow {}
    impl WindowImpl for JairWindow {}
    impl ApplicationWindowImpl for JairWindow {}
    impl AdwApplicationWindowImpl for JairWindow {}
}

glib::wrapper! {
    pub struct JairWindow(ObjectSubclass<imp::JairWindow>)
        @extends gtk::Widget, gtk::Window, gtk::ApplicationWindow, adw::ApplicationWindow,        @implements gio::ActionGroup, gio::ActionMap;
}

impl JairWindow {
    pub fn new<P: glib::IsA<gtk::Application>>(application: &P) -> Self {
        let window: Self = glib::Object::builder()
            .property("application", application)
            .build();

        window.setup_ui();
        window.setup_signals();
        window
    }

    fn setup_ui(&self) {
        // Setup platform combo box
        let platform_list = gtk::StringList::new(&[
            &gettext("Android"),
            &gettext("iOS / Apple"),
            &gettext("Web & Social Media"),
            &gettext("Custom"),
        ]);
        self.imp().platform_combo.set_model(Some(&platform_list));
        self.imp().platform_combo.set_selected(0);

        // Load initial presets for Android
        self.update_presets_for_platform(0);
        
        // Initialize format label
        self.imp().format_label.set_text(&gettext("PNG"));
    }

    fn update_presets_for_platform(&self, platform_index: u32) {
        let imp = self.imp();

        // Check if Custom is selected
        if platform_index == 3 {
            // Hide preset group and show custom size group
            imp.preset_group.set_visible(false);
            imp.custom_size_group.set_visible(true);
            return;
        }

        // Show preset group and hide custom size group for other platforms
        imp.preset_group.set_visible(true);
        imp.custom_size_group.set_visible(false);

        let string_list = gtk::StringList::new(&[]);

        match platform_index {
            0 => {
                // Android
                use crate::models::android_sizes::AndroidPreset;
                for preset in AndroidPreset::all() {
                    string_list.append(preset.name());
                }
            }
            1 => {
                // iOS / Apple
                use crate::models::ios_sizes::IOSPreset;
                for preset in IOSPreset::all() {
                    string_list.append(preset.name());
                }
            }
            2 => {
                // Web & Social Media
                use crate::models::generic_sizes::GenericPreset;
                for preset in GenericPreset::all() {
                    string_list.append(preset.name());
                }
            }
            _ => {}
        }

        imp.preset_combo.set_model(Some(&string_list));
        imp.preset_combo.set_selected(0);
    }

    fn setup_signals(&self) {
        let imp = self.imp();

        // Platform combo changed
        imp.platform_combo.connect_selected_notify(glib::clone!(@weak self as window => move |combo| {
            let selected = combo.selected();
            window.update_presets_for_platform(selected);
        }));

        // Add Images button
        imp.add_images_btn.connect_clicked(glib::clone!(@weak self as window => move |_| {
            window.on_add_images_clicked();
        }));

        // Clear Images button
        imp.clear_images_btn.connect_clicked(glib::clone!(@weak self as window => move |_| {
            window.on_clear_images_clicked();
        }));

        // Format switch
        imp.format_switch.connect_active_notify(glib::clone!(@weak self as window => move |switch| {
            let label = window.imp().format_label.get();
            if switch.is_active() {
                label.set_text(&gettext("PNG"));
            } else {
                label.set_text(&gettext("JPEG"));
            }
        }));

        // Process button
        imp.process_btn.connect_clicked(glib::clone!(@weak self as window => move |_| {
            window.on_process_clicked();
        }));
    }

    fn on_add_images_clicked(&self) {
        let dialog = gtk::FileChooserNative::new(
            Some("Select Images"),
            Some(self),
            gtk::FileChooserAction::Open,
            Some("Select"),
            Some("Cancel"),
        );

        dialog.set_select_multiple(true);

        // Create file filter for images
        let filter = gtk::FileFilter::new();
        filter.set_name(Some("Image files"));
        filter.add_mime_type("image/*");
        filter.add_pattern("*.jpg");
        filter.add_pattern("*.jpeg");
        filter.add_pattern("*.png");
        filter.add_pattern("*.gif");
        filter.add_pattern("*.bmp");
        filter.add_pattern("*.webp");

        dialog.add_filter(&filter);

        dialog.connect_response(glib::clone!(@weak self as window => move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                let files = dialog.files();
                window.add_images_from_files(&files);
            }
        }));

        dialog.show();
    }

    fn add_images_from_files(&self, files: &gio::ListModel) {
        use crate::services::processor;

        let mut images = self.imp().selected_images.borrow_mut();

        for i in 0..files.n_items() {
            if let Some(file) = files.item(i).and_downcast::<gio::File>() {
                if let Some(path) = file.path() {
                    if processor::is_supported_image(&path) && !images.contains(&path) {
                        images.push(path.clone());
                        self.add_image_to_list(&path);
                    }
                }
            }
        }

        // Enable process button if we have images
        self.imp().process_btn.set_sensitive(!images.is_empty());
    }

    fn add_image_to_list(&self, path: &std::path::Path) {
        let row = adw::ActionRow::new();

        let filename = path.file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown");

        row.set_title(filename);

        if let Some(parent) = path.parent() {
            row.set_subtitle(&parent.display().to_string());
        }

        // Add remove button
        let remove_btn = gtk::Button::builder()
            .icon_name("edit-delete-symbolic")
            .valign(gtk::Align::Center)
            .build();
        remove_btn.add_css_class("flat");

        let path_clone = path.to_path_buf();
        remove_btn.connect_clicked(glib::clone!(@weak self as window, @weak row => move |_| {
            window.remove_image(&path_clone, &row);
        }));

        row.add_suffix(&remove_btn);
        self.imp().images_list.append(&row);
    }

    fn remove_image(&self, path: &std::path::Path, row: &adw::ActionRow) {
        let mut images = self.imp().selected_images.borrow_mut();
        images.retain(|p| p != path);

        self.imp().images_list.remove(row);

        // Disable process button if no images left
        self.imp().process_btn.set_sensitive(!images.is_empty());
    }

    fn on_clear_images_clicked(&self) {
        self.imp().selected_images.borrow_mut().clear();

        // Remove all rows from the list
        while let Some(row) = self.imp().images_list.first_child() {
            self.imp().images_list.remove(&row);
        }

        self.imp().process_btn.set_sensitive(false);
        self.imp().status_label.set_visible(false);
    }

    fn on_process_clicked(&self) {
        let imp = self.imp();

        // Get selected platform and preset
        let platform_idx = imp.platform_combo.selected();

        // Get sizes based on platform
        let mut sizes = if platform_idx == 3 {
            // Custom platform - only use custom size
            Vec::new()
        } else {
            let preset_idx = imp.preset_combo.selected() as usize;
            match platform_idx {
                0 => {
                    // Android
                    use crate::models::android_sizes::AndroidPreset;
                    let preset = AndroidPreset::all()[preset_idx];
                    preset.get_sizes()
                }
                1 => {
                    // iOS / Apple
                    use crate::models::ios_sizes::IOSPreset;
                    let preset = IOSPreset::all()[preset_idx];
                    preset.get_sizes()
                }
                2 => {
                    // Web & Social Media
                    use crate::models::generic_sizes::GenericPreset;
                    let preset = GenericPreset::all()[preset_idx];
                    preset.get_sizes()
                }
                _ => return,
            }
        };

        // Add custom size if provided (or if Custom platform is selected)
        let custom_width_text = imp.custom_width_entry.text();
        let custom_height_text = imp.custom_height_entry.text();

        if !custom_width_text.is_empty() && !custom_height_text.is_empty() {
            if let (Ok(width), Ok(height)) = (
                custom_width_text.parse::<u32>(),
                custom_height_text.parse::<u32>()
            ) {
                if width > 0 && height > 0 {
                    sizes.push(crate::models::Size {
                        width,
                        height,
                        name: format!("Custom_{}x{}", width, height),
                    });
                }
            }
        }

        // If Custom platform is selected and no valid custom size, show error
        if platform_idx == 3 && sizes.is_empty() {
            imp.status_label.set_visible(true);
            imp.status_label.set_text("Please enter valid width and height values");
            return;
        }

        // Get output format
        let use_png = imp.format_switch.is_active();

        // Get selected images
        let images = imp.selected_images.borrow().clone();

        if images.is_empty() {
            return;
        }

        // Show file chooser for output directory
        let dialog = gtk::FileChooserNative::new(
            Some("Select Output Directory"),
            Some(self),
            gtk::FileChooserAction::SelectFolder,
            Some("Select"),
            Some("Cancel"),
        );

        dialog.connect_response(glib::clone!(@weak self as window => move |dialog, response| {
            if response == gtk::ResponseType::Accept {
                if let Some(file) = dialog.file() {
                    if let Some(out_dir) = file.path() {
                        window.process_images(images.clone(), out_dir, sizes.clone(), use_png);
                    }
                }
            }
        }));

        dialog.show();
    }

    fn process_images(
        &self,
        images: Vec<PathBuf>,
        out_dir: PathBuf,
        sizes: Vec<crate::models::Size>,
        use_png: bool,
    ) {
        use crate::services::processor;

        let imp = self.imp();

        // Show progress
        imp.progress_bar.set_visible(true);
        imp.progress_bar.set_fraction(0.0);
        imp.status_label.set_visible(true);
        imp.status_label.set_text("Processing images...");
        imp.process_btn.set_sensitive(false);

        // Process in a separate thread
        let (sender, receiver) = async_channel::unbounded();

        std::thread::spawn(move || {
            let total = images.len();
            let mut processed = 0;
            let mut successful = 0;
            let mut failed = 0;

            for image in &images {
                match processor::resize_and_save(image, &out_dir, &sizes, use_png) {
                    Ok(_) => successful += 1,
                    Err(_) => failed += 1,
                }

                processed += 1;
                let progress = processed as f64 / total as f64;
                let _ = sender.send_blocking((progress, processed, total, successful, failed, false));
            }

            // Send completion signal
            let _ = sender.send_blocking((1.0, processed, total, successful, failed, true));
        });

        // Update UI from main thread
        glib::spawn_future_local(glib::clone!(@weak self as window => async move {
            while let Ok((progress, processed, total, successful, failed, done)) = receiver.recv().await {
                let imp = window.imp();
                imp.progress_bar.set_fraction(progress);

                if done {
                    imp.progress_bar.set_visible(false);
                    imp.process_btn.set_sensitive(true);

                    let status = format!(
                        "Completed: {} successful, {} failed out of {} images",
                        successful, failed, total
                    );
                    imp.status_label.set_text(&status);
                    break;
                } else {
                    let status = format!("Processing {} of {} images...", processed, total);
                    imp.status_label.set_text(&status);
                }
            }
        }));
    }
}
