fn game_exe(e: &str) -> &str {
  match e {
    "Phasmophobia" => "Phasmophobia.exe",
    _ => ""
  }
}

#[tauri::command]
pub fn game_launch(game: &str, server: &str) {
  use std::process::Command;

  let _cmd = Command::new(game_exe(game))
    .env("online-fix-launcher", std::process::id().to_string())
    .env("online-fix-game", game)
    .env("online-fix-photon", server)
    .spawn();
}