export enum ByteFormat {
	ASCII = 'ascii',
	HEX = 'hex',
	BINARY = 'bin',
	DECIMAL = 'dec'
}

export interface FormatConfig {
	name: string;
	regex: RegExp,
	base: number;
	string: (n: number) => string;
	parse: (str: string) => number;
}

export const FORMATS: Record<ByteFormat, FormatConfig> = {
	[ByteFormat.ASCII]: {
		name: 'ASCII',
		regex: /[^\x00-\x7F]/g,
		base: 255,
		string: (n) => String.fromCharCode(n),
		parse: (str) => str.charCodeAt(0)
	},
	[ByteFormat.HEX]: {
		name: 'Hexadecimal',
		regex: /[^0-9A-Fa-f]/g,
		base: 16,
		string: (n) => n.toString(16),
		parse: (str) => parseInt(str, 16)
	},
	[ByteFormat.DECIMAL]: {
		name: 'Decimal',
		regex: /[^0-9]/g,
		base: 10,
		string: (n) => n.toString(10),
		parse: (str) => parseInt(str, 10)
	},
	[ByteFormat.BINARY]: {
		name: 'Binary',
		regex: /[^01]/g,
		base: 2,
		string: (n) => n.toString(2),
		parse: (str) => parseInt(str, 2)
	}
}