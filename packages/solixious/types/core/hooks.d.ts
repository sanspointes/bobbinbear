import type { RenderCallback, StageTypes, UpdateCallback } from "./store";
import type { ObjectMap } from "./utils";
import { Container } from "pixi.js";
/**
 * Accesses R3F's internal state, containing renderer, canvas, scene, etc.
 * @see https://docs.pmnd.rs/react-three-fiber/api/hooks#usethree
 */
export declare function useThree(): import("./store").RootState;
/**
 * Executes a callback before render in a shared frame loop.
 * Can order effects with render priority or manually render with a positive priority.
 * @see https://docs.pmnd.rs/react-three-fiber/api/hooks#useframe
 */
export declare function useFrame(callback: RenderCallback, renderPriority?: number): void;
/**
 * Executes a callback in a given update stage.
 * Uses the stage instance to indetify which stage to target in the lifecycle.
 */
export declare function useUpdate(callback: UpdateCallback, stage?: StageTypes): void;
/**
 * Returns a node graph of an object with named nodes & materials.
 * @see https://docs.pmnd.rs/react-three-fiber/api/hooks#usegraph
 */
export declare function useGraph(object: Container): import("solid-js").Accessor<ObjectMap>;
/**
 * Removes a loaded asset from cache.
 */
//# sourceMappingURL=hooks.d.ts.map