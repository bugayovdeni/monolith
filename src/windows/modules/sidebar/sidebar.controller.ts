export function initSidebars() {
  // Левый сайдбар
  const sidebarLeft = document.getElementById("sidebar-left");
  const btnLeft = document.querySelector(
    ".toggle-btn-left",
  ) as HTMLButtonElement;

  if (sidebarLeft && btnLeft) {
    btnLeft.addEventListener("click", (e) => {
      e.stopPropagation();
      sidebarLeft.classList.toggle("hidden");
    });
  }

  // Правый сайдбар
  const sidebarRight = document.getElementById("sidebar-right");
  const btnRight = document.querySelector(
    "#sidebar-right .toggle-btn",
  ) as HTMLButtonElement;

  if (sidebarRight && btnRight) {
    btnRight.addEventListener("click", (e) => {
      e.stopPropagation();
      sidebarRight.classList.toggle("hidden");
    });
  }
}
