// Gets the keys of an object (T) that extend Type
export type KeysWithType<T, Type> = {
    [K in keyof T]: T[K] extends Type ? K : never;
}[keyof T];

// Picks the object fields of object (T) that extend Type
export type PickOfType<T, Type> = Pick<T, KeysWithType<T, Type>>;
