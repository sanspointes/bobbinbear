import { ComponentProps, JSX } from 'solid-js';
import type { RenderProps } from '../core/index';
export interface CanvasProps extends Omit<RenderProps, 'size'>, ComponentProps<'div'> {
    children: JSX.Element;
    /** Canvas fallback content, similar to img's alt prop */
    fallback?: JSX.Element;
    /**
     * Options to pass to useMeasure.
     * @see https://github.com/pmndrs/react-use-measure#api
     */
    resize?: any;
    /** The target where events are being subscribed to, default: the div that wraps canvas */
    eventSource?: HTMLElement;
    /** The event prefix that is cast into canvas pointer x/y events, default: "offset" */
    eventPrefix?: 'offset' | 'client' | 'page' | 'layer' | 'screen';
    style?: JSX.CSSProperties;
}
export interface Props extends CanvasProps {
}
/**
 * A DOM canvas which accepts threejs elements as children.
 * @see https://docs.pmnd.rs/react-three-fiber/api/canvas
 */
export declare function Canvas(props: Props): JSX.Element;
//# sourceMappingURL=Canvas.d.ts.map