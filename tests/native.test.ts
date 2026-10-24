import { describe, test, expect } from "bun:test";
import { siparisOlustur, platform, destekleniyorMu } from "../src/lib/native";
import {
  exampleOrders,
  exampleProfiles,
  exampleCartItems,
} from "../src/lib/data";
import { validOrders, validProfile } from "../src/lib/coffee";
describe("Native web sözleşmesi", () => {
  test("web aynı tipli takip kodunu verir", async () => {
    const value = await siparisOlustur(5);
    expect(value.quantity).toBe(5);
    expect(value.code).toMatch(/^KMP-[0-9A-F]{8}$/);
    expect(await platform()).toBe("web");
    expect(destekleniyorMu("siparis")).toBe(true);
  });
  test("sıfır, negatif, kesir, NaN ve üst sınır reddedilir", async () => {
    for (const n of [0, -1, 1.5, NaN, 9901]) {
      let caught: unknown;
      try {
        await siparisOlustur(n);
      } catch (error) {
        caught = error;
      }
      expect(caught).toMatchObject({ code: "gecersiz_adet" });
    }
  });
  test("sentetik örnekler aynı doğrulamadan geçer", () => {
    expect(exampleCartItems).toHaveLength(6);
    expect(validOrders(exampleOrders)).toBe(true);
    expect(exampleProfiles.every(validProfile)).toBe(true);
  });
});
