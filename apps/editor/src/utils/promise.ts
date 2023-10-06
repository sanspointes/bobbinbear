export type ParkedPromise<T> = {
    promise: Promise<T>;
    resolve: (result: T) => void;
    reject: (reason?: Error) => void;
};
export const PromiseUtils = {
    createParkable<T>(): ParkedPromise<T> {
        let resolve!: (result: T) => void;
        let reject!: (reason?: Error) => void;
        const promise = new Promise<T>((res, rej) => {
            resolve = res;
            reject = rej;
        });

        return {
            promise,
            resolve,
            reject,
        };
    },
};
