import { test, expect } from '@playwright/test';
import { mkdir } from 'node:fs/promises';
const evidence='docs/kanit/batch-02/ekranlar';
test.beforeAll(async()=>{await mkdir(evidence,{recursive:true});});
test('dört liste hali, tekrar dene ve arama temizleme',async({page})=>{
 await page.goto('/?durum=yukleniyor');
 await expect(page.getByRole('status').filter({hasText:'Kahveler yükleniyor'})).toBeVisible();
 await page.locator('.catalog').screenshot({path:`${evidence}/yukleniyor.png`});
 await page.goto('/?durum=hata');
 await expect(page.getByRole('alert')).toContainText('Liste yüklenemedi.');
 await page.locator('.catalog').screenshot({path:`${evidence}/hata.png`});
 await page.getByRole('button',{name:'Tekrar dene',exact:true}).click();
 await expect(page.locator('.catalog .kart')).toHaveCount(24);
 await page.goto('/?durum=bos');
 await expect(page.getByRole('status')).toContainText('Sonuç bulunamadı');
 await page.locator('.catalog').screenshot({path:`${evidence}/bos.png`});
 await page.goto('/');
 await expect(page.locator('.catalog .kart')).toHaveCount(24);
 await page.locator('.catalog').screenshot({path:`${evidence}/dolu.png`});
 await page.getByRole('searchbox',{name:'Kahve veya kafe ara'}).fill('zzzzzz-bulunmayan');
 await expect(page.getByRole('status')).toContainText('Sonuç bulunamadı');
 await page.getByRole('button',{name:'Filtreleri temizle',exact:true}).click();
 await expect(page.locator('.catalog .kart')).toHaveCount(24);
});
test('klavye ile detay, filtreli dönüş, doğrudan adres ve 404',async({page,context})=>{
 await page.goto('/');
 const search=page.getByRole('searchbox',{name:'Kahve veya kafe ara'});
 await search.fill('Latte');
 await page.getByRole('combobox',{name:'Kahveleri sırala'}).selectOption('price-up');
 const card=page.locator('.catalog .kart-main').first();
 const href=await card.getAttribute('href');
 await card.focus();await page.keyboard.press('Enter');
 await expect(page).toHaveURL(/\/urun\/\d+\//);
 const name=await page.locator('#main h1').innerText();
 const direct=await context.newPage();await direct.goto(href!);await expect(direct.locator('#main h1')).toHaveText(name);await direct.close();
 await page.getByRole('link',{name:'Kahvelere dön',exact:true}).click();
 await expect(search).toHaveValue('Latte');
 await expect(page.getByRole('combobox',{name:'Kahveleri sırala'})).toHaveValue('price-up');
 await page.goto('/urun/999999/');await expect(page.getByRole('alert')).toContainText('Ürün bulunamadı');
});
test('dört dil ve dar RTL detay',async({page})=>{
 const cases=[['','Liste yüklenemedi.','ltr','tr'],['en/','Could not load the list.','ltr','en'],['ar/','تعذر تحميل القائمة.','rtl','ar'],['fa/','بارگذاری فهرست ناموفق بود.','rtl','fa']];
 for(const [prefix,error,dir,lang] of cases){
  await page.goto(`/${prefix}?durum=hata`);await expect(page.getByRole('alert')).toContainText(error);await expect(page.locator('html')).toHaveAttribute('dir',dir);await expect(page.locator('html')).toHaveAttribute('lang',lang);
  await page.goto(`/${prefix}siparisler/?durum=hata`);await expect(page.getByRole('alert')).toContainText(error);
 }
 await page.setViewportSize({width:390,height:844});await page.goto('/ar/urun/1/');
 await expect(page.locator('html')).toHaveAttribute('dir','rtl');
 await expect(page.getByRole('button',{name:/أضف إلى السلة/})).toBeEnabled();
 await page.screenshot({path:`${evidence}/ar-detay-mobil.png`,fullPage:true});
 expect(await page.evaluate(()=>document.documentElement.scrollWidth <= window.innerWidth)).toBe(true);
});
