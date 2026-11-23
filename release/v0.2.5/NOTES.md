## Gurista - Sov Map v0.2.5: the official desktop release

A live EVE Online sovereignty map for Windows. It has everything on [gurista.co.uk/sov-map](https://gurista.co.uk/sov-map/), in its own window, plus things a browser tab can't do.

### ⬇ Download
| File | What |
|---|---|
| **GuristaSovMap-Setup-0.2.5.exe** | **The one most people want.** Installs the app, a Start menu entry, an optional desktop shortcut and an uninstaller. It also installs Microsoft Edge WebView2 for you if your PC doesn't have it. No admin rights needed. |
| Gurista-Sov-Map-0.2.5-windows.zip | Portable: unzip anywhere and run `GuristaSovMap.exe`. |
| GuristaSovMap-0.2.5.exe | The portable app on its own. |
| SHA256SUMS.txt | Checksums. In PowerShell, `Get-FileHash .\GuristaSovMap-Setup-0.2.5.exe` should match. |

**Needs:** Windows 10 or 11, 64-bit. That's all.
**First launch:** Windows may say *"Windows protected your PC"* because the app is new and not code-signed. Click **More info → Run anyway**.

---

## Patch notes

### 🗺 The map
- **True in-game layout:** systems now sit at their real star positions, exactly like the in-game map. If you prefer the old layout, which pushes stacked systems apart, untick **In-game layout** in the bottom bar.
- **Sov territory covers the right space:** colour now follows the stargates between an alliance's own systems. Unclaimed null-sec, empty space between regions, and high-sec or NPC space stay dark. Before, colour bled across the gaps.
- **Coalitions:** a new **Coalition** colour mode. Membership comes from the verite.space coalition map, adjusted by hand where needed. There's a Coalitions tab with system counts, a coalition detail page showing members and the fights they're defending, coalition names on the map, and coalitions in search.
- **Gurista logo:** clicking it (top left) reloads the map.

### 📡 Intel channels (new)
- Open **Comms** on the left. The app reads your EVE chat logs and pins every report from your intel channels on the map, showing how long ago it was posted and how many jumps it is from you.
- **Your location is automatic:** it's picked up from your Local chat log every time you jump. If you multibox, you can choose which character to follow.
- **Reports it understands:** system names, short forms like "1DQ", **clr** (shown in green), **nv** (shown in gold) and status requests.
- **Alerts:** when hostiles are reported within your alert range (5 jumps unless you change it), you get a sound, a red banner, and the taskbar flashes if the app is in the background.
- **Channel setup:** channels with "intel" or ".imperium" in the name are ticked for you; tick or untick any others. You can set the alert range and how long reports stay on the map, and paste a report by hand.
- **Private:** your chat logs are read on your PC and never sent anywhere.

### 📊 Activity
- **Ship and pod kills are easy to see now:** they get solid markers with a glow, sized by how busy a system is compared to the others. Before, one 500-kill fight made every other system nearly invisible.
- **Honest time windows:** CCP only publishes the last hour of activity, so the longer windows (24h to 90 days) fill up from when tracking started. Windows that aren't full yet show a dashed outline, and the panel tells you exactly when each one will be complete.
- **Clearer labels:** the buttons now read Jumps / Ships / Pods / NPC.

### 🖥 The desktop app
- **Rebuilt as an exact copy of the website:** every map feature lands in the app the moment it ships on the site, with nothing to download.
- **Updates itself:** it checks a tiny (~300 byte) file every 6 hours. When a new version is out, an **Update** prompt appears; one click downloads it, swaps it in and restarts. Your settings stay. If the app can't replace itself (for example when installed for all users), the prompt links to the download page instead.
- **Desktop touches:**
  - dark window frame
  - remembers its size, position and maximised state
  - **F11** fullscreen, **F5** reload
  - outside links (zKillboard, Dotlan, gurista.co.uk) open in your normal browser
  - works offline from cache
  - install prompts from the website are hidden inside the app
- **Runs on any Windows 10/11 PC:** the app no longer needs the Visual C++ runtime ("VCRUNTIME140.dll was not found" is gone), and the installer sets up WebView2 by itself if it's missing.

### 🌐 Website
- **New landing choice:** Windows visitors get a one-time choice between *Open the Sov Map* in the browser and *Download for Windows*, showing the version and size. Nobody is forced; the **Get the app** button stays in the top bar for later.
- **Download page:** now serves releases straight from gurista.co.uk, with an installer button, a portable `.exe`, and checksums.
- The in-game layout, coalitions, territory, activity and intel-channel changes are all on the website too. Intel channels work in Chrome and Edge after a one-time "choose your Chatlogs folder".

---

**Privacy:** no account, no tracking. The app talks only to gurista.co.uk (map data) and CCP's public ESI API.

*EVE Online and the EVE logo are the registered trademarks of CCP hf. All rights reserved. Used with permission. Not affiliated with CCP.*
