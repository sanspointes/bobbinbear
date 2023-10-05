const UNIT_CHECK_REGEX = /(-?[0-9]+(?:\.[0-9])?)(cm|mm|in)?/;
export type MeasurementUnit = 'cm' | 'mm' | 'in';
type BaseUnitValue = {
    value: number;
};
type CmUnitValue = BaseUnitValue & { unit: 'cm' };
type MmUnitValue = BaseUnitValue & { unit: 'mm' };
type InUnitValue = BaseUnitValue & { unit: 'in' };

export type UnitValue = CmUnitValue | MmUnitValue | InUnitValue;

export const parseUnitValueString = (str: string): UnitValue | undefined => {
    const regexResult = UNIT_CHECK_REGEX.exec(str);
    if (!regexResult) return undefined;

    const valueString = regexResult[0];
    const unit = (regexResult[2] as MeasurementUnit | undefined) ?? 'mm';

    const value = Number.parseFloat(valueString);
    if (Number.isNaN(value) || !Number.isFinite(value)) return undefined;

    return {
        value,
        unit,
    };
};

const CM_TO_MM = 10;
const INCH_TO_MM = 25.4;

export const unitValueToMm = (unitValue: UnitValue): MmUnitValue => {
    const { unit, value } = unitValue;
    if (unit === 'mm') return unitValue;
    else if (unit === 'cm')
        return {
            unit: 'mm',
            value: value * CM_TO_MM,
        };
    else if (unit === 'in')
        return {
            unit: 'mm',
            value: value * INCH_TO_MM,
        };
    throw new Error(
        `unitValueToMm: Unsupported unit "${unit}" with value "${value}".`,
    );
};
