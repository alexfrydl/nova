# nova

https://gitlab.com/alexfrydl/nova

Nova is a game engine written in Rust and designed for 2D top-down adventure
games and RPGs.

The renderer is built with [gfx_hal][1] and works on all major desktop operating
systems.

Game state can be managed through an **Entity-Component-System** design, powered
by [specs][2], and also through **elements** which implement hierarchical state
and messaging similar to [Elm][4], [Yew][5], or [React][3].

[1]: https://github.com/gfx-rs/gfx
[2]: https://slide-rs.github.io/specs/
[3]: https://reactjs.org/
[4]: https://elm-lang.org/
[5]: https://github.com/DenisKolodin/yew

This is a work heavily in progress.

## Requirements

Nova requires Rust v1.34+ and either Vulkan, DirectX, or Metal development
libraries depending on your platform and what is bundled.
