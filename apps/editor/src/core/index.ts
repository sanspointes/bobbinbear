import initBobbinBear, {
    BBTool,
    EditorApi,
    JsApiEffectMsg,
    JsApiMsg,
    JsApiResponseMsg,
    main_web,
} from '@bearbroidery/bobbinbear-core';

type ParkedPromise<T> = {
    resolver: (value: T) => void;
    rejector: (error: Error) => void;
};
function parkPromise(promise: Promise<T>) {}

class CoreResponseAwaiter {
    promises: Record<number, ParkedPromise<JsApiResponseMsg[]>> = {};

    awaitResponse(responseId: number) {
        return new Promise<JsApiResponseMsg[]>((resolver, rejector) => {
            const parked: ParkedPromise<JsApiResponseMsg[]> = {
                resolver,
                rejector,
            };
            this.promises[responseId] = parked;
        });
    }

    handleReceived(msg: JsApiMsg) {
        if (msg.tag === 'Response') {
            const [responses, id] = msg.value;
            const parked = this.promises[id];
            if (parked) {
                parked.resolver(responses);
                delete this.promises[id];
            } else {
                console.error(
                    `CoreResponseAwaiter: Received response for msg ${id} but no parked promise.`,
                );
            }
        }
    }
}

type EffectCallback = (msg: JsApiEffectMsg) => void;

class CoreManager {
    responseQue = new CoreResponseAwaiter();
    editorApi: EditorApi | undefined;
    isStarted = true;
    effectCallbacks: EffectCallback[] = [];

    /**
     * Initialises the editor
     * @param canvasSelector -
     */
    async start(canvasSelector: string) {
        await this.initEditor(canvasSelector);
        requestAnimationFrame(this.tick.bind(this));
    }

    async setTool(tool: BBTool) {
        const rid = this.editorApi!.set_tool(tool);
        const response = await this.responseQue.awaitResponse(rid);
        console.log(response);
        return response;
    }

    private async initEditor(canvasSelector: string) {
        await initBobbinBear();
        const editor = await new Promise<EditorApi>((res, _) => {
            main_web(canvasSelector, (api: EditorApi) => {
                this.editorApi = api;
                res(this.editorApi);
            });
        });
        return editor;
    }

    async kill() {
        this.isStarted = false;
    }

    private tick() {
        if (this.editorApi) {
            let msg = this.editorApi.receive_msg();
            while (msg) {
                if (msg.tag === 'Response') {
                    this.responseQue.handleReceived(msg);
                } else {
                    this.handleEffect(msg.value);
                }
                msg = this.editorApi.receive_msg();
            }
        }
        if (this.isStarted) requestAnimationFrame(this.tick.bind(this));
    }

    addEffectCallback(handler: EffectCallback) {
        this.effectCallbacks.push(handler);
    }

    private handleEffect(msg: JsApiEffectMsg) {
        if (!this.effectCallbacks) return;
        for (let i = 0; i < this.effectCallbacks.length; i++) {
            const cb = this.effectCallbacks[i]!;
            cb(msg);
        }
    }
}

export const coreManager = new CoreManager();
