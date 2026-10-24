import { defineConfig } from '@playwright/test';
export default defineConfig({
 testDir: './e2e', testMatch: 'journeys.ts', timeout: 30000,
 use: { launchOptions: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH ? { executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH, args:['--no-sandbox','--disable-dev-shm-usage'] } : undefined, baseURL: 'http://127.0.0.1:1420', viewport: {width:1280,height:900}, trace:'retain-on-failure' },
 webServer: {command:'bun run dev --ignore-lock',url:'http://127.0.0.1:1420',reuseExistingServer:!process.env.CI,timeout:60000},
 reporter: [['list'],['html',{open:'never'}]],
});
