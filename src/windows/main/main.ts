import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from '@tauri-apps/api/window';
import { ChartManager } from '../charts/chart-manager';
import logoSrc from '../../assets/monolith.svg';

// when using `"withGlobalTauri": true`, you may use
// const { getCurrentWindow } = window.__TAURI__.window;

const appWindow = getCurrentWindow();
const chart = new ChartManager ('chart-container');

// Функция-помощник для навешивания событий
function bindWindowAction(id: string, action: () => Promise<void>) {
  const element = document.getElementById(id);
  if (element) {
    element.addEventListener('click', () => {
      action().catch((err) => console.error(`Ошибка ${id}:`, err));
    });
  } else {
    console.warn(`Элемент ${id} не найден`);
  }
}

// Привязываем действия
bindWindowAction('titlebar-minimize', () => appWindow.minimize());
bindWindowAction('titlebar-maximize', () => appWindow.toggleMaximize());
bindWindowAction('titlebar-close', () => appWindow.close());


let greetInputEl: HTMLInputElement | null;
let greetMsgEl: HTMLElement | null;

async function greet() {
  if (greetMsgEl && greetInputEl) {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsgEl.textContent = await invoke("greet", {
      name: greetInputEl.value,
    });
  }
}

window.addEventListener("DOMContentLoaded", () => {
  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    greet();
  });
  const logo = document.querySelector('.app-logo') as HTMLImageElement
  if (logo) {
    logo.src = logoSrc
  }
});

// ==== ГРАФИКИ ==
setInterval(() => {
  const now = Date.now();

  chart.updateData(
    [now, Math.random() * 100],
    [now, Math.random() * 50]
  );
}, 1000);

// ==== Side Bar ===
window.addEventListener('DOMContentLoaded', () => {
  const sidebar = document.getElementById('sidebar')!;
  const btn = document.querySelector('.toggle-btn') as HTMLButtonElement;

  btn.addEventListener('click', () => {
    console.log('Toggle, current classes:', sidebar.classList);
    sidebar.classList.toggle('hidden');
  });
});