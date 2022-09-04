# Giffy

Desktop application that lets you find GIFs on the web easily, without having to hunt for them manually.

## Build With

- **Tauri**
- **GitHub action** for cross-platform builds
- **Svelte**
- **Vite**
- **TypeScript**
- **`svelte-preprocess`** with Sass installed by default

## Credits
- Icon by [flaticon](https://www.flaticon.com/authors/anggara)

## Dev instructions

### Get started

1. Install Node.js
2. Install Rust
3. Follow the [Tauri setup guide](https://tauri.studio/en/docs/get-started/intro)
4. Run `npm install`
5. Find and replace the text `tauri-template` and `Tauri Template`.

### Commands
- `npm run dev`: Start app in dev mode
- `npm run build`: Build
- `npm run lint`: Lint
- `npm run format`: Format

### Release new version
1. Update `CHANGELOG.md`
2. Bump the version number in `src-tauri/Cargo.toml`
3. Run `cargo check` to update `Cargo.lock`
4. Create a git tag in the format `v#.#.#`
5. Add release notes to the generated GitHub release and publish it
