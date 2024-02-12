export function objectValues<K extends string | number | symbol, V>(
    object: Record<K, V>,
) {
    const values = Object.values(object) as unknown as V[];
    return values;
}
