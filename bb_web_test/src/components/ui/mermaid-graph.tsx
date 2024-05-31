import clsx from 'clsx';
import { JSX, createEffect, on, splitProps } from 'solid-js';

type MermaidGraphProps = JSX.HTMLAttributes<HTMLPreElement> & {
    graph: string;
};
export function MermaidGraph(props: MermaidGraphProps) {
    const [graphProps, preProps] = splitProps(props, ['graph', 'class']);

    let container!: HTMLPreElement;
    createEffect(on(()=> graphProps.graph, graph => {
        container.innerHTML = graph;
        container.removeAttribute('data-processed');
        console.log('Regenerating grpah with ', graph);

        import('mermaid').then((mod) => {
            mod.default.initialize({
                flowchart: {
                    useMaxWidth: 500,
                }
            })

            mod.default.run({
                querySelector: '.mermaid',
            });
        });
    }));

    return (
        <pre ref={container} class={clsx('mermaid', graphProps.class)} {...preProps}></pre>
    );
}
