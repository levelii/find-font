# font-finder

A high-performance font discovery library for Node.js, written in Rust.

## Features

- Fast font discovery using native system APIs
- Support for multiple platforms (Windows, macOS, Linux)
- TypeScript support
- Promise-based API

## Installation

```bash
npm install font-finder
# or
yarn add font-finder
```

## API

### `allFamilies(): Promise<string[]>`

Returns a list of all available font families on the system.

### `getFamilyVariants(familyName: string): Promise<FontDescriptor[]>`

Returns all font variants (Regular, Bold, Italic, etc.) for a specific font family.

### FontDescriptor Interface

```typescript
interface FontDescriptor {
  postscriptName?: string;  // PostScript name of the font
  family?: string;         // Font family name
  style?: string;          // Font style (e.g., "Regular", "Bold")
  weight?: number;         // Font weight (100-900)
  width?: number;          // Font width
  italic?: boolean;        // Whether the font is italic
  monospace?: boolean;     // Whether the font is monospace
}
```

## Example

```typescript
import { allFamilies, getFamilyVariants } from 'font-finder';

// Get all available font families
const families = await allFamilies();

// Get all variants of a specific font family
const arialVariants = await getFamilyVariants('Arial');
```

## Requirements

- Node.js >= 10
- Rust (for development)

## License

MIT 
