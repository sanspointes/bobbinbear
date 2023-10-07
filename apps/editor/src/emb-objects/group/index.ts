import { Uuid } from '../../utils/uuid';
import { EmbBase, EmbState } from '../shared';

/**
 * GROUP SCENE OBJECT
 */
export type EmbGroup = EmbBase & {
    /** Internal States */
    /** Unique ID for each scene object */
    id: Uuid;

    type: 'group';
};

export * from './EmbGroup';
