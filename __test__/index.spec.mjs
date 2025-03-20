import test from 'ava'
import { allFamilies, getFamilyVariants } from '../index.js'

test('allFamilies returns array of font families', async t => {
  const families = await allFamilies();
  t.true(Array.isArray(families));
  t.true(families.length > 0);
  
  // 验证字体族名称
  for (const family of families) {
    t.true(typeof family === 'string');
    t.true(family.length > 0);
  }
});

test('getFamilyVariants returns fonts for a family', async t => {
  // 先获取可用字体族
  const families = await allFamilies();
  t.true(families.length > 0, '系统中应该至少有一个字体族');
  
  const testFamily = families[0];
  const fonts = await getFamilyVariants(testFamily);
  
  t.true(Array.isArray(fonts));
  t.true(fonts.length > 0, '应该能找到至少一个字体');
  t.true(fonts.every(font => font.family === testFamily));
});

test('getFamilyVariants handles non-existent family', async t => {
  const nonExistentFamily = 'ThisFontDoesNotExist' + Date.now();
  await t.throwsAsync(async () => {
    await getFamilyVariants(nonExistentFamily);
  });
});
