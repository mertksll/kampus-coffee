/** Tek bir iznin anlık durumu (Rust `PermissionSnapshot` ile birebir). */
export interface IzinDurumu {
  /** Kararlı kimlik: "notification" gibi. */
  id: string;
  /** Kullanıcıya gösterilecek ad. */
  title: string;
  /** "granted" | "denied" | "prompt" | "unsupported" | "unknown" */
  status: "granted" | "denied" | "prompt" | "unsupported" | "unknown";
  /** Çalışma zamanı isteği gösterilebilir mi. */
  can_request: boolean;
  /** Neden/nasıl kullanıldığı. */
  notes: string;
}
