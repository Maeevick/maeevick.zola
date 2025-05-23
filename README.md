# maeevick.com

![Build and Deploy](https://github.com/Maeevick/maeevick.github.io/workflows/build_and_deploy/badge.svg)
![Grimble Running Build](https://github.com/Maeevick/maeevick.github.io/workflows/Build%20and%20Deploy%20Grimble%20Running%20Mini%20Game/badge.svg)
![License](https://img.shields.io/badge/License-MIT%20%2B%20CC%20BY--NC--SA%204.0-blue.svg)

Welcome to the repository of my personal creative hub! This project aims to build a unique fantasy-steampunk universe and aesthetics, my universe and hobby, with technical experiments, game development and creative writing.

https://www.maeevick.com

## TL;DR

The website is structured around four main sections:

- **The Tavern** - Home page and general purpose stuff
- **The Quests** - Games and software products/projects
- **The Codex of Chaos** - Lore, stories and universe building
- **The Trix's Lab** - Technical experiments and blog posts (almost failures, I swear!)

This repository contains both the technical implementation of the website and its creative content, each under different licenses.

## 🚀 Getting Started

_"Ah, someone else wants to tinker with my contraptions? Splendid! Though I must warn you, my workshop has a tendency to... explode. Occasionally. But that's part of the charm!"_ - Trix

### Prerequisites

Before diving into this arcano-technological adventure, you'll need:

- **Zola** (v0.20.0+) - The magical static site generator that rarely combusts
- **Basic HTTP Server** - For local testing (or any web server that doesn't judge my code)
- **A sense of humor** - Essential for understanding why everything works despite appearing chaotic

### Quick Start

```bash
# Clone this repository
git clone https://github.com/Maeevick/maeevick.github.io.git
cd maeevick.github.io/app

# Serve locally
zola serve

# Your Ethereal Plane portal will open at http://127.0.0.1:1111
```

### Development Commands

```bash
# Build for production
zola build

# Check for compilation issues
zola check
```

## 🧪 Trix's Lab (Tech Corner of this Website)

_"Welcome to my lab, fellow tinkerers! This is where I experiment with arcano-technological contraptions powering this very pocket of the ethereal plane you're currently visiting. Some call it 'web development' — I call it 'practical dimensional engineering with minimal explosions!'_

_As any proper apprentice knows, documenting your experiments is crucial — especially the failures. Perhaps EVEN MORE the failures! So here's my ever-expanding catalog of tools and techniques I'm employing in this grand experiment."_

### 🔧 Current Tech Stack Checklist

#### Core Infrastructure

- [x] **Zola** - Static site generator crafted in Rust (v0.20.0) - _"A marvelously efficient contraption that transmutes my markdown scribblings into browser-compatible enchantments! And, as almost all rust-based tools, it rarely explodes, which is always cool."_
- [x] **SASS/SCSS** - For styling sorcery (_for the moment_)
- [x] **Responsive Design** - Works on crystal balls of all sizes (_I would like to believe it_)
- [x] **GitHub Actions** - Automated sparkle launcher

#### Interactive Elements

- [x] **CSS-only Hamburger Menu and Pseudo-Carousel** - No JavaScript required, pure CSS wizardry
- [x] **Multi-language Support** - Français/English switching (_thanks Zola_)
- [x] **WASM Games Integration** - For when words aren't enough

#### Games & Interactive Content

- [x] **Rust + Bevy Engine** - For creating digital adventures
- [x] **WebAssembly (WASM)** - Browser-compatible game magic
- [x] **Scaleway Object Storage** - Cloud deployment for interactive content

### Architecture Overview

The site follows a hybrid approach:

- **Static Generation** with Zola for lightning-fast loading and SEO
- **Dynamic Elements with WASM** deployed independently for not-so-heavy interactivity

For detailed technical deep-dives, brewing recipes, and explosion reports, visit [The Trix's Lab](https://www.maeevick.com/lab/) where I document my trials, explorations, and occasional triumphs.

### Future Enhancements

_"This is merely the beginning of our arcano-technological journey! I've got plans for integrating HTMX enchantments, more WebAssembly constructs, and possibly some mechanical automations with Actix-web... assuming my workshop doesn't catch fire again. However, it's too soon! So now, let me hand over to our Chief Archivist..."_

## 📚 Codex of Chaos (The Grimble's legendary library)

Mixing legends, lores and novels with a lot of other things, the Big Fat Codex belonging to Grimble is always ready to be augmented.

### Contributing (Or: How to Get Your Name in Grimble's Big Book)

_"Ah, interested in contributing to this... organized chaos? How delightfully unexpected! As the self-proclaimed Keeper of All Things Properly Filed (despite what certain apprentices say about my organizational methods), I have established some VERY IMPORTANT protocols."_ - Grimble

**🚧 Under Construction 🚧**

_"Please note that my contribution guidelines are currently being meticulously compiled, cross-referenced, and alphabetized. This process may take anywhere from several days to several centuries, depending on how many times Trix's experiments interrupt my cataloging."_

### What I'm Looking For

- **Technical Improvements** - Better spells, more efficient enchantments
- **Bug Reports** - Help me identify which "features" are actually "problems"
- **Suggestions** - New ideas for the ever-expanding universe
- **Advices** - About my Non-Expertise and my Eternal-Learning of Rust

### Current Process

1. **Open an issue** - _"Begin by properly documenting your suggestion in our Official Complaint... er, Issue Registry"_
2. **Discussion** - _"We shall debate its merits using civilized discourse (tea optional but recommended)"_
3. **Pull Request** - _"If your proposal survives my rigorous filing system, you may submit your changes"_
4. **Immortalization** - _"Successful contributors will be forever commemorated in the [Codex of Chaos](https://www.maeevick.com/codex/), alongside other legendary figures and mildly interesting footnotes"_
5. **Not ready to open an issue publicly?** Find me on [LinkedIn](https://www.linkedin.com/in/aurel-estoup/) or [BlueSky](https://bsky.app/profile/maeevick.bsky.social)

### Important Note

_"This being a rather... personal digital grimoire, I reserve the right to be quite selective about modifications. Think of it as 'publicly intimate' - open for all to see, but the content reflects my peculiar mind palace. Don't take rejections personally; blame my organizational standards."_

## License

This project uses a dual licensing approach to separate the technical implementation from the creative content.

### Software and Technical Components

All code, templates, configuration files, and technical documentation are licensed under the [MIT License](LICENSE).

This means you are free to:

- Use the technical implementation for any purpose, including commercial
- Modify, adapt, and build upon the codebase
- Distribute your modifications under any license of your choice

### Creative Content

All creative content including but not limited to texts, stories, character descriptions, universe lore, artwork, and images are licensed under the [Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License](LICENSE).

For more details, see the [CC BY-NC-SA 4.0](https://creativecommons.org/licenses/by-nc-sa/4.0/) license summary.

### Practical Examples

To clarify the dual licensing:

✅ **You can**:

- Use the Zola templates, HTMX implementation, and CSS structure for your own projects
- Fork the repository to create your own website with similar technical capabilities
- Create fan fiction or derivative works based on the universe, if proper attribution is given and it's not commercial

❌ **You cannot**:

- Use the characters Trix or Grimble in a commercial project without permission
- Republish the Codex of Chaos content verbatim without attribution
- Create commercial works based on the creative content

If you have questions about usage not covered here, please open an issue or contact me (see just above).
