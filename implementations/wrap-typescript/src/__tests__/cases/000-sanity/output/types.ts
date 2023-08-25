import { BigInt, BigNumber, JSONString, Bytes } from "./common";

export class AnotherType {
  prop: string | null;
  circular: CustomType | null;
  _const: string | null;
}

export enum CustomEnum {
  STRING,
  BYTES,
  _MAX_,
}

export class CustomMapValue {
  foo: string;
}

export class CustomType {
  str: string;
  optStr: string | null;
  u: number;
  optU: number | null;
  _u8: number;
  _u16: number;
  _u32: number;
  i: number;
  _i8: number;
  _i16: number;
  _number: number;
  bigint: BigInt;
  optBigint: BigInt | null;
  bignumber: BigNumber;
  optBignumber: BigNumber | null;
  json: JSONString;
  optJson: JSONString | null;
  bytes: ArrayBuffer;
  optBytes: ArrayBuffer | null;
  _boolean: boolean;
  optBoolean: boolean | null;
  u_array: Array<number>;
  uOpt_array: Array<number> | null;
  _opt_uOptArray: Array<number | null> | null;
  optStrOptArray: Array<string | null> | null;
  uArrayArray: Array<Array<number>>;
  uOptArrayOptArray: Array<Array<number | null> | null>;
  uArrayOptArrayArray: Array<Array<Array<number>> | null>;
  crazyArray: Array<Array<Array<Array<number> | null>> | null> | null;
  object: AnotherType;
  optObject: AnotherType | null;
  objectArray: Array<AnotherType>;
  optObjectArray: Array<AnotherType | null> | null;
  en: CustomEnum;
  optEnum: CustomEnum | null;
  enumArray: Array<CustomEnum>;
  optEnumArray: Array<CustomEnum | null> | null;
  map: Map<string, number>;
  mapOfArr: Map<string, Array<number>>;
  mapOfObj: Map<string, AnotherType>;
  mapOfArrOfObj: Map<string, Array<AnotherType>>;
  mapCustomValue: Map<string, CustomMapValue | null>;
}

export class _else {
  _else: string;
}

export class Env {
  prop: string;
  optProp: string | null;
  optMap: Map<string, number | null> | null;
}

export enum _while {
  _for,
  _in,
  _MAX_,
}
