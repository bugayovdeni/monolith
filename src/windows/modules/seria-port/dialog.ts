import { invoke } from "@tauri-apps/api/core"; // Для Tauri v2. Если v1 -> "@tauri-apps/api/tauri"
import { createPortDialogTemplate } from "./template";
import "./styles.css";

interface PortInfo {
  name: string;
  description: string;
}

export class PortDialog {
  private container: HTMLElement | null = null;
  private listEl: HTMLElement | null = null;
  private statusEl: HTMLElement | null = null;
  private btnConnect: HTMLButtonElement | null = null;
  private selectedPort: string | null = null;
  private isActive = false;

  init(anchor: HTMLElement) {
    if (this.container) return; // Защита от двойного инжекта
    this.container = createPortDialogTemplate();
    anchor.appendChild(this.container);

    this.listEl = this.container.querySelector("#port-list");
    this.statusEl = this.container.querySelector("#dialog-status");
    this.btnConnect = this.container.querySelector("#btn-connect");

    this.bindEvents();
  }

  private bindEvents() {
    this.container
      ?.querySelector("#btn-scan")
      ?.addEventListener("click", () => this.scan());
    this.btnConnect?.addEventListener("click", () => this.connect());
    this.container
      ?.querySelector("#btn-close")
      ?.addEventListener("click", () => this.close());

    window.addEventListener("keydown", (e) => {
      if (e.key === "Escape" && this.isActive) this.close();
    });

    this.container?.addEventListener("click", (e) => {
      if (e.target === this.container) this.close();
    });
  }

  async open() {
    if (!this.container) return;
    this.container.classList.remove("hidden");
    this.isActive = true;
    this.selectedPort = null;
    this.btnConnect!.disabled = true;
    this.statusEl!.textContent = "";
    await this.scan();
  }

  close() {
    if (!this.container || !this.isActive) return;
    this.container.classList.add("hidden");
    this.isActive = false;
  }

  private async scan() {
    if (!this.listEl || !this.statusEl) return;
    this.statusEl.textContent = "🔍 Сканирование портов...";
    this.listEl.innerHTML = "";

    try {
      const ports = await invoke<string[]>("get_serial_ports");
      this.renderList(ports);
      this.statusEl.textContent = ports.length
        ? "✅ Выберите порт"
        : "⚠️ Порты не найдены. Проверьте драйвер.";
    } catch (err: any) {
      this.statusEl.textContent = `❌ Ошибка скана: ${err.message || err}`;
    }
  }

  private renderList(ports: string[]) {
    if (!this.listEl) return;
    this.listEl.innerHTML = "";
    ports.forEach((port) => {
      const el = document.createElement("div");
      el.className = "port-item";
      el.textContent = port;
      el.addEventListener("click", () => {
        this.selectedPort = port;
        this.btnConnect!.disabled = false;
        this.listEl!.querySelectorAll(".port-item").forEach((i) =>
          i.classList.remove("selected"),
        );
        el.classList.add("selected");
      });
      this.listEl!.appendChild(el);
    });
  }

  private async connect() {
    if (!this.selectedPort || !this.btnConnect || !this.statusEl) return;
    this.btnConnect.disabled = true;
    this.statusEl.textContent = "⏳ Открытие порта...";

    try {
      const res = await invoke<string>("connect_port", {
        portName: this.selectedPort,
      });
      this.statusEl.textContent = `✅ ${res}`;
      // Кидаем событие наружу. main.ts подхватит и обновит статус-бар.
      window.dispatchEvent(
        new CustomEvent("port:connected", { detail: this.selectedPort }),
      );
      setTimeout(() => this.close(), 600);
    } catch (err: any) {
      this.statusEl.textContent = `❌ Не удалось подключиться: ${err.message || err}`;
      this.btnConnect.disabled = false;
    }
  }
}

export const portDialog = new PortDialog();
