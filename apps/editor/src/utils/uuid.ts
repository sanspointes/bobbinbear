import { createUniqueId } from 'solid-js';

export type Uuid = string;

export const uuid = (value: string) => {
    return value as Uuid;
};
export const newUuid = () => {
    return uuid(createUniqueId());
};
