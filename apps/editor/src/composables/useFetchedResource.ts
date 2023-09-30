import { MaybeAccessor, access } from '@solid-primitives/utils';
import { Accessor, createEffect, createMemo, createSignal } from 'solid-js';

type UseFetchResourceOptions<TResult> = {
    responseHandler: (response: Response) => Promise<TResult>;
};

const DEFAULT_OPTIONS: UseFetchResourceOptions<Response> = {
    responseHandler: (response) => response,
};

type UseFetchResourceResult<TResult> = {
    loading: Accessor<boolean>;
    data: Accessor<TResult | undefined>;
};

export function useFetchResource<TResult = Response>(
    url: MaybeAccessor<string>,
    options: MaybeAccessor<UseFetchResourceOptions<TResult>>,
): UseFetchResourceResult<TResult> {
    const opts = createMemo(() => {
        return Object.assign({}, DEFAULT_OPTIONS, access(options));
    });

    const abortController = new AbortController();
    const [loading, setLoading] = createSignal(false);
    const [data, setData] = createSignal<TResult | undefined>();

    createEffect(() => {
        abortController.abort();
        setLoading(true);

        fetch(access(url), {
            signal: abortController.signal,
        })
            .then(opts().responseHandler)
            .then((r) => {
                setData(r as Exclude<TResult, Function>);
            });
    });

    return {
        loading,
        data,
    };
}
