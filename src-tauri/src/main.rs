// This file is part of Monkeytype Desktop <https://github.com/Fuwn/monkeytype-desktop>.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, version 3.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.
//
// Copyright (C) 2022-2022 Fuwn <contact@fuwn.me>
// SPDX-License-Identifier: GPL-3.0-only

#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
#![deny(
  warnings,
  nonstandard_style,
  unused,
  future_incompatible,
  rust_2018_idioms,
  unsafe_code,
  clippy::all,
  clippy::nursery,
  clippy::pedantic
)]
#![recursion_limit = "128"]

// #[tauri::command]
// fn greet(name: &str) -> String {
//   format!("Hello, {}! You've been greeted from Rust!", name)
// }

use discord_rich_presence::{activity, DiscordIpc};
use tauri::Manager;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut client =
    discord_rich_presence::DiscordIpcClient::new("722389325483999243")?;

  client.connect()?;
  client.set_activity(
    activity::Activity::new()
      .details("Typing on Monkeytype")
      .state("Monkeytype Desktop")
      .assets(
        activity::Assets::new()
          .large_image("large")
          .large_text("Monkeytype Desktop")
          .small_image("icon")
          .small_text("https://github.com/Fuwn/monkeytype-desktop"),
      ),
  )?;
  tauri::Builder::default()
    // .invoke_handler(tauri::generate_handler![greet])
    .setup(|app| {
      app.get_window("main").unwrap().eval("window.location.replace('https://monkeytype.com')").unwrap();

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  client.close()?;

  Ok(())
}
