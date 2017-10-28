# ViXi
Vi-like editing with the Xi core, written in pure Rust

## Design

ViXi is written with the idea that Vi's mode switching and keystroke
remapping features can be managed in a separate library and dropped
into either the Xi core library or into any Xi frontend.

The strict separation of concerns implicit in this design is intended
to avoid a lot of the difficulties found in extending and maintaining
the Vim and Neovim codebases.

By convention, generic type names follow this pattern:
* `K` is a keystroke, implementing `Copy` and `Ord`.
* `T` is an arbitrary type, typically stored as a value in a map.
* `Op` is an arbitrary operation type (enums from `op.rs`).

## Todo

- [ ] Decide on an interface between ViXi and a ViXi object's owner.
- [ ] Implement MVP operations for Normal, Insert, and Op-pending.

