export function createPortDialogTemplate(): HTMLElement {
  const container = document.createElement("div");
  container.id = "port-dialog";
  container.className = "modal-overlay hidden";

  container.innerHTML = `
    <div class="modal-content">
      <h3>Подключение к адаптеру</h3>
      <div id="port-list" class="port-list">
        <div class="loading">Готово к сканированию</div>
      </div>
      <div class="modal-actions">
        <button id="btn-scan" type="button">Обновить</button>
        <button id="btn-connect" type="button" disabled>Подключить</button>
        <button id="btn-close" type="button">Отмена</button>
      </div>
      <div id="dialog-status" class="status-text"></div>
    </div>
  `;

  return container;
}
