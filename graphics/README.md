# graphics

Graphics library for rendering a radar display using [embedded_graphics](https://docs.rs/embedded-graphics/latest/embedded_graphics/)

Rendering can be re-used between the firmware and simulator (draw functions are generic over embedded_graphics `DrawTarget` so will render on any device that implements the `DrawTarget` trait)
