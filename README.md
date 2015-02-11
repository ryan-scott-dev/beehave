# Beehave

A simple library for defining and executing behaviour trees.

## Example

```rust
let tree_behaviour: Selector<Tree> = behaviour_selector!("Tree Root", [
    behaviour_sequence!("Photosynthesise", [
        condition!("Ensure Can Make Energy",
            |tree: &mut Tree| {
                tree.can_make_energy()
            },
            action!("Make Energy", |tree: &mut Tree| {
                tree.make_energy();
                Result::Success
            })
        ),
        condition!("Ensure Can Grow",
            |tree: &mut Tree| {
                tree.can_grow()
            },
            action!("Grow", |tree: &mut Tree| {
                tree.grow();
                Result::Success
            })
        ),
        condition!("Ensure Can Emit Oxygen",
            |tree: &mut Tree| {
                tree.can_emit_oxygen()
            },
            action!("Emit Oxygen", |tree: &mut Tree| {
                tree.emit_oxygen();
                Result::Success
            })
        )
    ]),
    condition!("Ensure Can Gather Sun",
        |tree: &mut Tree| {
            tree.can_gather_sun()
        },
        action!("Emit Oxygen", |tree: &mut Tree| {
            tree.gather_sun();
            Result::Success
        })
    ),
    condition!("Ensure Can Gather Water",
        |tree: &mut Tree| {
            tree.can_gather_water()
        },
        action!("Emit Oxygen", |tree: &mut Tree| {
            tree.gather_water();
            Result::Success
        })
    )
]);
```

[See More](./example/main.rs)

## See also

[PistonDevelopers/ai_behavior](https://github.com/PistonDevelopers/ai_behavior)

## License

Component Experiment was created by [Ryan Scott](http://github.com/archytaus) is distributed under the [MIT](http://ryanscott.mit-license.org) license.
