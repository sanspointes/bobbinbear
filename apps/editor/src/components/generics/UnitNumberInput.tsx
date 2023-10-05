import {
    Accessor,
    JSX,
    Setter,
    createContext,
    createEffect,
    createSignal,
    on,
    splitProps,
    useContext,
} from 'solid-js';
import {
    TextInputInput,
    TextInputInputProps,
    TextInputLabel,
    TextInputRoot,
    TextInputRootProps,
} from './TextInput';
import {
    MeasurementUnit,
    UnitValue,
    parseUnitValueString,
    unitValueToMm,
} from '@/utils/measurement-units';

/**
 * CONTEXT SETUP
 */
type UnitNumberInputContextModel = {
    getCurrentValueAsMm(): UnitValue | undefined;
    setInternalValue(value: string): void;
    lastValidValue: Accessor<number>;
    isFocused: Accessor<boolean>;
    setIsFocused: Setter<boolean>;
};

const UnitNumberInputContext =
    createContext<UnitNumberInputContextModel | null>(null);

const useUnitNumberInputContext = () => {
    const ctx = useContext(UnitNumberInputContext);
    if (!ctx)
        throw new Error(
            'useUnitNumberInputContext: This component must exist within a <UnitNumberInputRoot /> component.',
        );
    return ctx as UnitNumberInputContextModel;
};

type UnitNumberInputRootProps = Omit<
    TextInputRootProps,
    'value' | 'onChange'
> & {
    /**
     * Number value in MM
     */
    value: number;
    /*
     * Callback, receives number value in mm
     */
    onChange: (value: number) => void;
    /**
     * In what unit should the mm value be displayed
     */
    desiredUnit?: MeasurementUnit;
};
export function UnitNumberInputRoot(props: UnitNumberInputRootProps) {
    // eslint-disable-next-line solid/reactivity
    const [lastValidValue, setLastValidValue] = createSignal(props.value);
    const [internalValue, setInternalValue] = createSignal('');
    const [isFocused, setIsFocused] = createSignal(false);

    const getCurrentValueAsMm = () => {
        const unitValue = parseUnitValueString(internalValue());
        if (unitValue) return unitValueToMm(unitValue);
    };

    createEffect(
        on(internalValue, (strValue) => {
            console.log(strValue);
            const mmValue = getCurrentValueAsMm();
            if (mmValue) {
                setLastValidValue(mmValue.value);
                props.onChange(mmValue.value);
            }
        }),
    );
    createEffect(
        on(
            () => props.value,
            (value) => {
                console.log(value);
                const strValue = value.toString();
                if (strValue !== internalValue()) {
                    setLastValidValue(value);
                    if (!isFocused()) setInternalValue(strValue);
                }
            },
        ),
    );

    return (
        <UnitNumberInputContext.Provider
            value={{
                getCurrentValueAsMm,
                setInternalValue,
                lastValidValue,
                isFocused,
                setIsFocused,
            }}
        >
            <TextInputRoot
                {...props}
                value={internalValue()}
                onChange={setInternalValue}
            />
        </UnitNumberInputContext.Provider>
    );
}

export const UnitNumberInputLabel = TextInputLabel;

type UnitNumberInputInputProps = TextInputInputProps & {
    unit?: MeasurementUnit;
};
export function UnitNumberInputInput(props: UnitNumberInputInputProps) {
    const [thisProps, inputProps] = splitProps(props, ['unit']);
    const ctx = useUnitNumberInputContext();

    return (
        <div class="flex items-end">
            <TextInputInput
                {...inputProps}
                onFocus={() => {
                    ctx.setIsFocused(true);
                }}
                onBlur={(event) => {
                    ctx.setIsFocused(false);
                    if (props.onBlur && typeof props.onBlur === 'function')
                        props.onBlur(event);

                    const mmValue = ctx.getCurrentValueAsMm();
                    if (!mmValue) {
                        console.log(
                            'On focus trying to update internal value with',
                            mmValue,
                        );
                        ctx.setInternalValue(ctx.lastValidValue().toString());
                    } else {
                        ctx.setInternalValue(mmValue.value.toString());
                    }
                }}
                class="z-10 rounded-r-none"
            />
            <div class="z-0 p-2 h-full text-orange-800 text-opacity-80 bg-white rounded-r-md border-l border-orange-100 border-dashed">
                {thisProps.unit ?? 'mm'}
            </div>
        </div>
    );
}
