import { SITE } from "./config.js";

function qrUrl(pageUrl) {
  if (pageUrl && /^https?:\/\//i.test(pageUrl)) {
    return `${SITE}/?u=${encodeURIComponent(pageUrl)}`;
  }
  return `${SITE}/`;
}

chrome.runtime.onInstalled.addListener(() => {
  chrome.contextMenus.removeAll(() => {
    chrome.contextMenus.create({
      id: "placaqr-page",
      title: "3D QR of this page",
      contexts: ["page"],
    });
    chrome.contextMenus.create({
      id: "placaqr-link",
      title: "3D QR of this link",
      contexts: ["link"],
    });
  });
});

chrome.contextMenus.onClicked.addListener((info, tab) => {
  if (info.menuItemId === "placaqr-link") {
    chrome.tabs.create({ url: qrUrl(info.linkUrl) });
    return;
  }
  if (info.menuItemId === "placaqr-page") {
    chrome.tabs.create({ url: qrUrl(tab?.url || info.pageUrl) });
  }
});
