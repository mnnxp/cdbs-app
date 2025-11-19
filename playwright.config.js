module.exports = {
  use: {
    baseURL: process.env.BASE_URL || 'http://localhost:8080',
    headless: true, // false для отладки
    viewport: { width: 1280, height: 720 },
    screenshot: 'only-on-failure',
    trace: 'retain-on-failure', // для отладки
  },
  testDir: './tests',
  reporter: [
    // ['html'], // красивый отчет
    ['line']  // простой вывод в консоль
  ],
  timeout: 15000, // 15 секунд таймаут
};