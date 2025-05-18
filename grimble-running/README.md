# Grimble Running

_An arcano-technological contraption to gently tease our dear Library Assistant on his non-anniversary_

---

## 📚 The Origin Story

Well, well, well... You know how our beloved Grimble gets when he's organizing books? All that meticulous jumping from shelf to shelf, dodging falling tomes and avoiding the occasional explosion from my latest experiments?

I thought: "What if we could recreate this thrilling daily routine in a more... controlled environment?"

So here we are! Grimble Running - a delightfully simple jumping game that perfectly captures our favorite goblin's extraordinary talent for avoiding obstacles (mostly my fault, to be honest) while going about his very serious book-sorting business.

_Note: No actual goblins were harmed in the making of this game. Though Grimble did mutter something about "unnecessary digital representations of perfectly normal library activities" when he saw it._

_Note 2: It's literally inspired by 'A Life of a Dinosaur in a Web' (Google Chrome)_

## 🎮 What It Does

- **Simple Controls**: Press SPACE to jump over obstacles (just like dodging my failed contraptions!)
- **Progressive Difficulty**: The faster you go, the more it resembles a typical day in the lab
- **Score System**: Count how many obstacles you avoid (Grimble's personal record is still classified)
- **Game Over & Restart**: Because even Grimble needs a break sometimes (Press R to try again)

## 🧹 Technical Sorcery

Built with **Rust** and **Bevy Engine**, then transmuted into **WebAssembly** for maximum portability across the ethereal planes (and web browsers).

_"It's not magic, it's just sufficiently advanced engineering that looks like magic to people who don't read enough technical grimoires."_ - Trix

## 🛠️ Development Commands

### Prerequisites

Make sure you have the Rust toolchain installed, then add the WASM target:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
cargo install basic-http-server  # for serving locally
```

### Development (Native)

For rapid iteration and testing:

```bash
# Run the game natively (fastest for development)
cargo run # or make run


# Build for native platforms
cargo build --release # or make release
```

### WebAssembly Build

For web deployment:

```bash
# Build for WASM target
cargo build --target wasm32-unknown-unknown --profile wasm-release

# Generate JavaScript bindings
wasm-bindgen --target web --no-typescript --out-dir ./pkg --out-name grimble_running ./target/wasm32-unknown-unknown/release/grimble_running.wasm

# or (both in)
make wasm-release

# Serve locally for testing
basic-http-server .
# Then open http://127.0.0.1:4000
```

_Warning: The current WASM bundle is... let's say "generously sized" (~45MB) for such a simple game. I'm working on convincing Bevy to be less enthusiastic about including every possible feature. Future optimizations planned!_

### Project Structure

```
grimble-running/
├── .cargo/
│   └── config.toml          # WASM build configuration
├── src/
│   ├── main.rs              # Native entry point
│   └── lib.rs               # WASM entry point
│   ├── game.rs              # Game logic
├── pkg/                     # Generated WASM files
├── index.html               # Demo page
└── Cargo.toml               # Dependencies and build config
└── Makefile                 # The Magician
```

## 📦 Features

- [x] Grimble _(a sophisticated gray square)_
- [x] Obstacles _(equally sophisticated gray - not the same - rectangles)_
- [x] Gravity simulation _(because physics matters!)_
- [x] Score tracking _(for competitive race organizing)_
- [x] Progressive speed increase _(like a day getting progressively more chaotic)_
- [x] Game over screen _(for when even Grimble can't keep up)_
- [x] WebAssembly support _(for universal goblin accessibility)_

## 🎯 Future Improvements

- [ ] More sophisticated sprites _(maybe Grimble could look like... a slightly better gray square?)_
- [ ] Sound effects _(the gentle **thud** of books being properly shelved)_
- [ ] Background scenery _(the Great Library, obviously...or not)_
- [ ] Different obstacle types _(flying books, unstable experiments, etc.)_
- [ ] High score persistence _(for Grimble's competitive streak)_

## 📝 License

This project (as "SOFTWARE") is licensed under the MIT License - see the [LICENSE](../LICENSE) file for details.

_Translation: Feel free to use this code to create your own academic procrastination tools or goblin-themed entertainment software._

## 🙏 Acknowledgments

- **Grimble** - For being an excellent (and unwitting) muse
- **The Bevy Community** - For making game development in Rust surprisingly non-explosive
- **The Rust Team** - For creating a language that rarely causes laboratory fires
- **WebAssembly** - For proving that magic is just science we haven't explained yet

---

_Built with_ ❤️ _in the Arcano-Technological Laboratory_  
_"When in doubt, add more rust. The programming language, not the corrosion."_ - Trix
