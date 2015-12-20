# Beehave [![Build Status](https://travis-ci.org/Archytaus/beehave.svg)](https://travis-ci.org/Archytaus/beehave)

A simple library for A simple library for defining and evaluating a hierarchical state machine (behaviour tree).

[Documentation](https://archytaus.github.io/beehave/beehave)

## Compatibility

Because this library uses `fn_traits` and `unboxed_closures` it will only compile using rust nightly.

This was last tested against `rustc 1.7.0-nightly (8ad12c3e2 2015-12-19)`

## Example

Building a behaviour tree using the supplied macros:
```rust
let world_behaviour: Selector<World> = behaviour_selector!("World Root", [
    condition!("Ensure Can Shine",
        |world: &mut World| {
            world.can_shine()
        },
        action!("Cycle Day/Night", |world: &mut World| {
            world.toggle_sun()
        })
    ),
    condition!("Ensure Can Rain",
        |world: &mut World| {
            world.can_rain()
        },
        action!("Rain", |world: &mut World| {
            world.rain()
        })
    )
]);

let tree_behaviour: Selector<Tree> = behaviour_selector!("Tree Root", [
    behaviour_sequence!("Photosynthesise", [
        condition!("Ensure Can Make Energy",
            |tree: &mut Tree| {
                tree.can_make_energy()
            },
            action!("Make Energy", |tree: &mut Tree| {
                tree.make_energy()
            })
        ),
        condition!("Ensure Can Grow",
            |tree: &mut Tree| {
                tree.can_grow()
            },
            action!("Grow", |tree: &mut Tree| {
                tree.grow()
            })
        ),
        condition!("Ensure Can Emit Oxygen",
            |tree: &mut Tree| {
                tree.can_emit_oxygen()
            },
            action!("Emit Oxygen", |tree: &mut Tree| {
                tree.emit_oxygen()
            })
        )
    ]),
    condition!("Ensure Can Gather Sun",
        |tree: &mut Tree| {
            tree.can_gather_sun()
        },
        action!("Emit Oxygen", |tree: &mut Tree| {
            tree.gather_sun()
        })
    ),
    condition!("Ensure Can Gather Water",
        |tree: &mut Tree| {
            tree.can_gather_water()
        },
        action!("Emit Oxygen", |tree: &mut Tree| {
            tree.gather_water()
        })
    )
]);
```

Evaluating the behaviour tree against an actor:
```rust
tree_behaviour.evaluate(&mut tree);
world_behaviour.evaluate(&mut world);
```

For more information please see the [full example](./example).

## Design Goals

- Easily generate an immutable behaviour tree
- Evaluate the behaviour tree on a mutable actor

## See also

[PistonDevelopers/ai_behavior](https://github.com/PistonDevelopers/ai_behavior)

## License

Component Experiment was created by [Ryan Scott](http://github.com/archytaus) is distributed under the [MIT](http://ryanscott.mit-license.org) license.
