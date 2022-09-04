#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{
  api::shell, AboutMetadata, CustomMenuItem, Manager, Menu, MenuEntry, MenuItem, Submenu,
  SystemTray, SystemTrayEvent, SystemTrayMenu, WindowBuilder, WindowUrl,
};

fn main() {
  let ctx = tauri::generate_context!();

  let builder = tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![])
    .setup(|app| {
      let _window = WindowBuilder::new(app, "main", WindowUrl::default())
        .title("Giffy")
        .inner_size(800.0, 550.0)
        .min_inner_size(400.0, 200.0)
        .build()
        .expect("Unable to create window");
      Ok(())
    })
    .menu(Menu::with_items([
      #[cfg(target_os = "macos")]
      MenuEntry::Submenu(Submenu::new(
        &ctx.package_info().name,
        Menu::with_items([
          MenuItem::About(ctx.package_info().name.clone(), AboutMetadata::default()).into(),
          MenuItem::Separator.into(),
          MenuItem::Services.into(),
          MenuItem::Separator.into(),
          MenuItem::Hide.into(),
          MenuItem::HideOthers.into(),
          MenuItem::ShowAll.into(),
          MenuItem::Separator.into(),
          MenuItem::Quit.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "File",
        Menu::with_items([MenuItem::CloseWindow.into()]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Edit",
        Menu::with_items([
          MenuItem::Undo.into(),
          MenuItem::Redo.into(),
          MenuItem::Separator.into(),
          MenuItem::Cut.into(),
          MenuItem::Copy.into(),
          MenuItem::Paste.into(),
          MenuItem::SelectAll.into(),
        ]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "View",
        Menu::with_items([MenuItem::EnterFullScreen.into()]),
      )),
      MenuEntry::Submenu(Submenu::new(
        "Window",
        Menu::with_items([MenuItem::Minimize.into(), MenuItem::Zoom.into()]),
      )),
      // You should always have a Help menu on macOS because it will automatically
      // show a menu search field
      MenuEntry::Submenu(Submenu::new(
        "Help",
        Menu::with_items([CustomMenuItem::new("Learn More", "Learn More").into()]),
      )),
    ]))
    .on_menu_event(|event| {
      let event_name = event.menu_item_id();
      match event_name {
        "Learn More" => {
          let url = "https://github.com/AkashRajpurohit/giffy".to_string();
          shell::open(&event.window().shell_scope(), url, None).unwrap();
        }
        _ => {}
      }
    })
    .system_tray(SystemTray::new().with_menu(SystemTrayMenu::new()))
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::LeftClick {
        position: _,
        size: _,
        ..
      } => {
        let app_handle = app.clone();
        let window = app_handle.get_window("main").unwrap();

        if window.is_visible().unwrap() {
          window.hide().unwrap();
        } else {
          window.show().unwrap();
          window.set_focus().unwrap();
        };
      }
      _ => {}
    });

  #[allow(unused_variables)]
  let app = builder
    .invoke_handler(tauri::generate_handler![])
    .run(ctx)
    .expect("error while building tauri application");
  
  // Currently this does prevent the app to quit but the system tray click
  // causes panic for finding the main window.
  // #[allow(unused_variables)]
  // app.run(move |app_handle, e| {
  //   if let RunEvent::ExitRequested { api, .. } = &e {
  //     // Keep the event loop running even if all windows are closed
  //     // This allow us to catch system tray events when there is no window
  //     api.prevent_exit();
  //   }
  // });
}
