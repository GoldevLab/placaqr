//! Interactive PlacaQR tool — Resuma island; generation is 100% Rust #[server].

use resuma::prelude::*;

use crate::actions::preview_spec;
use crate::design::{DesignSpec, PreviewResult};

fn hint_class_for(contrast: &str, module_mm: f32) -> String {
    if contrast == "low" {
        "scan-status bad".into()
    } else if module_mm < 1.2 {
        "scan-status warn".into()
    } else {
        "scan-status".into()
    }
}

#[island]
pub fn placaqr_tool() -> View {
    let initial = preview_spec(&DesignSpec::default()).unwrap_or_else(|_| PreviewResult {
        svg_2d: String::new(),
        mesh: Vec::new(),
        module_mm: 0.0,
        contrast: "medium".into(),
        hint: "Generating preview…".into(),
        modules: 0,
    });
    let ssr_2d = crate::preview::svg_data_uri(&initial.svg_2d);
    let hint_ssr = match initial.contrast.as_str() {
        "low" => "Low contrast — may not scan".to_string(),
        "medium" => "Contrast is OK — test with a phone".to_string(),
        _ => "Looks good to scan".to_string(),
    };
    let hint_class_ssr = hint_class_for(&initial.contrast, initial.module_mm);

    visible_task!(
        r##"
(async (state, __resuma) => {
    const mod = await import("/js/placaqr-ui.js");
    if (typeof mod.initPlacaqr === "function") {
        return await mod.initPlacaqr(state, __resuma);
    }
})
"##
    );

    view! {
        <div id="placaqr-island" class="tool-grid" data-placaqr data-rot-y="35" data-view="2d">
            <div class="tool-panel">
                <fieldset class="field-block">
                    <legend class="step-legend">
                        <span class="step-num">"1"</span>
                        "What should it open?"
                    </legend>
                    <div class="seg" role="group" aria-label="Content type"
                        onClick={js! {
                            const btn = event.target.closest("[data-preset]");
                            if (!btn) return;
                            event.stopPropagation();
                            const mod = await import("/js/placaqr-ui.js");
                            await mod.initPlacaqr(state, __resuma);
                            const root = document.getElementById("placaqr-island");
                            if (typeof root?.__placaqrApplyPreset === "function") {
                                root.__placaqrApplyPreset(btn.dataset.preset);
                                if (typeof root.__placaqrRefresh === "function") await root.__placaqrRefresh();
                            }
                        }}
                    >
                        <button type="button" class="chip is-active" data-preset="google" aria-pressed="true">"Reviews"</button>
                        <button type="button" class="chip" data-preset="url" aria-pressed="false">"Link"</button>
                        <button type="button" class="chip" data-preset="wifi" aria-pressed="false">"WiFi"</button>
                        <button type="button" class="chip" data-preset="instagram" aria-pressed="false">"Instagram"</button>
                        <button type="button" class="chip" data-preset="whatsapp" aria-pressed="false">"WhatsApp"</button>
                        <button type="button" class="chip" data-preset="phone" aria-pressed="false">"Phone"</button>
                        <button type="button" class="chip" data-preset="vcard" aria-pressed="false">"Card"</button>
                        <button type="button" class="chip" data-preset="email" aria-pressed="false">"Email"</button>
                        <button type="button" class="chip" data-preset="text" aria-pressed="false">"Text"</button>
                    </div>
                    <label class="field payload-field">
                        <span id="payload-label">"Google review link"</span>
                        <input data-bind="raw" type="text" autocomplete="url" inputmode="url" enterkeyhint="done"
                            maxlength="800"
                            placeholder="https://g.page/r/…"
                            value="https://g.page/r/example/review" />
                    </label>
                    <div class="extra-fields" data-fields="wifi" hidden="">
                        <label class="field"><span>"Network name (SSID)"</span>
                            <input data-bind="wifi_ssid" type="text" autocomplete="off" placeholder="Cafe Guest" /></label>
                        <label class="field" data-wifi-pass="">
                            <span class="field-split">
                                <span>"Password"</span>
                                <button type="button" class="linkish" data-toggle-pass="" aria-pressed="false">"Show"</button>
                            </span>
                            <input data-bind="wifi_pass" type="password" autocomplete="off" placeholder="••••••••" />
                        </label>
                        <label class="field"><span>"Security"</span>
                            <select data-bind="wifi_sec">
                                <option value="WPA" selected="">"WPA/WPA2"</option>
                                <option value="WEP">"WEP"</option>
                                <option value="nopass">"Open network"</option>
                            </select>
                        </label>
                    </div>
                    <div class="extra-fields" data-fields="whatsapp" hidden="">
                        <label class="field"><span>"WhatsApp number"</span>
                            <input data-bind="wa_phone" type="tel" inputmode="tel" autocomplete="tel"
                                placeholder="15551234567" /></label>
                        <label class="field"><span>"Pre-filled message (optional)"</span>
                            <input data-bind="wa_text" type="text" maxlength="120" placeholder="Hi, I have a question" /></label>
                    </div>
                    <div class="extra-fields" data-fields="vcard" hidden="">
                        <label class="field"><span>"Name"</span>
                            <input data-bind="vcard_name" type="text" autocomplete="name" placeholder="Maria Lopez" /></label>
                        <label class="field"><span>"Phone"</span>
                            <input data-bind="vcard_phone" type="tel" autocomplete="tel" placeholder="+1 555 123 4567" /></label>
                        <label class="field"><span>"Email"</span>
                            <input data-bind="vcard_email" type="email" autocomplete="email" placeholder="hello@cafe.com" /></label>
                        <label class="field"><span>"Business (optional)"</span>
                            <input data-bind="vcard_org" type="text" autocomplete="organization" placeholder="Cafe Verde" /></label>
                    </div>
                    <div class="extra-fields" data-fields="phone" hidden="">
                        <label class="field"><span>"Phone number"</span>
                            <input data-bind="phone" type="tel" inputmode="tel" autocomplete="tel"
                                placeholder="15551234567" /></label>
                    </div>
                    <div class="extra-fields" data-fields="email" hidden="">
                        <label class="field"><span>"Email address"</span>
                            <input data-bind="email_to" type="email" autocomplete="email" placeholder="hello@cafe.com" /></label>
                        <label class="field"><span>"Subject (optional)"</span>
                            <input data-bind="email_subj" type="text" placeholder="Table booking" /></label>
                    </div>
                </fieldset>

                <fieldset class="field-block">
                    <legend class="step-legend">
                        <span class="step-num">"2"</span>
                        <span data-for="2d">"How it looks"</span>
                        <span data-for="3d" hidden="">"What to print"</span>
                    </legend>
                    <div class="object-cards" data-for="3d" hidden="" role="radiogroup" aria-label="Object type">
                        <label class="object-card">
                            <input type="radio" name="object" value="stand" checked="" />
                            <span class="obj-ico obj-ico-stand" aria-hidden="true">{"\u{00a0}"}</span>
                            <strong>"Stand"</strong>
                            <span>"Raised"</span>
                        </label>
                        <label class="object-card">
                            <input type="radio" name="object" value="tile" />
                            <span class="obj-ico obj-ico-tile" aria-hidden="true">{"\u{00a0}"}</span>
                            <strong>"Tile"</strong>
                            <span>"Flush"</span>
                        </label>
                        <label class="object-card">
                            <input type="radio" name="object" value="keychain" />
                            <span class="obj-ico obj-ico-key" aria-hidden="true">{"\u{00a0}"}</span>
                            <strong>"Keychain"</strong>
                            <span>"Keys"</span>
                        </label>
                        <label class="object-card">
                            <input type="radio" name="object" value="plaque" />
                            <span class="obj-ico obj-ico-plaque" aria-hidden="true">{"\u{00a0}"}</span>
                            <strong>"Plaque"</strong>
                            <span>"Wall"</span>
                        </label>
                        <label class="object-card">
                            <input type="radio" name="object" value="coin" />
                            <span class="obj-ico obj-ico-coin" aria-hidden="true">{"\u{00a0}"}</span>
                            <strong>"Coin"</strong>
                            <span>"Bag"</span>
                        </label>
                    </div>
                    <label class="check-row" data-for="3d" data-magnet-field="" hidden="">
                        <input data-bind="magnet" type="checkbox" />
                        <span>"Magnet pocket on the back (6 × 2 mm fridge magnet)"</span>
                    </label>
                    <label class="field" data-for="3d" data-label-field="" hidden="">
                        <span>"Label on the object"</span>
                        <input data-bind="label" type="text" maxlength="28"
                            value="Scan and review" placeholder="Scan and review" />
                    </label>
                    <div class="logo-pick">
                        <label class="logo-btn">
                            <input data-logo-file="" type="file"
                                accept="image/png,image/jpeg,image/webp,image/svg+xml" />
                            "Logo"
                        </label>
                        <img class="logo-thumb" data-logo-preview="" alt="Uploaded logo" width="48" height="48" hidden="" />
                        <button type="button" class="linkish" data-logo-clear="" hidden="">"Remove"</button>
                        <span class="logo-help">"Optional · PNG, JPG, SVG"</span>
                        <input data-bind="logo_png" type="hidden" value="" autocomplete="off" />
                    </div>
                    <label class="field" data-for="3d" hidden="">
                        <span>"Width (mm)"</span>
                        <input data-bind="size_mm" type="number" min="30" max="120" step="1" value="55"
                            inputmode="numeric" aria-describedby="scan-hint" /></label>
                    <label class="field" data-for="3d" hidden="">
                        <span>"QR relief (mm)"</span>
                        <input data-bind="relief_mm" type="number" min="0.4" max="2.5" step="0.1" value="0.8" /></label>
                    <label class="field" data-for="2d">
                        <span>"Center emoji"</span>
                        <input data-bind="emoji" type="text" maxlength="4" value="" placeholder="⭐" /></label>
                    <div class="field" data-for="2d">
                        <span>"Dot shape"</span>
                        <div class="seg" role="group" aria-label="Dot shape">
                            <button type="button" class="chip is-active" data-shape="square" aria-pressed="true">"Squares"</button>
                            <button type="button" class="chip" data-shape="rounded" aria-pressed="false">"Rounded"</button>
                            <button type="button" class="chip" data-shape="circles" aria-pressed="false">"Circles"</button>
                        </div>
                        <input data-bind="module_shape" type="hidden" value="square" autocomplete="off" />
                    </div>
                </fieldset>

                <fieldset class="field-block">
                    <legend class="step-legend">
                        <span class="step-num">"3"</span>
                        "Colors"
                    </legend>
                    <div class="swatch-row" role="group" aria-label="Color presets">
                        <button type="button" class="swatch swatch--light is-active" data-colors="#fafafa,#12111a" title="Light / ink" aria-label="Light base, ink QR" aria-pressed="true">
                            <span class="swatch-a">{"\u{00a0}"}</span><span class="swatch-b">{"\u{00a0}"}</span>
                        </button>
                        <button type="button" class="swatch swatch--dark" data-colors="#12111a,#fafafa" title="Ink / light" aria-label="Ink base, light QR" aria-pressed="false">
                            <span class="swatch-a">{"\u{00a0}"}</span><span class="swatch-b">{"\u{00a0}"}</span>
                        </button>
                        <button type="button" class="swatch swatch--cream" data-colors="#fff8e7,#1a3a5c" title="Cream / blue" aria-label="Cream base, blue QR" aria-pressed="false">
                            <span class="swatch-a">{"\u{00a0}"}</span><span class="swatch-b">{"\u{00a0}"}</span>
                        </button>
                        <button type="button" class="swatch swatch--indigo" data-colors="#eef0ff,#1e1b4b" title="Lilac / navy" aria-label="Lilac base, navy QR" aria-pressed="false">
                            <span class="swatch-a">{"\u{00a0}"}</span><span class="swatch-b">{"\u{00a0}"}</span>
                        </button>
                    </div>
                    <div class="color-picks">
                        <label class="color-pick">
                            <span class="color-pick-meta">
                                <span class="color-pick-name">"Base"</span>
                                <span class="color-pick-help" data-for="2d">"Background"</span>
                                <span class="color-pick-help" data-for="3d" hidden="">"Object body"</span>
                            </span>
                            <span class="color-well">
                                <input data-bind="color_base" type="color" value="#fafafa" />
                            </span>
                        </label>
                        <label class="color-pick">
                            <span class="color-pick-meta">
                                <span class="color-pick-name">"QR"</span>
                                <span class="color-pick-help" data-for="2d">"Modules"</span>
                                <span class="color-pick-help" data-for="3d" hidden="">"Raised code"</span>
                            </span>
                            <span class="color-well">
                                <input data-bind="color_fg" type="color" value="#12111a" />
                            </span>
                        </label>
                    </div>
                </fieldset>

                <p class={hint_class_ssr} id="scan-hint">{hint_ssr}</p>

                <div class="download-block">
                    <button type="button" class="btn btn-primary btn-download" data-for="3d" data-export="3mf" hidden="">"Download 3MF to print"</button>
                    <button type="button" class="btn btn-primary btn-download" data-for="2d" data-export="png">"Download PNG"</button>
                    <div class="download-more" data-for="3d" hidden="">
                        <button type="button" class="btn btn-ghost" data-export="stl">"STL"</button>
                    </div>
                    <div class="download-more" data-for="2d">
                        <button type="button" class="btn btn-ghost" data-export="svg">"SVG"</button>
                    </div>
                    <p class="hint" id="tool-status" hidden=""></p>
                    <details class="print-how" data-for="3d" hidden="">
                        <summary>"How to print the 3MF"</summary>
                        <ol class="print-recipe">
                            <li>"Open it in Bambu Studio, Orca, or PrusaSlicer."</li>
                            <li>"Filament 1 = Base · filament 2 = QR."</li>
                            <li>"0.16–0.20 mm layer · 0.4 mm nozzle · no supports."</li>
                        </ol>
                    </details>
                </div>
            </div>

            <div class="preview-panel">
                <div class="preview-head">
                    <p class="preview-kicker">"Live preview"</p>
                    <div class="preview-tabs" role="tablist">
                    <button type="button" class="tab is-active" data-tab="2d" role="tab" aria-selected="true"
                        onClick={js! {
                            const root = document.getElementById("placaqr-island");
                            if (!root) return;
                            root.dataset.view = "2d";
                            if (typeof root.__placaqrSyncMode === "function") root.__placaqrSyncMode();
                            if (root.__placaqrRefresh) await root.__placaqrRefresh();
                        }}
                    >"2D"</button>
                    <button type="button" class="tab" data-tab="3d" role="tab" aria-selected="false"
                        onClick={js! {
                            const root = document.getElementById("placaqr-island");
                            if (!root) return;
                            root.dataset.view = "3d";
                            if (typeof root.__placaqrSyncMode === "function") root.__placaqrSyncMode();
                            root.querySelector(".view3d-host")?.classList.add("is-loading");
                            if (root.__placaqrRefresh) await root.__placaqrRefresh();
                        }}
                    >"3D"</button>
                    </div>
                </div>
                <div class="preview-stage" data-stage="2d">
                    <div id="preview-2d" class="svg-host">
                        <img class="preview-img" src={ssr_2d} width="512" height="512" alt="QR code preview" />
                    </div>
                </div>
                <div class="scan-row" data-for="2d">
                    <p class="scan-cta">"Scan this preview with your phone."</p>
                    <p class="scan-test" id="scan-test" hidden=""></p>
                    <button type="button" class="btn btn-ghost btn-verify" data-verify-scan="">"Check this preview"</button>
                </div>
                <div class="preview-stage is-hidden" data-stage="3d" hidden="">
                    <div id="preview-3d" class="view3d-host is-loading">
                        <canvas id="view3d-canvas" aria-label="3D object preview"></canvas>
                        <div class="view3d-loader" role="status" aria-live="polite">
                            <span class="view3d-spinner" aria-hidden="true"></span>
                            <span>"Preparing 3D preview…"</span>
                        </div>
                        <p class="view3d-caption">"Drag to rotate · scroll to zoom"</p>
                    </div>
                    <div class="rot-controls">
                        <button type="button" class="btn btn-ghost" data-rot="-25">"↺"</button>
                        <button type="button" class="btn btn-ghost" data-rot="25">"↻"</button>
                    </div>
                </div>
            </div>
        </div>
    }
}
