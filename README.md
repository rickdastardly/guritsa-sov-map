# Gurista - Sov Map

**A live EVE Online sovereignty map for Windows.** Every sov holding, ADM and sov fight in New Eden, plus live kills, battles, gatecamps, activity, route and jump planning, killboard, intel tools and an intel-channel tracker, in a fast, lightweight desktop app.

![Gurista - Sov Map](assets/icon-256.png)

**[Download the latest release](https://github.com/rickdastardly/gurista-sov-map/releases/latest)** · Website version: [gurista.co.uk/sov-map](https://gurista.co.uk/sov-map/)

## What's in it

| | |
|---|---|
| **Sovereignty** | Territory by alliance or coalition, ADM, security, sov fights, changes, history playback, true in-game star positions |
| **Live** | Kills as they happen (zKillboard), kill flashes on the map, filters |
| **Combat** | Battles, gatecamps, PvP heat over 1h to 90 days |
| **Activity** | Jumps, ship kills, pod kills and NPC kills per system |
| **Route & jumps** | Gate routes (safer / shorter, avoid camps, Thera & Turnur), capital jump ranges and fuel |
| **Overlays** | Incursions, faction warfare, Thera/Turnur connections, industry index |
| **Killboard** | Top alliances, corps and pilots, biggest kills, Sov Index |
| **Intel** | Paste Local or d-scan for a threat read-out |
| **Intel channels** | Reads your EVE chat logs, pins reports on the map with age and jumps from you, and alerts you when hostiles are close |

## Install

1. Download **`GuristaSovMap-Setup-<version>.exe`** from [Releases](https://github.com/rickdastardly/gurista-sov-map/releases/latest) and run it. No admin rights needed.
   Prefer no installer? Download the `.zip`, unzip anywhere and run `GuristaSovMap.exe`.
2. Windows may say *"Windows protected your PC"* the first time: the app is new and isn't code-signed. Click **More info → Run anyway**.

**Needs:** Windows 10 or 11 (64-bit). That's it: the app has no runtime DLLs to install, and the installer sets up Microsoft Edge WebView2 (the engine that draws the map) by itself if your PC doesn't have it yet. The portable version opens Microsoft's download page for WebView2 if it's ever missing.

The app keeps itself up to date: it checks a tiny file every 6 hours and offers a one-click update when there's a new version. Map data comes from [gurista.co.uk](https://gurista.co.uk/sov-map/), so new map features arrive without updating the app at all.

## Privacy

No account, no tracking. The app talks to gurista.co.uk (map data) and CCP's public ESI API. Chat logs are read on your PC only and never sent anywhere.

## Verify your download

Each release lists SHA-256 hashes. In PowerShell:

```powershell
Get-FileHash .\GuristaSovMap-Setup-0.2.5.exe
```

## Build it yourself

Needs [Rust](https://rustup.rs) (stable) and, for the installer, [Inno Setup 6](https://jrsoftware.org/isinfo.php) (the script installs it with winget if missing).

```powershell
powershell -ExecutionPolicy Bypass -File .\build.ps1
```

Output in `dist\`: the portable `GuristaSovMap.exe`, the installer, and `dist\app\` (the files the website serves for self-update).

| File | What |
|---|---|
| `src/main.rs` | Window (tao) + Edge WebView2 (wry), window state, links, updates |
| `src/desktop.js` | Script injected into the map: hides web install prompts, update prompt, intel bridge |
| `src/chatlogs.rs` | Tails EVE chat logs for the intel-channel tracker |
| `src/update.rs` | Self-update from gurista.co.uk/sov-map/app/latest.json |
| `installer.iss` | Inno Setup installer |

## Legal

EVE Online and the EVE logo are the registered trademarks of CCP hf. All rights reserved. Used with permission. This project is not affiliated with CCP. Code under the [MIT licence](LICENSE).
