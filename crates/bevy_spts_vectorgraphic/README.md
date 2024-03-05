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
  <h3 align="center">Bevy (SPTS) VectorGraphic</h3>

  <p align="center">
    Bevy integration for lyon where the path state is represented in the ECS World.
    <!-- <br /> -->
    <!-- <a href="https://github.com/othneildrew/Best-README-Template"><strong>Explore the docs »</strong></a> -->
    <!-- <br /> -->
    <br />
    <a href="https://github.com/sanspointes/bevy-wasm-api/issues">Report Bug</a>
    ·
    <a href="https://github.com/sanspointesbevy-wasm-api/issues">Request Feature</a>
  </p>
</div>

> :warning: This is only uploaded for information sharing sake, and is not in a polished enough state to be relied
> on as an external dependency.  Feel free to use it but don't expect reliability until a crates.io release.

## How it works

This crate is a bevy plugin to help with creating vector graphics editors where the state of 
the vector graphic is represented via the ECS world.  This should make it easier to extend with constraints and other features.

### Entities / Hierarchy / Components

This crate requires a specific hierarchy of entities to work correctly.  You must create an entity with a `VectorGraphic` component
and add to it a number of `Endpoint` and `Edge` children.  Whenever there is a change, these children will be used to rebuild the path.

Endpoints represent a node in the vector graphic.  They have a `next_edge: Option<Entity>` and `prev_edge: Option<Entity>` field
which define the path.  Their position is derived from the `Transform` component (local coordinates from origin of parent).

Edges represent the lines/curves between endpoints.  They have a `next_endpoint: Entity` and `prev_endpoint: Entity` field 
which also define the path.  These must be kept in sync with the `Endpoint` components that they reference.  You can use 
the `spawn_edge` and `despawn_edge` extensions for `World` and `Commands` to make this more ergonomic.
The curve type, be it a line, quadratic or cubic curve, is stored within the `EdgeVariant` component.

### Systems / System Sets

The systems powering this plugin all run in a custom set within the `PostUpdate` set called `VectorGraphicSet`.
Here's an overview of the process each frame:

#### `VectorGraphicSet::DetectChanges` 

Detects changes to `Endpoints`/`Edges`, markes the parent `VectorGraphic` as requiring an update. 

1. `sys_(add_spawned|remove_despawned)_(edges|endpoints)_(to|from)_vector_graphic` - `VectorGraphic` component tracks all of its children.
These systems simply propogate changes to the world to these systems.  And mark the vector graphic as needing a re-render.
2. `sys_check_changed_endpoints_or_edges` - Checks if endpoint/edge components mutated and if so, marks parent vector graphic as needing a re-render.

#### `VectorGraphicSet::UpdatePath`

Rebuilds the lyon `Path` of the graphic so it's ready for mesh generation.

1. `todo`

#### `VectorGraphicSet::Remesh`

Rebuilds the mesh of the graphic so the changes can be rendered.

1. `sys_check_vector_graphic_style_changed` - Checks for changes to the fill/stroke to trigger a remesh using pre-calculated paths
2. `sys_remesh_vector_graphic` - Remeshes the vector graphic with the latest path / styles.




