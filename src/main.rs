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

mod application;
mod config;
mod window;
mod preferences;
mod models;
mod services;

use self::application::JairApplication;
use self::window::JairWindow;

use config::{GETTEXT_PACKAGE, LOCALEDIR, PKGDATADIR};
use gettextrs::{bind_textdomain_codeset, bindtextdomain, textdomain};
use gtk::{gio, glib};
use gtk::prelude::*;

fn main() -> glib::ExitCode {
    // Try to read language preference from dconf/gsettings using gsettings command
    // This avoids initializing GTK too early
    if let Ok(output) = std::process::Command::new("gsettings")
        .args(&["get", "codes.lsb.jair", "language"])
        .output() 
    {
        if output.status.success() {
            let lang_pref = String::from_utf8_lossy(&output.stdout).trim().trim_matches('\'').to_string();
            if lang_pref != "system" && !lang_pref.is_empty() {
                std::env::set_var("LANGUAGE", &lang_pref);
                std::env::set_var("LANG", format!("{}.UTF-8", lang_pref));
            }
        }
    }

    // Set up gettext translations
    // Try to bind to multiple possible locale directories for development and installed builds
    let mut locale_dirs = vec![];

    // Check if we're in a snap environment
    if let Ok(snap_dir) = std::env::var("SNAP") {
        locale_dirs.push(format!("{}/usr/share/locale", snap_dir));
    }

    locale_dirs.extend([
        LOCALEDIR.to_string(),
        "/usr/share/locale".to_string(),
        "/usr/local/share/locale".to_string(),
        "./po".to_string(),
        "../po".to_string(),
    ]);

    let mut locale_bound = false;
    for dir in &locale_dirs {
        let path = std::path::Path::new(dir);
        // Check if directory exists and contains actual translation files
        if path.exists() {
            let test_locale = path.join("es").join("LC_MESSAGES").join(format!("{}.mo", GETTEXT_PACKAGE));
            if test_locale.exists() || dir == &"./po" || dir == &"../po" {
                bindtextdomain(GETTEXT_PACKAGE, dir).expect("Failed to bind text domain");
                locale_bound = true;
                break;
            }
        }
    }

    if !locale_bound {
        eprintln!("Warning: Could not find translation files in any expected location");
    }

    bind_textdomain_codeset(GETTEXT_PACKAGE, "UTF-8").expect("Failed to set codeset");
    textdomain(GETTEXT_PACKAGE).expect("Failed to set text domain");

    // Load resources - try from multiple locations for development/installed builds
    // First check if JAIR_DATA_DIR environment variable is set (snap environment)
    let mut resource_paths = vec![];

    if let Ok(data_dir) = std::env::var("JAIR_DATA_DIR") {
        resource_paths.push(format!("{}/jair.gresource", data_dir));
    }

    resource_paths.extend([
        PKGDATADIR.to_owned() + "/jair.gresource",
        "data/jair.gresource".to_string(),
        "./data/jair.gresource".to_string(),
    ]);

    let mut resources_loaded = false;
    for path in &resource_paths {
        if let Ok(resources) = gio::Resource::load(path) {
            gio::resources_register(&resources);
            resources_loaded = true;
            break;
        }
    }

    if !resources_loaded {
        eprintln!("Warning: Could not load resources from any location");
    }

    // Create a new GtkApplication. The application manages our main loop,
    // application windows, integration with the window manager/compositor, and
    // desktop features such as file opening and single-instance applications.
    let app = JairApplication::new("codes.lsb.jair", &gio::ApplicationFlags::empty());

    // Run the application. This function will block until the application
    // exits. Upon return, we have our exit code to return to the shell. (This
    // is the code you see when you do `echo $?` after running a command in a
    // terminal.
    app.run()
}
