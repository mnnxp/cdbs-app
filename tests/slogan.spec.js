const { test, expect } = require('@playwright/test');

const sloganTests = [
  { lang: 'English', slogan: 'Making The Present Better' },
  { lang: 'Russian', slogan: 'Делаем настоящее лучше' },
  { lang: 'Chinese', slogan: '让现在更美好' }
];

sloganTests.forEach(({ lang, slogan }) => {
  test(`CADBase slogan in ${lang}`, async ({ page }) => {
    await page.goto('/');
    if (lang !== 'English') {
      await page.click('.navbar-link .fa-language');
      await page.click(`.navbar-dropdown a:has-text("${lang === 'Russian' ? 'Русский' : '中文'}")`);
    }
    const footerLink = lang === 'English' ? 'What is?' : lang === 'Russian' ? 'Что это?' : '这是什么?';
    await page.click(`footer a:has-text("${footerLink}")`);
    await expect(page.locator('.modal.is-active .modal-card-title')).toHaveText(slogan);
  });
});