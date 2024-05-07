<!-- Improved compatibility of back to top link: See: https://github.com/othneildrew/Best-README-Template/pull/73 -->
<a name="readme-top"></a>
<!--
*** Thanks for checking out the Best-README-Template. If you have a suggestion
*** that would make this better, please fork the repo and create a pull request
*** or simply open an issue with the tag "enhancement".
*** Don't forget to give the project a star!
*** Thanks again! Now go create something AMAZING! :D
-->
<!-- PROJECT LOGO -->
<br />
<div align="center">
  <h4>Sanspointes</h4>
  <h3 align="center">Bevy Changeset</h3>

  <p align="center">
    Component reflection based undo/redo system for bevy that mirrors the Commands API.
    <!-- <br /> -->
    <!-- <a href="https://github.com/othneildrew/Best-README-Template"><strong>Explore the docs »</strong></a> -->
    <!-- <br /> -->
    <br />
    <a href="https://github.com/sanspointes/bevy-wasm-api/issues">Report Bug</a>
    ·
    <a href="https://github.com/sanspointesbevy-wasm-api/issues">Request Feature</a>
  </p>
</div>

## Bevy Spts Changeset

This is a plugin for the Bevy engine that makes it easy to implement an undo / redo 
system or simply defer and propagate commands.

> :warning: This crate is under active development and shouldn't be used as a dependency just yet.
> Take a look at the roadmap section below to see what needs to be done before sharing.

```rust

pub struct MyChangesetTag;
fn build(app: &mut App) {
    let changeset_resource = ChangesetResource::<MyChangesetTag>::new();
    app.insert_resource(changeset_resource);
}

fn update(world: &mut World) {
    // world.changeset() returns something similar to the bevy Commands api.
    let mut builder = world.changeset();
    let uid = builder
        .spawn((SpatialBundle::default(), MyComponent(0)))
        .insert(ExtraComponent(1))
        .uid(); // Unique ID component persists between undo/redos
    builder.entity(uid).remove::<MyComponent>();
    // builder.build() returns a `Changeset` object.
    let changeset = builder.build();

    ChangesetResource::<MyChangeset>::context_scope(&mut world, |world, cx| {
        // Applying the changeset to the world returns the inverse changeset
        let undo = changeset.apply(world, cx).unwrap();
        // Lookup entity by uid via a HashmapResource.
        let spawned_entity = uid.entity(world).unwrap();
        assert_eq!(world.get<MyComponent>(spawned_entity).unwrap(), MyComponent(0));
        // Undo 
        undo.apply(world, cx).unwrap();

        // Can no longer lookup entity.
        assert!(uid.entity(world).is_none())
    });
}
```

## Install

Run `cargo add bevy_spts_changeset` or add this line to your `Cargo.toml`
```
bevy_spts_changeset = "0.1"
```

## Features

- `insert`/`remove` components.
- `spawn`/`despawn` entities.
- `despawn_recursive` with working undo to respawn hierarchy graph.

## Faq

### "Why do I need to use the `Uid` component?"

I am not smart enough/don't have time right now to do entity id mapping and having a 
persistent `Uid` component suits my needs but I welcome any contribution (or even just
guidance) on removing this dependency.

### "Do I need `&mut World` to use this plugin"

Right now yes.  There's the possibility of making it a SystemParam, like Bevy's `Commands`,
and then emitting the inverse changeset in an event but I'm keeping it simple for now.  I can
add this behaviour once the roadmap is solidified.

### "Can I filter which components are copied when I `despawn()` or `despawn_recursive()`"

Yes, that's why we have to use `context_scope` on the `ChangesetResource`.

Setup your filter like so:
```rust
fn build(app: &mut App) {
    let filter = SceneFilter::default()
            .allow::<Transform>();

    let changeset_resource = ChangesetResource::<MyChangesetTag>::new().with_filter(filter);
}
```

And now when you call `despawn()` or `despawn_recursive()` on the 


