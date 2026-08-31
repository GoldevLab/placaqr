/** PlacaQR island UI. Eager module load — Resuma visible_task waits for a footer marker. */
let bootP = null;

const run = async (state, __resuma) => {
    const root = document.getElementById("placaqr-island");
    if (!root) return;
    if (root.dataset.placaqrReady === "1") return;
    root.dataset.placaqrReady = "1";

    const g = (sel) => root.querySelector(sel);
    root.dataset.view = root.dataset.view || "2d";
    const numOr = (sel, fallback) => {
        const raw = String(g(sel)?.value ?? "").trim().replace(",", ".");
        if (!raw) return fallback;
        const n = Number(raw);
        return Number.isFinite(n) ? n : fallback;
    };

    const escWifi = (s) => String(s).replace(/([\\;,:"])/g, "\\$1");
    const digits = (s) => String(s).replace(/\D/g, "");
    const DEFAULT_LABELS = {
        url: "Scan me",
        wifi: "Join WiFi",
        google: "Scan and review",
        instagram: "Follow us",
        whatsapp: "Message us",
        vcard: "Save contact",
        email: "Email us",
        phone: "Call us",
        text: "Scan me",
    };

    const buildSpec = () => {
        const preset = root.querySelector("[data-preset].is-active")?.dataset.preset || "google";
        let payload = (g('[data-bind="raw"]')?.value || "").trim();
        if (preset === "wifi") {
            const ssidRaw = (g('[data-bind="wifi_ssid"]')?.value || "").trim();
            if (!ssidRaw) payload = "";
            else {
                const ssid = escWifi(ssidRaw);
                const sec = g('[data-bind="wifi_sec"]')?.value || "WPA";
                if (sec === "nopass") payload = `WIFI:T:nopass;S:${ssid};;`;
                else payload = `WIFI:T:${sec};S:${ssid};P:${escWifi(g('[data-bind="wifi_pass"]')?.value || "")};;`;
            }
        } else if (preset === "google") {
            if (payload && !/^https?:\/\//i.test(payload)) {
                if (/\.|google|g\.page|maps/i.test(payload)) payload = "https://" + payload.replace(/^\/+/, "");
                else payload = "https://g.page/r/" + payload.replace(/^\/+/, "");
            }
        } else if (preset === "instagram") {
            const t = payload.replace(/^@/, "").trim();
            if (!t) payload = "";
            else if (/^https?:\/\//i.test(t)) payload = t;
            else if (/instagram\.com/i.test(t)) payload = "https://" + t.replace(/^\/+/, "");
            else payload = "https://instagram.com/" + t.replace(/^\/+/, "");
        } else if (preset === "whatsapp") {
            const n = digits(g('[data-bind="wa_phone"]')?.value || "");
            const msg = (g('[data-bind="wa_text"]')?.value || "").trim();
            if (!n) payload = "";
            else {
                payload = "https://wa.me/" + n;
                if (msg) payload += "?text=" + encodeURIComponent(msg);
            }
        } else if (preset === "vcard") {
            const vEsc = (s) => String(s).replace(/[\r\n]+/g, " ").replace(/\\/g, "\\\\").replace(/,/g, "\\,").replace(/;/g, "\\;");
            const nameRaw = (g('[data-bind="vcard_name"]')?.value || "").trim();
            const telRaw = (g('[data-bind="vcard_phone"]')?.value || "").trim();
            const emRaw = (g('[data-bind="vcard_email"]')?.value || "").trim();
            const orgRaw = (g('[data-bind="vcard_org"]')?.value || "").trim();
            if (!nameRaw && !telRaw && !emRaw && !orgRaw) payload = "";
            else {
                const fn = vEsc(nameRaw || "Contact");
                const tel = vEsc(telRaw);
                const em = vEsc(emRaw);
                const org = vEsc(orgRaw);
                const lines = ["BEGIN:VCARD", "VERSION:3.0", "FN:" + fn];
                if (tel) lines.push("TEL;TYPE=CELL:" + tel);
                if (em) lines.push("EMAIL:" + em);
                if (org) lines.push("ORG:" + org);
                lines.push("END:VCARD");
                payload = lines.join("\r\n");
            }
        } else if (preset === "email") {
            const to = (g('[data-bind="email_to"]')?.value || "").trim();
            const sub = (g('[data-bind="email_subj"]')?.value || "").trim();
            if (!to) payload = "";
            else {
                payload = "mailto:" + to;
                if (sub) payload += "?subject=" + encodeURIComponent(sub);
            }
        } else if (preset === "phone") {
            const n = digits(g('[data-bind="phone"]')?.value || "");
            payload = n ? ("tel:+" + n.replace(/^00/, "")) : "";
        } else if (preset === "url") {
            if (payload && !/^[a-z][a-z0-9+.-]*:\/\//i.test(payload)) {
                payload = "https://" + payload;
            }
        }
        const object = root.querySelector('input[name="object"]:checked')?.value || "stand";
        const hideLabel = object === "coin" || object === "tile";
        return {
            payload,
            object,
            label: hideLabel ? "" : (g('[data-bind="label"]')?.value || ""),
            emoji: (g('[data-bind="logo_png"]')?.value) ? "" : (g('[data-bind="emoji"]')?.value || ""),
            logo_png: g('[data-bind="logo_png"]')?.value || "",
            size_mm: Math.min(120, Math.max(30, numOr('[data-bind="size_mm"]', 55))),
            relief_mm: Math.min(2.5, Math.max(0.4, numOr('[data-bind="relief_mm"]', 0.8))),
            color_base: g('[data-bind="color_base"]')?.value || "#fafafa",
            color_fg: g('[data-bind="color_fg"]')?.value || "#12111a",
            module_shape: root.querySelector("[data-shape].is-active")?.dataset.shape
                || g('[data-bind="module_shape"]')?.value
                || "square",
            rot_y: Number(root.dataset.rotY || 35) || 35,
            with_3d: root.dataset.view === "3d",
            magnet: object === "tile" && !!(g('[data-bind="magnet"]')?.checked),
        };
    };

    let timer = null;
    let viewerP = null;
    let previewSeq = 0;
    let exporting = false;
    const friendlyError = (e) => {
        let raw = String(e?.message || e || "").trim();
        const quoted = raw.match(/Validation\("([^"]+)"\)/) || raw.match(/Validation\((.+)\)\s*$/);
        if (quoted) raw = quoted[1].replace(/^"|"$/g, "");
        raw = raw.replace(/^Error:\s*/i, "").replace(/^Validation:\s*/i, "");
        if (/failed to fetch|networkerror|load failed|network/i.test(raw)) {
            return "Could not connect. Wait a moment and try again.";
        }
        if (/timeout|timed out/i.test(raw)) {
            return "This is taking too long. Please try again.";
        }
        return raw || "Could not update the preview.";
    };
    const setStatus = (text, bad) => {
        const st = root.querySelector("#tool-status");
        if (!st) return;
        st.textContent = text || "";
        st.className = "hint" + (bad && text ? " bad" : "");
        st.hidden = !text;
    };
    const logoInp = () => g('[data-bind="logo_png"]');
    const syncLogoUi = () => {
        const data = logoInp()?.value || "";
        const thumb = root.querySelector("[data-logo-preview]");
        const clear = root.querySelector("[data-logo-clear]");
        if (thumb) {
            thumb.hidden = !data;
            if (data) thumb.src = data;
            else thumb.removeAttribute("src");
        }
        if (clear) clear.hidden = !data;
        const emoji = g('[data-bind="emoji"]');
        if (emoji) emoji.disabled = !!data;
    };
    const ingestLogo = async (file) => {
        if (!file) return;
        const type = String(file.type || "");
        const name = String(file.name || "");
        const looksImage = type.startsWith("image/") || /\.(png|jpe?g|webp|gif|svg)$/i.test(name);
        if (!looksImage) {
            setStatus("Use a PNG, JPG, or SVG logo.", true);
            return;
        }
        if (file.size > 8 * 1024 * 1024) {
            setStatus("That image is too large. Try a simpler logo.", true);
            return;
        }
        setStatus("Placing your logo…");
        let bmp = null;
        try {
            if (typeof createImageBitmap === "function") {
                try { bmp = await createImageBitmap(file); } catch (_) {}
            }
            if (!bmp) {
                const url = URL.createObjectURL(file);
                try {
                    const im = new Image();
                    im.src = url;
                    await im.decode();
                    bmp = im;
                } finally {
                    URL.revokeObjectURL(url);
                }
            }
            if (!bmp || !bmp.width || !bmp.height) {
                setStatus("Could not read that image. Try PNG or JPG.", true);
                return;
            }
            const fit = (size) => {
                const canvas = document.createElement("canvas");
                canvas.width = size;
                canvas.height = size;
                const ctx = canvas.getContext("2d", { alpha: true });
                if (!ctx) throw new Error("canvas");
                ctx.clearRect(0, 0, size, size);
                const pad = size * 0.08;
                const scale = Math.min((size - pad * 2) / bmp.width, (size - pad * 2) / bmp.height);
                const w = Math.max(1, bmp.width * scale);
                const h = Math.max(1, bmp.height * scale);
                ctx.drawImage(bmp, (size - w) / 2, (size - h) / 2, w, h);
                return canvas.toDataURL("image/png");
            };
            let dataUrl = fit(96);
            if (dataUrl.length > 90000) dataUrl = fit(64);
            if (dataUrl.length > 90000) {
                setStatus("That logo is too detailed. Use a flatter icon.", true);
                return;
            }
            const hidden = logoInp();
            if (hidden) hidden.value = dataUrl;
            const fileInp = root.querySelector("[data-logo-file]");
            if (fileInp) fileInp.value = "";
            const emoji = g('[data-bind="emoji"]');
            if (emoji) emoji.value = "";
            syncLogoUi();
            setStatus("");
            schedule();
        } catch (err) {
            setStatus("Could not read that image. Try PNG or JPG.", true);
        } finally {
            if (bmp && typeof bmp.close === "function") {
                try { bmp.close(); } catch (_) {}
            }
        }
    };
    const set3dLoading = (on) => {
        const host = root.querySelector(".view3d-host");
        if (host) host.classList.toggle("is-loading", !!on);
    };
    const loadCreateViewer = async () => {
        if (typeof globalThis.__placaqrCreateViewer === "function") {
            return globalThis.__placaqrCreateViewer;
        }
        try {
            const mod = await import("/js/placaqr-3d.js");
            if (typeof mod.createViewer === "function") return mod.createViewer;
        } catch (_) {}
        if (!globalThis.__placaqrViewerLoad) {
            globalThis.__placaqrViewerLoad = new Promise((resolve, reject) => {
                const s = document.createElement("script");
                s.type = "module";
                s.src = "/js/placaqr-3d.js";
                const n = [...document.querySelectorAll("script")].map((el) => el.nonce).find(Boolean);
                if (n) s.nonce = n;
                s.onload = () => {
                    const fn = globalThis.__placaqrCreateViewer;
                    if (typeof fn === "function") resolve(fn);
                    else {
                        globalThis.__placaqrViewerLoad = null;
                        reject(new Error("3D viewer did not start"));
                    }
                };
                s.onerror = () => {
                    globalThis.__placaqrViewerLoad = null;
                    reject(new Error("Could not load 3D preview"));
                };
                document.head.appendChild(s);
            });
        }
        return globalThis.__placaqrViewerLoad;
    };
    const ensureViewer = async () => {
        if (!viewerP) {
            viewerP = (async () => {
                try {
                    const createViewer = await loadCreateViewer();
                    const canvas = root.querySelector("#view3d-canvas");
                    if (!canvas) throw new Error("3D canvas is missing");
                    return createViewer(canvas);
                } catch (err) {
                    viewerP = null;
                    throw err;
                }
            })();
            viewerP.then((viewer) => {
                const canvas = root.querySelector("#view3d-canvas");
                if (!canvas) return;
                canvas.addEventListener("webglcontextlost", (ev) => {
                    ev.preventDefault();
                    viewerP = null;
                    try { viewer.dispose(); } catch (_) {}
                }, { once: true });
            }).catch(() => {});
        }
        return viewerP;
    };
    let refreshP = Promise.resolve();
    let previewOk = false;
    const refresh = async () => {
        const in3d = root.dataset.view === "3d";
        const seq = ++previewSeq;
        if (in3d) set3dLoading(true);
        const run = (async () => {
        try {
            const r = await __resuma.action("preview_design", [buildSpec()]);
            if (seq !== previewSeq) return;
            previewOk = true;
            const p2 = root.querySelector("#preview-2d");
            if (p2 && r.svg_2d) p2.innerHTML = r.svg_2d;
            let viewerWarn = false;
            if (in3d && r.mesh && r.mesh.length) {
                try {
                    const viewer = await ensureViewer();
                    if (seq !== previewSeq) return;
                    viewer.setMesh(r.mesh);
                    viewer.resize();
                } catch (_) {
                    viewerWarn = true;
                    setStatus("3D preview could not start. You can still download the file.", true);
                }
            }
            const hintEl = root.querySelector("#scan-hint");
            if (hintEl) {
                const contrastOnly = {
                    high: "Looks good to scan",
                    medium: "Contrast is OK — test with a phone",
                    low: "Low contrast — may not scan",
                };
                hintEl.textContent = in3d ? r.hint : (contrastOnly[r.contrast] || r.hint);
                hintEl.className = "scan-status" + (r.contrast === "low" ? " bad" : in3d && r.module_mm < 1.2 ? " warn" : "");
            }
            syncSwatches();
            if (!viewerWarn) {
                const st = root.querySelector("#tool-status");
                if (st && st.classList.contains("bad")) setStatus("");
            }
        } catch (e) {
            if (seq !== previewSeq) return;
            previewOk = false;
            setStatus(friendlyError(e), true);
        } finally {
            if (seq === previewSeq) set3dLoading(false);
        }
        })();
        refreshP = run;
        await run;
    };
    const flushPreview = async () => {
        if (timer) {
            clearTimeout(timer);
            timer = null;
            await refresh();
            return;
        }
        await refreshP;
        if (!previewOk) await refresh();
    };
    const syncSwatches = () => {
        const b = (g('[data-bind="color_base"]')?.value || "").toLowerCase();
        const f = (g('[data-bind="color_fg"]')?.value || "").toLowerCase();
        const key = `${b},${f}`;
        root.querySelectorAll(".swatch").forEach((el) => {
            const on = (el.dataset.colors || "").toLowerCase() === key;
            el.classList.toggle("is-active", on);
            el.setAttribute("aria-pressed", on ? "true" : "false");
        });
    };
    const schedule = () => {
        clearTimeout(timer);
        timer = setTimeout(() => {
            timer = null;
            refresh();
        }, 260);
    };
    const setDotShape = (shape) => {
        const next = String(shape || "square");
        const current = root.querySelector("[data-shape].is-active")?.dataset.shape
            || g('[data-bind="module_shape"]')?.value
            || "square";
        root.querySelectorAll("[data-shape]").forEach((el) => {
            const on = el.dataset.shape === next;
            el.classList.toggle("is-active", on);
            el.setAttribute("aria-pressed", on ? "true" : "false");
        });
        const hidden = g('[data-bind="module_shape"]');
        if (hidden) hidden.value = next;
        if (next !== current) schedule();
    };
    root.__placaqrRefresh = refresh;

    const SAMPLES = {
        url: "https://your-menu.com",
        google: "https://g.page/r/example/review",
        instagram: "@yourshop",
        text: "Table 12",
    };
    const KNOWN_SAMPLES = new Set(Object.values(SAMPLES));
    const OBJECT_SIZES = { stand: 55, tile: 50, keychain: 38, plaque: 80, coin: 42 };
    let lastObject = root.querySelector('input[name="object"]:checked')?.value || "stand";
    const detectPreset = (raw) => {
        const s = String(raw || "").trim();
        if (!s) return null;
        if (/^WIFI:/i.test(s)) return "wifi";
        if (/^BEGIN:VCARD/i.test(s)) return "vcard";
        if (/^mailto:/i.test(s)) return "email";
        if (/wa\.me\//i.test(s) || /api\.whatsapp\.com/i.test(s)) return "whatsapp";
        if (/instagram\.com/i.test(s) || /^@[\w.]+$/.test(s)) return "instagram";
        if (/g\.page\/r\//i.test(s) || /search\.google\.com\/local\/writereview/i.test(s)) return "google";
        if (/^tel:/i.test(s)) return "phone";
        if (/^https?:\/\//i.test(s)) return "url";
        return null;
    };
    const unescWifi = (s) => String(s).replace(/\\([\\;,:"])/g, "$1");
    const fillFromPaste = (p, text) => {
        const raw = String(text || "").trim();
        if (p === "wifi") {
            const fields = {};
            const re = /([TSPH]):((?:[^\\;]|\\.)*)/gi;
            let m;
            while ((m = re.exec(raw))) fields[m[1].toUpperCase()] = unescWifi(m[2]);
            const ssid = g('[data-bind="wifi_ssid"]');
            const pass = g('[data-bind="wifi_pass"]');
            const sec = g('[data-bind="wifi_sec"]');
            if (ssid && fields.S != null) ssid.value = fields.S;
            if (sec && fields.T) {
                const t = fields.T.toUpperCase() === "NOPASS" ? "nopass" : (fields.T.toUpperCase() === "WEP" ? "WEP" : "WPA");
                sec.value = t;
            }
            if (pass && fields.P != null) pass.value = fields.P;
            return;
        }
        if (p === "vcard") {
            const grab = (key) => {
                const m = raw.match(new RegExp("^" + key + "(?:;[^:]*)?:(.*)$", "im"));
                return m ? m[1].replace(/\\([\\,;])/g, "$1").trim() : "";
            };
            const set = (sel, v) => { const el = g(sel); if (el && v) el.value = v; };
            set('[data-bind="vcard_name"]', grab("FN"));
            set('[data-bind="vcard_phone"]', grab("TEL"));
            set('[data-bind="vcard_email"]', grab("EMAIL"));
            set('[data-bind="vcard_org"]', grab("ORG"));
            return;
        }
        if (p === "email") {
            const m = raw.match(/^mailto:([^?]*)(?:\?(.*))?/i);
            if (!m) return;
            const to = g('[data-bind="email_to"]');
            const sub = g('[data-bind="email_subj"]');
            if (to) {
                try { to.value = decodeURIComponent(m[1].trim()); }
                catch (_) { to.value = m[1].trim(); }
            }
            const q = new URLSearchParams(m[2] || "");
            if (sub && q.get("subject")) sub.value = q.get("subject");
            return;
        }
        if (p === "phone") {
            const n = digits(raw.replace(/^tel:/i, ""));
            const el = g('[data-bind="phone"]');
            if (el && n) el.value = n;
            return;
        }
        if (p === "whatsapp") {
            const n = (raw.match(/wa\.me\/(\d+)/i) || raw.match(/[?&]phone=(\d+)/i) || [])[1] || "";
            let msg = "";
            try {
                const q = raw.includes("?") ? new URLSearchParams(raw.slice(raw.indexOf("?") + 1)) : null;
                msg = q ? (q.get("text") || "") : "";
            } catch (_) {}
            const phone = g('[data-bind="wa_phone"]');
            const textEl = g('[data-bind="wa_text"]');
            if (phone && n) phone.value = n;
            if (textEl && msg) textEl.value = msg;
        }
    };
    const syncObjectUi = () => {
        const object = root.querySelector('input[name="object"]:checked')?.value || "stand";
        const in3d = root.dataset.view === "3d";
        root.querySelectorAll(".object-card").forEach((el) => {
            el.classList.toggle("is-active", el.querySelector("input")?.checked);
        });
        const labelField = root.querySelector("[data-label-field]");
        if (labelField) {
            labelField.hidden = !in3d || object === "coin" || object === "tile";
            labelField.toggleAttribute("inert", labelField.hidden);
        }
        const mag = root.querySelector("[data-magnet-field]");
        if (mag) {
            mag.hidden = !in3d || object !== "tile";
            mag.toggleAttribute("inert", mag.hidden);
        }
        const ae = document.activeElement;
        if (ae instanceof HTMLElement && root.contains(ae) && ae.closest("[hidden], [inert]")) {
            const mode = root.dataset.view === "3d" ? "3d" : "2d";
            root.querySelector(`[data-tab="${mode}"]`)?.focus();
        }
    };
    const parkHiddenFocus = () => {
        const ae = document.activeElement;
        if (!(ae instanceof HTMLElement) || !root.contains(ae)) return;
        if (!ae.closest("[hidden], [inert]")) return;
        const preset = root.querySelector("[data-preset].is-active")?.dataset.preset || "google";
        const usesRaw = preset === "url" || preset === "google" || preset === "instagram" || preset === "text";
        const next = usesRaw
            ? g('[data-bind="raw"]')
            : root.querySelector(`[data-fields="${preset}"] input:not([hidden]), [data-fields="${preset}"] select`);
        (next instanceof HTMLElement ? next : root.querySelector(`[data-tab="${root.dataset.view === "3d" ? "3d" : "2d"}"]`))?.focus();
    };
    const syncModeUi = () => {
        const mode = root.dataset.view === "3d" ? "3d" : "2d";
        root.dataset.view = mode;
        root.querySelectorAll("[data-for]").forEach((el) => {
            const hide = el.getAttribute("data-for") !== mode;
            el.hidden = hide;
            el.toggleAttribute("inert", hide);
        });
        root.querySelectorAll("[data-tab]").forEach((el) => {
            const on = el.getAttribute("data-tab") === mode;
            el.classList.toggle("is-active", on);
            el.setAttribute("aria-selected", on ? "true" : "false");
        });
        root.querySelectorAll("[data-stage]").forEach((el) => {
            const hide = el.getAttribute("data-stage") !== mode;
            el.classList.toggle("is-hidden", hide);
            el.hidden = hide;
        });
        syncObjectUi();
        const ae = document.activeElement;
        if (ae instanceof HTMLElement && root.contains(ae) && ae.closest("[hidden], [inert]")) {
            root.querySelector(`[data-tab="${mode}"]`)?.focus();
        }
    };
    root.__placaqrSyncMode = syncModeUi;
    const applyPreset = (p, opts = {}) => {
        const fillSamples = opts.fillSamples !== false;
        root.querySelectorAll("[data-preset]").forEach((el) => {
            const on = el.dataset.preset === p;
            el.classList.toggle("is-active", on);
            el.setAttribute("aria-pressed", on ? "true" : "false");
        });
        const usesRaw = p === "url" || p === "google" || p === "instagram" || p === "text";
        const payloadField = root.querySelector(".payload-field");
        if (payloadField) {
            payloadField.hidden = !usesRaw;
            payloadField.toggleAttribute("inert", !usesRaw);
        }
        root.querySelectorAll("[data-fields]").forEach((el) => {
            const hide = el.dataset.fields !== p;
            el.hidden = hide;
            el.toggleAttribute("inert", hide);
        });
        const passWrap = root.querySelector("[data-wifi-pass]");
        const sec = g('[data-bind="wifi_sec"]')?.value || "WPA";
        if (passWrap) {
            const hidePass = p !== "wifi" || sec === "nopass";
            passWrap.hidden = hidePass;
            passWrap.toggleAttribute("inert", hidePass);
        }
        parkHiddenFocus();
        const meta = {
            url: { label: "Website or link", ph: "https://your-menu.com" },
            wifi: { label: "Website or link", ph: "" },
            google: { label: "Google review link", ph: "https://g.page/r/…" },
            instagram: { label: "Instagram username", ph: "@yourshop" },
            phone: { label: "Phone", ph: "" },
            text: { label: "Text to encode", ph: "Table 12" },
            whatsapp: { label: "WhatsApp", ph: "" },
            vcard: { label: "Contact card", ph: "" },
            email: { label: "Email", ph: "" },
        }[p] || { label: "Website or link", ph: "https://your-menu.com" };
        const lab = root.querySelector("#payload-label");
        const inp = g('[data-bind="raw"]');
        if (lab) lab.textContent = meta.label;
        if (inp) {
            if (meta.ph) inp.placeholder = meta.ph;
            inp.autocomplete = (p === "url" || p === "google") ? "url" : "off";
            inp.inputMode = (p === "url" || p === "google") ? "url" : "text";
            if (usesRaw && fillSamples) {
                const cur = inp.value.trim();
                if (!cur || KNOWN_SAMPLES.has(cur)) inp.value = SAMPLES[p] || "";
            }
        }
        const labelInp = g('[data-bind="label"]');
        if (labelInp) {
            const cur = labelInp.value.trim();
            const known = Object.values(DEFAULT_LABELS);
            if (!cur || known.includes(cur)) labelInp.value = DEFAULT_LABELS[p] || "Scan me";
        }
    };
    root.__placaqrApplyPreset = applyPreset;
    const syncWifiPass = () => {
        const passWrap = root.querySelector("[data-wifi-pass]");
        const sec = g('[data-bind="wifi_sec"]')?.value || "WPA";
        const wifiOn = root.querySelector("[data-preset].is-active")?.dataset.preset === "wifi";
        if (passWrap) {
            const hidePass = !wifiOn || sec === "nopass";
            passWrap.hidden = hidePass;
            passWrap.toggleAttribute("inert", hidePass);
        }
        parkHiddenFocus();
    };
    const setScanTest = (text, ok) => {
        const el = root.querySelector("#scan-test");
        if (!el) return;
        el.hidden = !text;
        el.textContent = text || "";
        el.className = "scan-test" + (ok === true ? " ok" : ok === false ? " bad" : "");
    };
    const previewBitmap = async () => {
        const host = root.querySelector("#preview-2d");
        if (!host) return null;
        const svg = host.querySelector("svg");
        if (svg) {
            const xml = new XMLSerializer().serializeToString(svg);
            const blob = new Blob([xml], { type: "image/svg+xml;charset=utf-8" });
            const url = URL.createObjectURL(blob);
            try {
                const im = new Image();
                im.src = url;
                await im.decode();
                return im;
            } finally {
                URL.revokeObjectURL(url);
            }
        }
        const img = host.querySelector("img");
        if (img && img.complete && img.naturalWidth) return img;
        return null;
    };
    const saveBlob = (blob, filename) => {
        const a = document.createElement("a");
        a.href = URL.createObjectURL(blob);
        a.download = filename;
        a.rel = "noopener";
        document.body.appendChild(a);
        a.click();
        a.remove();
        setTimeout(() => URL.revokeObjectURL(a.href), 2500);
    };
    const bytesFromB64 = (b64) => Uint8Array.from(atob(b64), (c) => c.charCodeAt(0));
    const verifyPreview = async () => {
        await flushPreview();
        if (!previewOk) {
            setScanTest("Fix the content first.", false);
            return;
        }
        if (typeof BarcodeDetector !== "function") {
            setScanTest("Point your phone at the preview.", null);
            return;
        }
        setScanTest("Checking this preview…", null);
        try {
            const src = await previewBitmap();
            if (!src) throw new Error("missing");
            const det = new BarcodeDetector({ formats: ["qr_code"] });
            const codes = await det.detect(src);
            if (codes && codes[0] && codes[0].rawValue) {
                setScanTest("This preview scans.", true);
            } else {
                setScanTest("Could not read it here — try your phone on the preview.", false);
            }
        } catch (err) {
            setScanTest("Use your phone on the preview.", null);
        }
    };
    root.addEventListener("pointerdown", (e) => {
        if (e.button != null && e.button !== 0) return;
        const t = e.target instanceof Element ? e.target : e.target && e.target.parentElement;
        if (!(t instanceof Element)) return;
        const shapeBtn = t.closest("[data-shape]");
        if (!shapeBtn) return;
        e.preventDefault();
        setDotShape(shapeBtn.dataset.shape);
    });
    root.addEventListener("selectstart", (e) => {
        const t = e.target instanceof Element ? e.target : e.target && e.target.parentElement;
        if (t instanceof Element && t.closest(".chip, .seg")) e.preventDefault();
    });
    root.addEventListener("input", (e) => {
        const t = e.target;
        if (t instanceof HTMLInputElement && t.type === "file") return;
        schedule();
    });
    root.addEventListener("change", (e) => {
        const t = e.target;
        if (!(t instanceof HTMLElement)) return;
        if (t.dataset.preset != null || t.closest?.("[data-preset]")) {
            const btn = t.closest("[data-preset]") || t;
            const p = btn.dataset.preset;
            if (p) {
                applyPreset(p);
                schedule();
            }
            return;
        }
        if (t.name === "object") {
            const sizeInp = g('[data-bind="size_mm"]');
            const next = OBJECT_SIZES[t.value];
            const prevDefault = OBJECT_SIZES[lastObject];
            if (sizeInp && next && numOr('[data-bind="size_mm"]', NaN) === prevDefault) {
                sizeInp.value = String(next);
            }
            lastObject = t.value;
            syncObjectUi();
            schedule();
            return;
        }
        if (t instanceof HTMLInputElement && t.dataset.logoFile != null) {
            const file = t.files && t.files[0];
            if (file) ingestLogo(file);
            return;
        }
        if (t.dataset.colors) {
            const [base, fg] = t.dataset.colors.split(",");
            const b = g('[data-bind="color_base"]');
            const f = g('[data-bind="color_fg"]');
            if (b) b.value = base;
            if (f) f.value = fg;
            syncSwatches();
            schedule();
            return;
        }
        if (t.dataset.bind === "wifi_sec") {
            syncWifiPass();
            schedule();
            return;
        }
        if (t.dataset.bind) schedule();
    });
    root.addEventListener("click", async (e) => {
        const t = e.target instanceof Element ? e.target : e.target && e.target.parentElement;
        if (!t) return;
        const presetBtn = t.closest("[data-preset]");
        if (presetBtn) {
            applyPreset(presetBtn.dataset.preset);
            schedule();
            return;
        }
        const shapeBtn = t.closest("[data-shape]");
        if (shapeBtn) {
            e.preventDefault();
            setDotShape(shapeBtn.dataset.shape);
            return;
        }
        const showPass = t.closest("[data-toggle-pass]");
        if (showPass) {
            e.preventDefault();
            e.stopPropagation();
            const inp = g('[data-bind="wifi_pass"]');
            if (inp) {
                const hide = inp.type === "password";
                inp.type = hide ? "text" : "password";
                showPass.textContent = hide ? "Hide" : "Show";
                showPass.setAttribute("aria-pressed", hide ? "true" : "false");
            }
            return;
        }
        const clearLogo = t.closest("[data-logo-clear]");
        if (clearLogo) {
            e.preventDefault();
            const hidden = logoInp();
            if (hidden) hidden.value = "";
            const file = root.querySelector("[data-logo-file]");
            if (file) file.value = "";
            syncLogoUi();
            schedule();
            return;
        }
        const colors = t.closest("[data-colors]");
        if (colors) {
            const [base, fg] = colors.dataset.colors.split(",");
            const b = g('[data-bind="color_base"]');
            const f = g('[data-bind="color_fg"]');
            if (b) b.value = base;
            if (f) f.value = fg;
            syncSwatches();
            schedule();
            return;
        }
        const verify = t.closest("[data-verify-scan]");
        if (verify) {
            e.preventDefault();
            verifyPreview();
            return;
        }
        const rot = t.closest("[data-rot]");
        if (rot) {
            e.preventDefault();
            ensureViewer().then((viewer) => viewer.rotate(Number(rot.dataset.rot || 0))).catch(() => {});
            return;
        }
        const exp = t.closest("[data-export]");
        if (exp) {
            if (exporting) return;
            exporting = true;
            const fmt = exp.dataset.export;
            setStatus("Preparing your file…");
            try {
                await flushPreview();
                if (!previewOk) return;
                if (fmt === "png") {
                    let blob = null;
                    try {
                        const src = await previewBitmap();
                        if (!src) throw new Error("missing");
                        const size = 1024;
                        const canvas = document.createElement("canvas");
                        canvas.width = size;
                        canvas.height = size;
                        const ctx = canvas.getContext("2d");
                        if (!ctx) throw new Error("canvas");
                        ctx.drawImage(src, 0, 0, size, size);
                        blob = await new Promise((resolve, reject) => {
                            canvas.toBlob((b) => (b ? resolve(b) : reject(new Error("png"))), "image/png");
                        });
                    } catch (_) {
                        blob = null;
                    }
                    if (blob) {
                        saveBlob(blob, "placaqr.png");
                    } else {
                        const r = await __resuma.action("export_design", [buildSpec(), "png"]);
                        saveBlob(new Blob([bytesFromB64(r.base64)], { type: r.mime }), r.filename);
                    }
                } else {
                    const r = await __resuma.action("export_design", [buildSpec(), fmt]);
                    saveBlob(new Blob([bytesFromB64(r.base64)], { type: r.mime }), r.filename);
                }
                if (fmt === "3mf") {
                    setStatus("3MF ready. In the slicer: filament 1 = Base, filament 2 = QR. 0.16–0.20 mm layer, 0.4 mm nozzle, no supports.");
                } else {
                    setStatus("Download ready");
                }
                const note = document.querySelector("#ad-download-dialog .dialog-note");
                if (note) {
                    note.textContent = (fmt === "3mf" || fmt === "stl")
                        ? "Your file is downloading. Open the 3MF: filament 1 = Base, filament 2 = QR. Print at 0.16–0.20 mm, no supports."
                        : "Your file is downloading.";
                }
                try {
                    const dlg = document.getElementById("ad-download-dialog");
                    if (dlg && !dlg.open) dlg.showModal();
                    globalThis.__placaqrFillAds?.(dlg);
                } catch (_) {}
                try {
                    const toast = document.getElementById("toast-ad");
                    toast?.showPopover?.();
                    globalThis.__placaqrFillAds?.(toast);
                } catch (_) {}
            } catch (err) {
                setStatus(friendlyError(err), true);
            } finally {
                exporting = false;
            }
        }
    });

    document.querySelector(".toast-close")?.addEventListener("click", () => {
        document.getElementById("toast-ad")?.hidePopover?.();
    });

    root.addEventListener("placaqr:refresh", () => schedule());
    root.addEventListener("paste", (e) => {
        const t = e.target;
        if (!(t instanceof HTMLInputElement) || t.dataset.bind !== "raw") return;
        const text = (e.clipboardData && e.clipboardData.getData("text")) || "";
        const detected = detectPreset(text);
        if (!detected) return;
        const usesRaw = detected === "url" || detected === "google" || detected === "instagram" || detected === "text";
        applyPreset(detected, { fillSamples: false });
        if (!usesRaw) {
            e.preventDefault();
            fillFromPaste(detected, text);
            syncWifiPass();
            schedule();
        }
    });
    applyPreset(root.querySelector("[data-preset].is-active")?.dataset.preset || "google", { fillSamples: false });
    syncModeUi();
    syncSwatches();
    syncLogoUi();
    const logoZone = root.querySelector(".logo-pick");
    if (logoZone) {
        logoZone.addEventListener("dragover", (e) => {
            e.preventDefault();
            logoZone.classList.add("is-over");
        });
        logoZone.addEventListener("dragleave", () => logoZone.classList.remove("is-over"));
        logoZone.addEventListener("drop", (e) => {
            e.preventDefault();
            logoZone.classList.remove("is-over");
            const file = e.dataTransfer && e.dataTransfer.files && e.dataTransfer.files[0];
            if (file) ingestLogo(file);
        });
    }
    await refresh();
    return () => {
        clearTimeout(timer);
        if (viewerP) viewerP.then((v) => v.dispose()).catch(() => {});
    };
};

export function initPlacaqr(state, __resuma) {
  if (!bootP) {
    bootP = Promise.resolve(run(state || {}, __resuma || globalThis.__resuma)).catch((err) => {
      bootP = null;
      const root = document.getElementById("placaqr-island");
      if (root) delete root.dataset.placaqrReady;
      throw err;
    });
  }
  return bootP;
}

function whenReady() {
  const ok = () => document.getElementById("placaqr-island") && globalThis.__resuma && typeof globalThis.__resuma.action === "function";
  if (ok()) return Promise.resolve();
  return new Promise((resolve) => {
    const tick = () => {
      if (ok()) {
        clearInterval(id);
        resolve();
      }
    };
    const id = setInterval(tick, 30);
    document.addEventListener("DOMContentLoaded", tick, { once: true });
    setTimeout(() => {
      clearInterval(id);
      if (ok()) resolve();
    }, 20000);
  });
}

whenReady().then(() => initPlacaqr({}, globalThis.__resuma)).catch((err) => {
  console.error("[placaqr] ui", err);
});
