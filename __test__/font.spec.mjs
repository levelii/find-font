import test from 'ava';
import { allFamilies, getFamilyVariants } from '../index.js';

// 基础功能测试
test('allFamilies - basic functionality', async t => {
  const families = await allFamilies();
  t.true(Array.isArray(families), 'should return an array');
  t.true(families.length > 0, 'should return at least one font family');
});

// 字体族名称验证
test('allFamilies - family name validation', async t => {
  const families = await allFamilies();
  const family = families[0];

  t.true(typeof family === 'string', 'family should be a string');
  t.true(family.length > 0, 'family should not be empty');
});

// 获取字体测试
test('getFamilyVariants - font properties', async t => {
  const families = await allFamilies();
  const testFamily = families[0];
  const fonts = await getFamilyVariants(testFamily);

  t.true(Array.isArray(fonts), 'should return an array');
  t.true(fonts.length > 0, 'should return at least one font');

  const font = fonts[0];
  t.true(typeof font.family === 'string', 'family should be a string');
  t.true(font.family.length > 0, 'family should not be empty');
  t.true(typeof font.style === 'string', 'style should be a string');
  t.true(typeof font.weight === 'number', 'weight should be a number');
  t.true(typeof font.width === 'number', 'width should be a number');
  t.true(typeof font.italic === 'boolean', 'italic should be a boolean');
  t.true(typeof font.monospace === 'boolean', 'monospace should be a boolean');
});

// 错误处理测试
test('getFamilyVariants - error handling', async t => {
  const nonExistentFamily = 'ThisFontDoesNotExist' + Date.now();
  await t.throwsAsync(async () => {
    await getFamilyVariants(nonExistentFamily);
  });
});

