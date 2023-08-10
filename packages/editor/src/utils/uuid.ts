import { createUniqueId } from "solid-js";
type OptionalRecord = Record<string, unknown> | undefined

export type Uuid<T extends OptionalRecord = undefined> = string & { __uuidBrand: T }

export const uuid = <T extends OptionalRecord = undefined>(value: string) => {
    return value as Uuid<T>
}
export const newUuid = <T extends OptionalRecord = undefined>() => {
  return uuid<T>(createUniqueId())
}
