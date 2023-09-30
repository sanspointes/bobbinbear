import { Uuid } from '@/utils/uuid';
import { EmbBase, EmbHasDimensions } from '../shared';

export type EmbText = EmbBase &
    EmbHasDimensions & {
        id: Uuid<EmbText>;
        type: 'text';

        value: string;
    };
