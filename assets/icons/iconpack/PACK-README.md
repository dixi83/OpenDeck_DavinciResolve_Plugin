# 🎨 DaVinci Resolve Nodes - Icon Pack for Ulanzi Studio

<p align="center">
  <img src="DaVinci Resolve Nodes Icon Pack.jpeg" alt="DaVinci Resolve Nodes Icon Pack" width="100%">
</p>

![Ulanzi Studio](https://img.shields.io/badge/Ulanzi%20Studio-Compatible-blue?style=for-the-badge)
![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)
![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20macOS-lightgrey?style=for-the-badge)

A detailed, command-accurate icon pack (`.png`) designed for **Ulanzi Studio** and **Ulanzi Deck** users.

Trigger DaVinci Resolve's most-used editing, color and node commands straight from your deck: cuts, transitions, scopes, node trees and page switching — all four pages, one deck.

---

## ✨ Features

- 🎨 **High Quality & Optimized**
  - Crisp PNG icons tailored for Ulanzi Deck displays.

- 🎨 **DaVinci Resolve-Accurate Design**
  - Icons that mirror Resolve's own Edit, Color, Fusion and Deliver page commands.

- ⚡ **Automated Installers**
  - One-click installation for:
    - **Windows** (`.bat`)
    - **macOS** (`.command`)

- 📁 **Organized Structure**
  - Automatically installs into a dedicated `DaVinci_Resolve_Nodes` folder inside your Ulanzi Studio icon library.

---

## 📁 Repository Structure

```text
DaVinci Resolve Nodes Icon Pack/                    (repo root)
├── DaVinci Resolve Nodes Icon Pack/                # Icons + manifest
│   ├── Soft_Home-044-DAVINCI-RESOLVE_Add_Serial_Node_ulanzi.png
│   ├── Soft_Home-044-DAVINCI-RESOLVE_razor_ulanzi.png
│   ├── ...                                         # 70 icons total
│   └── manifest.json
├── DaVinci Resolve Nodes Icon Pack.jpeg
├── instalar_icones_ulanzi.bat
├── instalar_icones_ulanzi.command
└── README.md
```

> ⚠️ The **README, banner and installer scripts must stay at the repo root**, outside the `DaVinci Resolve Nodes Icon Pack/` folder. Only the icons and `manifest.json` belong inside it — that's the exact folder the installers copy from and the one Ulanzi Studio expects to import.

---

# 🚀 Installation

## 🖥️ Windows

1. Download or clone this repository.
2. Double-click **`instalar_icones_ulanzi.bat`** (at the repo root, next to the `DaVinci Resolve Nodes Icon Pack` folder).
3. Restart **Ulanzi Studio**.
4. Your icons will be available under:

```
Icons → DaVinci_Resolve_Nodes
```

---

## 🍎 macOS

1. Download or clone this repository.
2. Open **Terminal** and make the installer executable (if needed):

```bash
chmod +x instalar_icones_ulanzi.command
```

3. Double-click **`instalar_icones_ulanzi.command`** (at the repo root, next to the `DaVinci Resolve Nodes Icon Pack` folder).
4. Restart **Ulanzi Studio**.
5. Your icons will be available under:

```
Icons → DaVinci_Resolve_Nodes
```

---

# 🔧 Manual Installation

If you prefer to install the icons manually:

### Windows

```text
%APPDATA%\Ulanzi\UlanziDeck\Icons\DaVinci_Resolve_Nodes
```

### macOS

```text
~/Library/Application Support/Ulanzi/UlanziDeck/Icons/DaVinci_Resolve_Nodes
```

Copy everything from the inner `DaVinci Resolve Nodes Icon Pack/` folder (all `.png` files + `manifest.json`) into the destination folder above.

---

# 🎛️ Icon List

**70 icons**, grouped by workflow:

| Group | Icons |
|---|---|
| **Nodes & Color Page** | Add_Parallel_Mixer_Node, Add_Parallel_Node, Add_Serial_Node, add-layer-node, Next_Node, Previous_Node, Apply_Grade, PrimariesLog_Color_Wheels, color |
| **Scopes & Viewer** | SCOPES, Show_Scopes, Video_Scopes_On, Vectorscope, Waveform, Viewer_Overlay_On_Off, Zoom_Viewer_to_Fit, Safe_Area |
| **Edit Page & Trim** | Blade_Edit_Mode, Normal_Edit_Mode, Trim_Mode, Dynamic_Trim_Mode, Ripple_Cut, Ripple_Overwrite, Ripple_delete, Extend_Edit, Split_Clip, razor, slip, cut, copy, paste, edit, insert, overwrite, Slide |
| **Clips & Markers** | Join_Clips, Link_Clips, Mark_Out, mark_in, Go_to_In, Go_to_Out, Clear_In_and_Out, Grab_Still, add-keyframe |
| **Playback & Transport** | Play_Forward, Play_Reverse, Fast_Forward, Fast_Reverse, PauseStart_Playback, Loop, Freeze_Frame, stop |
| **Audio & Tracks** | Fairlight, Mute, Mute_Tracks, Solo_Tracks, audio |
| **Pages & Project** | Fusion, Media, gallery, deliver, New_Bin, New_Project, New_Timeline, Import_Media, Save_Project, Replace, Undo |
| **Transitions & Extras** | Add_Transition, Soft_Home-012-SOFTWARE_davinci-resolve (app launcher icon) |

---

# 💬 Community & Support

Found a bug or have an icon request?

- Open a GitHub **Issue**
- Share your setup with the Ulanzi community

---

# 📄 License

This project is released under the **MIT License**.

Feel free to use, modify, expand, and share it.

---

# ⭐ Support the Project

If you enjoy this icon pack, consider giving this repository a **⭐ Star** on GitHub.

It helps other Ulanzi users discover the project and supports future updates.
