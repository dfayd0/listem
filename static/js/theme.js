const toggle = document.getElementById("theme-switch");

toggle.addEventListener("click", () => {
  const root = document.documentElement;
  const next = root.getAttribute("data-theme") === "dark" ? "light" : "dark";
  root.setAttribute("data-theme", next);
  localStorage.setItem("theme", next);
});

// csrf double submit: echo the cookie back as a header on every htmx request
function getCookie(name) {
  return document.cookie
    .split(";")
    .map((c) => c.trim())
    .find((c) => c.startsWith(name + "="))
    ?.split("=")
    .slice(1)
    .join("=");
}

document.body.addEventListener("htmx:configRequest", (event) => {
  const token = getCookie("csrf_token");
  if (token) {
    event.detail.headers["x-csrf-token"] = token;
  }
});

function showError(msg, container) {
  const el = (container || document).querySelector(".form-error");
  if (!el) return;
  el.textContent = msg;
  el.classList.remove("hidden");
  setTimeout(() => el.classList.add("hidden"), 4000);
}
