export type ListState = "yukleniyor" | "hata" | "bos" | "dolu";
/** Tek yükleme sınırı; demo denetimi yalnız geliştirme derlemesinde etkin. */
export async function loadList<T>(
  source: () => T[],
  scenario = "",
): Promise<T[]> {
  if (import.meta.env.DEV) {
    if (scenario === "yukleniyor")
      await new Promise((resolve) => setTimeout(resolve, 5000));
    if (scenario === "hata") throw new Error("Demo liste hatası");
    if (scenario === "bos") return [];
  }
  return source();
}
