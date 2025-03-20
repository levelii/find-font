# font-finder

一个用 Rust 编写的高性能 Node.js 字体发现库。

## 特性

- 使用原生系统 API 快速发现字体
- 支持多平台（Windows、macOS、Linux）
- TypeScript 支持
- 基于 Promise 的 API

## 安装

```bash
npm install font-finder
# 或
yarn add font-finder
```

## API

### `allFamilies(): Promise<string[]>`

返回系统中所有可用的字体族列表。

### `getFamilyVariants(familyName: string): Promise<FontDescriptor[]>`

返回指定字体族的所有字体变体（Regular、Bold、Italic 等）。

### FontDescriptor 接口

```typescript
interface FontDescriptor {
  postscriptName?: string;  // 字体的 PostScript 名称
  family?: string;         // 字体系列名称
  style?: string;          // 字体样式（如 "Regular"、"Bold"）
  weight?: number;         // 字体粗细（100-900）
  width?: number;          // 字体宽度
  italic?: boolean;        // 是否为斜体
  monospace?: boolean;     // 是否为等宽字体
}
```

## 示例

```typescript
import { allFamilies, getFamilyVariants } from 'font-finder';

// 获取所有可用的字体族
const families = await allFamilies();

// 获取指定字体族的所有变体
const arialVariants = await getFamilyVariants('Arial');
```

## 系统要求

- Node.js >= 10
- Rust（用于开发）

## 许可证

MIT 
