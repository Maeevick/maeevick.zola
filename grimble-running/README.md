# Grimble Running

![Build Status](https://github.com/Maeevick/maeevick.github.io/workflows/Build%20and%20Deploy%20Grimble%20Running%20Mini%20Game/badge.svg)
![License](https://img.shields.io/badge/License-MIT-blue.svg)

_An arcano-technological contraption to gently tease our dear Library Assistant on his non-anniversary_

## 📚 The Origin Story

Well, well, well... You know how our beloved Grimble gets when he's organizing books? All that meticulous jumping from shelf to shelf, dodging falling tomes and avoiding explosions?

I thought: "What if we could recreate this thrilling daily routine in a more... controlled environment?"

So here we are! Grimble Running - a delightfully simple jumping game that perfectly captures our favorite goblin's extraordinary talent for avoiding obstacles (mostly the Trix's ones) while going about his very serious book-sorting business.

_Note: No actual goblins were harmed in the making of this game. Though Grimble did mutter something about "unnecessary digital representations of perfectly normal library activities" when he saw it._

_Note 2: It's literally inspired by 'A Life of a Dinosaur in a Web' (Google Chrome)_

## 🎮 Gameplay

**The Goal:** Help Grimble navigate through his daily obstacle course cleaning his library. [Play Now on the Website!](https://www.maeevick.com/quests/grimble-running/)

**Key points:**

- **Simple Controls**:
  I've kept the controls refreshingly simple because, let's face it, Grimble has enough complexity in his filing systems. No need to overwhelm the poor fellow with a 47-button gaming peripheral.
  - Press SPACE or CLICK/TOUCH to jump over obstacles _(just like dodging my brain distractions!)_
- **Progressive Difficulty**: The faster you go, the more it resembles a typical day in the library
  - More Chaos to come later _(exact release date: "undefined")_
- **Score System**: Count how many obstacles you avoid (_Grimble's personal record is still classified_)
  - Local persistance of your Highest Scores _(maybe)_ soon
- **Current Limitations:**
  - Rigid size not responsive at all _(turn your narrow mobile screen to landscape and aim right to center the game)_

## 🛠️ Technical Sorcery

_"It's not magic, it's just sufficiently advanced engineering that looks like magic to people who don't read enough technical grimoires."_ - Trix

Built with **Rust** and **Bevy Engine**, then transmuted into **WebAssembly** for maximum portability across the ethereal planes (and web browsers).

### 🤓 Development Commands

#### Prerequisites

Make sure you have the Rust toolchain installed, then add the WASM target:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install basic-http-server  # for serving locally (it can be any http server)
```

#### Development (Native)

For rapid iteration and testing:

```bash
# Run the game natively (fastest for development)
cargo run # or make run

# Build for native platforms
cargo build --release # or make release
```

#### WebAssembly Build

For web deployment:

```bash
# Build for WASM target
cargo build --target wasm32-unknown-unknown --profile wasm-release

# Generate JavaScript bindings
wasm-bindgen --target web --no-typescript --out-dir ./pkg --out-name grimble_running ./target/wasm32-unknown-unknown/release/grimble_running.wasm

# or (both in one command)
make wasm-release

# Serve locally for testing
basic-http-server .
# Then open http://127.0.0.1:4000
```

**Warning**: The current WASM bundle is... let's say "generously sized" (~45MB) for such a simple game. I'm working on convincing Bevy to be less enthusiastic about including every possible feature. Future optimizations planned!

**Development Integration:**
This project automatically deploys via GitHub Actions to Scaleway Object Storage and integrates seamlessly with the main website.

For more technical details on this automated sorcery or other technical stuff, visit [The Trix's Lab](https://www.maeevick.com/lab/) (you may take a look at the GitHub Action's workflow too).

#### Project Structure

```
grimble-running/
├── .cargo/
│   └── config.toml          # WASM build configuration
├── src/
│   ├── main.rs              # Native entry point
│   ├── lib.rs               # WASM entry point
│   └── game.rs              # Game logic
├── pkg/                     # Generated WASM files
├── index.html               # Game interface
├── Cargo.toml               # Dependencies and build config
└── Makefile                 # Build automation (a.k.a. The Magician)
```

### 📦 Features

- [x] Grimble _(a sophisticated gray square)_
- [x] Obstacles _(equally sophisticated gray - not the same - rectangles)_
- [x] Keyboard and Touch controls _(for desktop and mobile goblin management)_
- [x] Gravity simulation _(because physics matters!)_
- [x] Score tracking _(for competitive party organizing)_
- [x] Progressive speed increase _(like a day getting progressively more chaotic)_
- [x] Game over screen _(for when even Grimble can't keep up)_
- [x] WebAssembly support _(for universal goblin accessibility)_
- [x] Automated CI/CD deployment _(because manual deployment is so pre-digital age)_

### 🎯 Future Improvements

- [ ] More sophisticated sprites _(maybe Grimble could look like... a slightly better gray square?)_
- [ ] Sound effects _(the gentle **thud** of books being properly shelved)_
- [ ] Background scenery _(the Great Library, obviously...or not)_
- [ ] Different obstacle types _(flying books, unstable experiments, etc.)_
- [ ] High score persistence _(for Grimble's competitive streak)_

## 👋 Contributing (Or: How to Get Immortalized in Grimble's Chronicles)

_"While I appreciate the enthusiasm for improving my... representation in digital form, I must remind potential contributors that this is a rather personal project reflecting someone's peculiar interpretation of my daily routine."_ - Grimble

**🚧 Under Construction 🚧**

_"My contribution guidelines are currently being compiled in triplicate, with proper cross-references and color-coded annotations. This process requires approximately 47 different forms and may take some time."_

### Current Process

1. **Bug Reports** - If you spot a glitch in my humble-stupid-tiny-game, please document it properly
2. **Feature Suggestions** - Ideas welcome, though I reserve the right to file them under "Interesting but Impractical"
3. **Discussion** - Open an issue for civilized discourse

For everything above, open an issue it's the simplest way

**Successful contributors will be commemorated in the [Codex of Chaos](https://www.maeevick.com/codex/)** alongside other notable figures and curiosities!

## 📝 License

This project (as "SOFTWARE") is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

_Translation: Feel free to use this code to create your own academic procrastination tools or goblin-themed entertainment software._

## 🙏 Acknowledgments

- **Grimble** - For being an excellent (and unwitting) muse
- **The Bevy Community** - For making game development in Rust surprisingly non-explosive
- **The Rust Team** - For creating a language that rarely causes laboratory fires
- **WebAssembly** - For proving that magic is just science we haven't explained yet
- **GitHub Actions** - For automating deployment so I can focus on more important explosions
  - actions-rust-lang/setup-rust-toolchain
  - Swatinem/rust-cache
  - taiki-e/install-action
- **Scaleway Object Storage** - For keeping my creations in the cloud

---

_Built with_ ❤️ _in the Arcano-Technological Laboratory_  
_"When in doubt, add more rust. The programming language, not the corrosion."_ - Trix
