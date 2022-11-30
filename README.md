# TUI Utils: Handy utils for building TUIs

This crate builds on top of [tui](https://docs.rs/tui/latest/tui/) which is used for some of my applications.
I've noticed that I repeat a lot of the same logic when implementing, for example, lists, creating blocks,
making components, and more. So I'm developing this crate for that reason, to be able to collect and re-use a lot of the
common stuff.

## Current utils

- [component](./src/component.rs)
    - `Component`: trait that defines methods for drawing and input handling
- [blocks](./src/blocks.rs)
    - `Dim`: trait that allows for dimming the blocks
    - `bold_block`: creates a block with bold borders and a color of choice
    - `default_block`: create a default block with all borders and a color of choice
    - `block_constructor`: finer control over the style, also allows for skipping borders
- [keys](./src/keys.rs)
    - `SharedKeys`: Allows for creating a `Rc` around your keymap
      which is cheaper to clone and delegate between components since you're only
      passing around pointers to the same underlying value.
    - `Keybind`: Meant to be used to define your keymap. `Keybind` can be compared
      to `crossterm::event::KeyEvent` which is useful for input handling. `Keybind`
      also implements `ToString`, more info on that can be found in the module.
    - `key_match`: helper to compare a `crossterm::event::KeyEvent` with a `Keybind`.
- [rect](./src/rect.rs)
    - `centered_rect`: creates a centered `Rect` that is half the width and height of the
      original `Rect`.
- [split](./src/split.rs)
    - `Ratio`: Tuple struct used to define split ratios.
    - `v_split`: creates a vertically split view within a given `Rect` with a given `Ratio`.
    - `h_split`: creates a horizontally split view within a given `Rect` with a given `Ratio`.
- [state](./src/state.rs)
    - `Boundary`: Tuple struct used to define a boundary for a `BoundedState`. This implements
      `From<&Vec<T>>` which allows you to generate a boundary from a arbitrary vector which is
      helpful for when the vector changes in size.
    - `BoundedState`: A bounds checked wrapper around `ListState`. This allows for very easy
      state management while also being able to access the underlying state for rendering.
- [term](./src/term.rs)
    - `init`: Initializes the terminal like you normally would.
    - `restore`: Restores the terminal like you normally would.
    - `restore_with_err`: Restores the terminal and prints a given error.
