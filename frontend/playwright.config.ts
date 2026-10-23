import{defineConfig}from'@playwright/test';
export default defineConfig({testDir:'tests',testMatch:'journeys.spec.ts',use:{baseURL:'http://127.0.0.1:14173',trace:'on',screenshot:'on'},reporter:[['html',{outputFolder:'../evidence/prototype/playwright-report',open:'never'}],['list']],outputDir:'../evidence/prototype/test-results'});
