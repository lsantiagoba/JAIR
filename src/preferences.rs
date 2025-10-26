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

mod imp {
    use super::*;

    #[derive(Debug, Default, gtk::CompositeTemplate)]
    #[template(resource = "/codes/lsb/jair/preferences.ui")]
    pub struct JairPreferencesWindow {
        #[template_child]
        pub language_combo: TemplateChild<adw::ComboRow>,
    }

    #[glib::object_subclass]
    impl ObjectSubclass for JairPreferencesWindow {
        const NAME: &'static str = "JairPreferencesWindow";
        type Type = super::JairPreferencesWindow;
        type ParentType = adw::PreferencesWindow;

        fn class_init(klass: &mut Self::Class) {
            klass.bind_template();
        }

        fn instance_init(obj: &glib::subclass::InitializingObject<Self>) {
            obj.init_template();
        }
    }

    impl ObjectImpl for JairPreferencesWindow {
        fn constructed(&self) {
            self.parent_constructed();
            let obj = self.obj();
            obj.setup_language_combo();
        }
    }

    impl WidgetImpl for JairPreferencesWindow {}
    impl WindowImpl for JairPreferencesWindow {}
    impl AdwWindowImpl for JairPreferencesWindow {}
    impl PreferencesWindowImpl for JairPreferencesWindow {}
}

glib::wrapper! {
    pub struct JairPreferencesWindow(ObjectSubclass<imp::JairPreferencesWindow>)
        @extends gtk::Widget, gtk::Window, adw::Window, adw::PreferencesWindow,
        @implements gio::ActionGroup, gio::ActionMap, gtk::Accessible, gtk::Buildable,
                    gtk::ConstraintTarget, gtk::Native, gtk::Root, gtk::ShortcutManager;
}

impl JairPreferencesWindow {
    pub fn new() -> Self {
        glib::Object::builder().build()
    }

    fn setup_language_combo(&self) {
        let imp = self.imp();
        
        // Create the language options
        let languages = gtk::StringList::new(&[
            &gettextrs::gettext("System Default"),
            "English",
            "Español",
        ]);
        
        imp.language_combo.set_model(Some(&languages));

        // Load current language setting
        let settings = gio::Settings::new("codes.lsb.jair");
        let current_lang = settings.string("language");
        
        let selected = match current_lang.as_str() {
            "system" => 0,
            "en" => 1,
            "es" => 2,
            _ => 0,
        };
        
        imp.language_combo.set_selected(selected);

        // Connect to language change
        imp.language_combo.connect_selected_notify(glib::clone!(@weak self as window => move |combo| {
            let settings = gio::Settings::new("codes.lsb.jair");
            let selected = combo.selected();
            
            let lang_code = match selected {
                0 => "system",
                1 => "en", 
                2 => "es",
                _ => "system",
            };
            
            let _ = settings.set_string("language", lang_code);
            
            // Show restart notification
            let toast = adw::Toast::builder()
                .title(&gettextrs::gettext("Language will change after restarting the application"))
                .timeout(3)
                .build();
                
                if let Some(main_window) = window.transient_for() {
                if let Some(toast_overlay) = main_window
                    .first_child()
                    .and_then(|child| child.downcast::<adw::ToastOverlay>().ok()) {
                    toast_overlay.add_toast(toast);
                }
            }
        }));
    }
}