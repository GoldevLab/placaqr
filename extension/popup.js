import { SITE } from "./config.js";

document.getElementById("home").addEventListener("click", () => {
  chrome.tabs.create({ url: `${SITE}/` });
});

document.getElementById("this-page").addEventListener("click", async () => {
  const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
  const u = tab?.url || "";
  const dest = /^https?:\/\//i.test(u) ? `${SITE}/?u=${encodeURIComponent(u)}` : `${SITE}/`;
  chrome.tabs.create({ url: dest });
});
