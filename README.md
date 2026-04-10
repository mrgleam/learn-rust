# Rust Introduction Slides 🦀

This repository contains a premium Marp-based slide deck that summarizes the "What is Rust?" topic from Google's Comprehensive Rust course.

## 🚀 How to Present

### Option 1: VS Code (Recommended)
1. Install the **[Marp for VS Code](https://marketplace.visualstudio.com/items?itemName=marp-team.marp-vscode)** extension.
2. Open `rust-intro.md`.
3. Click the **Marp icon** (a slide icon) in the top-right corner of the editor to open the preview.
4. To enter full-screen presentation mode, use the Marp preview toolbar or press `F11`.

### Option 2: Makefile (Quickest for CLI users)
If you have `make` installed, you can use the provided Makefile:
- `make present`: Launch the presentation in preview mode.
- `make pdf`: Export slides to PDF (saved in `/dist`).
- `make html`: Export slides to HTML (saved in `/dist`).
- `make pptx`: Export slides to PowerPoint (saved in `/dist`).

### Option 3: Marp CLI (Manual)
You can use `npx` to run the Marp CLI and start a preview server:

```bash
npx @marp-team/marp-cli rust-intro.md --preview
```

### Option 4: Export to PDF/PowerPoint/HTML
If you have the Marp CLI or VS Code extension, you can export the slides:
- **CLI**: `npx @marp-team/marp-cli rust-intro.md --pdf`
- **VS Code**: Click the Marp icon in the editor, and select **Export Slide Deck...**

## 🎙️ Speaker Notes
The slides include speaker notes for key sections (specifically the "Why Rust?" comparison). 
- In the **VS Code Preview**, you can see notes by enabling the "Show speaker notes" option in the Marp preview settings.
- In **CLI Preview Mode**, speaker notes are visible in the presenter view (accessible via the browser interface).

## 📁 Structure
- `rust-intro.md`: The main slide deck.
- `assets/`: Contains images used in the slides (e.g., the premium cover logo).

---
*Created with ❤️ by Antigravity*
