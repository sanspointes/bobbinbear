import { Uuid } from '../../utils/uuid';
import {
    EmbBase,
    EmbHasFill,
    EmbHasInspecting,
    EmbHasLine,
    EmbState,
} from '../shared';
import { VectorShape } from '../vec-seg';

export * from './EmbVector';

export type EmbVector = EmbBase &
    EmbState &
    EmbHasFill &
    EmbHasLine &
    EmbHasInspecting & {
        /** Internal States */
        /** Unique ID for each scene object */
        id: Uuid;

        type: 'vector';
        shape: VectorShape;
    };
