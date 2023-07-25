/// <reference types="webxr" />
import type { RootState, Subscription } from './store';
export type UpdateSubscription = Omit<Subscription, 'priority'>;
/**
 * Class representing a stage that updates every frame.
 * Stages are used to build a lifecycle of effects for an app's frameloop.
 */
export declare class Stage {
    private subscribers;
    private _frameTime;
    constructor();
    /**
     * Executes all callback subscriptions on the stage.
     * @param delta - Delta time between frame calls.
     * @param [frame] - The XR frame if it exists.
     */
    frame(delta: number, frame?: XRFrame): void;
    /**
     * Adds a callback subscriber to the stage.
     * @param ref - The mutable callback reference.
     * @param store - The store to be used with the callback execution.
     * @returns A function to remove the subscription.
     */
    add(ref: UpdateSubscription['ref'], store: RootState): () => void;
    get frameTime(): number;
}
/**
 * Class representing a stage that updates every frame at a fixed rate.
 * @param name - Name of the stage.
 * @param [fixedStep] - Fixed step rate.
 * @param [maxSubsteps] - Maximum number of substeps.
 */
export declare class FixedStage extends Stage {
    private _fixedStep;
    private _maxSubsteps;
    private _accumulator;
    private _alpha;
    private _fixedFrameTime;
    private _substepTimes;
    constructor(fixedStep?: number, maxSubSteps?: number);
    /**
     * Executes all callback subscriptions on the stage.
     * @param delta - Delta time between frame calls.
     * @param [frame] - The XR frame if it exists.
     */
    frame(delta: number, frame?: XRFrame): void;
    get frameTime(): number;
    get substepTimes(): number[];
    get fixedStep(): number;
    set fixedStep(fixedStep: number);
    get maxSubsteps(): number;
    set maxSubsteps(maxSubsteps: number);
    get accumulator(): number;
    get alpha(): number;
}
export declare const Stages: {
    Early: Stage;
    Fixed: FixedStage;
    Update: Stage;
    Late: Stage;
    Render: Stage;
    After: Stage;
};
export declare const Lifecycle: Stage[];
//# sourceMappingURL=stages.d.ts.map