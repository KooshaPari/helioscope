import { test, expect } from '@playwright/test'
const BASE = process.env.BASE_URL || 'http://localhost:5173'
test('homepage loads', async ({ page }) => { await page.goto(BASE); await expect(page.locator('body')).toBeVisible() })
