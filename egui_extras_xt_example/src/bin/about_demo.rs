use eframe::egui::{self, Style, Visuals};
use eframe::emath::vec2;
use egui_extras_xt::ui::hyperlink_with_icon::HyperlinkWithIcon;

struct PackageInfo {
    name: &'static str,
    version: &'static str,
    authors: &'static str,
    description: Option<&'static str>,
    homepage: Option<&'static str>,
    repository: Option<&'static str>,
    license: Option<&'static str>,
}

macro_rules! option_env_some {
    ( $x:expr ) => {
        match option_env!($x) {
            Some("") => None,
            opt => opt,
        }
    };
}

macro_rules! package_info {
    ( ) => {
        PackageInfo {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            authors: env!("CARGO_PKG_AUTHORS"),
            description: option_env_some!("CARGO_PKG_DESCRIPTION"),
            license: option_env_some!("CARGO_PKG_LICENSE"),
            homepage: option_env_some!("CARGO_PKG_HOMEPAGE"),
            repository: option_env_some!("CARGO_PKG_REPOSITORY"),
        }
    };
}

impl PackageInfo {
    fn authors(&self) -> impl Iterator<Item = (&'static str, Option<&'static str>)> {
        self.authors.split(':').map(|author_line| {
            let author_parts = author_line
                .split(|c| ['<', '>'].contains(&c))
                .map(str::trim)
                .collect::<Vec<_>>();
            (author_parts[0], author_parts.get(1).cloned())
        })
    }
}

struct AboutDemoApp {
    package_info: PackageInfo,
    about_open: bool,
}

impl Default for AboutDemoApp {
    fn default() -> Self {
        Self {
            package_info: package_info!(),
            about_open: false,
        }
    }
}

impl eframe::App for AboutDemoApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("About").clicked() {
                self.about_open = true;
            }
        });

        egui::Window::new("About")
            .open(&mut self.about_open)
            .show(ctx, |ui| {
                ui.heading(self.package_info.name);
                ui.label(format!("Version {}", self.package_info.version));

                ui.separator();

                if let Some(description) = self.package_info.description {
                    ui.label(description);
                    ui.separator();
                }

                ui.horizontal(|ui| {
                    if let Some(homepage) = self.package_info.homepage {
                        ui.hyperlink_with_icon_to("Home page", homepage);
                    }

                    if let Some(repository) = self.package_info.repository {
                        ui.hyperlink_with_icon_to("Repository", repository);
                    }
                });

                ui.separator();

                ui.collapsing("Authors", |ui| {
                    ui.horizontal(|ui| {
                        for (author_name, author_email) in self.package_info.authors() {
                            if let Some(author_email) = author_email {
                                if !["noreply@", "no-reply@", "@users.noreply."]
                                    .iter()
                                    .any(|no_reply| author_email.contains(no_reply))
                                {
                                    ui.hyperlink_with_icon_to(
                                        author_name,
                                        format!("mailto:{author_email:}"),
                                    );
                                } else {
                                    ui.label(author_name);
                                }
                            } else {
                                ui.label(author_name);
                            }
                        }
                    });

                    // (!) Rust incremental compilation bug:
                    // When the 'license' field is changed in the crate's Cargo.toml,
                    // source files that include that field through `env!()` macros
                    // are not picked up for recompilation.
                    // Always do `cargo clean` + full rebuild when changing Cargo.toml metadata.
                    if let Some(license) = &self.package_info.license {
                        ui.separator();
                        ui.label(format!("License: {license:}"));
                    };
                });
            });
    }
}

fn main() {
    // TODO: Move to egui_extras_xt/src/ui as a reusable component.

    let options = eframe::NativeOptions {
        initial_window_size: Some(vec2(580.0, 680.0)),
        ..Default::default()
    };

    eframe::run_native(
        "About demo",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_style(Style {
                visuals: Visuals::dark(),
                ..Style::default()
            });

            Box::new(AboutDemoApp::default())
        }),
    );
}