(() => {
  let stopHero = null;
  const mountQrDust = () => {
    if (typeof stopHero === "function") {
      stopHero();
      stopHero = null;
    }
    if (window.matchMedia("(prefers-reduced-motion: reduce)").matches) return;
    const root = document.querySelector("[data-hero-particles]");
    if (!root) return;
    const canvas = document.createElement("canvas");
    canvas.className = "hero-particles-canvas";
    canvas.setAttribute("aria-hidden", "true");
    root.replaceChildren(canvas);
    const ctx = canvas.getContext("2d", { alpha: true });
    if (!ctx) return;

    const mobile = window.matchMedia("(max-width: 640px)").matches;
    const modules = Array.from({ length: mobile ? 36 : 58 }, () => ({
      x: Math.random(),
      y: Math.random(),
      z: 0.3 + Math.random() * 0.7,
      gx: Math.round(Math.random() * 14) / 14,
      gy: Math.round(Math.random() * 8) / 8,
      snap: 0.08 + Math.random() * 0.12,
    }));
    const strands = Array.from({ length: mobile ? 5 : 8 }, () => ({
      x: Math.random(),
      y: Math.random(),
      z: 0.35 + Math.random() * 0.65,
      phase: Math.random() * Math.PI * 2,
      speed: 0.12 + Math.random() * 0.18,
    }));

    let w = 0;
    let h = 0;
    let pointerX = 0;
    let pointerY = 0;
    let raf = 0;
    let last = 0;
    let accent = "#8b5cf6";
    let ink = "#c4b5fd";

    const readColors = () => {
      const cs = getComputedStyle(document.documentElement);
      accent = (cs.getPropertyValue("--accent") || "#8b5cf6").trim() || "#8b5cf6";
      ink = (cs.getPropertyValue("--primary") || "#c4b5fd").trim() || "#c4b5fd";
    };
    const resize = () => {
      const dpr = Math.min(window.devicePixelRatio || 1, 2);
      w = root.clientWidth;
      h = root.clientHeight;
      if (!w || !h) return;
      canvas.width = Math.floor(w * dpr);
      canvas.height = Math.floor(h * dpr);
      canvas.style.width = w + "px";
      canvas.style.height = h + "px";
      ctx.setTransform(dpr, 0, 0, dpr, 0, 0);
    };
    const onMove = (e) => {
      const r = root.getBoundingClientRect();
      if (!r.width || !r.height) return;
      pointerX = (e.clientX - r.left) / r.width - 0.5;
      pointerY = (e.clientY - r.top) / r.height - 0.5;
    };
    const finder = (cx, cy, scale, t) => {
      const breathe = 1 + Math.sin(t * 0.7) * 0.04;
      const s = 5.5 * scale * breathe;
      ctx.strokeStyle = accent;
      ctx.fillStyle = accent;
      ctx.lineWidth = 1.15 * scale;
      ctx.globalAlpha = 0.22;
      ctx.strokeRect(cx - s * 1.55, cy - s * 1.55, s * 3.1, s * 3.1);
      ctx.strokeRect(cx - s * 0.95, cy - s * 0.95, s * 1.9, s * 1.9);
      ctx.globalAlpha = 0.28;
      ctx.fillRect(cx - s * 0.42, cy - s * 0.42, s * 0.84, s * 0.84);
    };
    const tick = (now) => {
      if (!root.isConnected) {
        if (typeof stopHero === "function") stopHero();
        return;
      }
      if (document.hidden) {
        raf = 0;
        last = 0;
        return;
      }
      const dt = Math.min(0.032, last ? (now - last) / 1000 : 0.016);
      last = now;
      const t = now * 0.001;
      ctx.clearRect(0, 0, w, h);

      finder(w * 0.08, h * 0.22, 1.15, t);
      finder(w * 0.92, h * 0.2, 0.95, t + 1.2);
      finder(w * 0.1, h * 0.82, 0.85, t + 2.1);

      ctx.strokeStyle = ink;
      for (const s of strands) {
        s.y -= s.speed * dt * 0.12;
        s.phase += dt * 1.4;
        if (s.y < -0.12) {
          s.y = 1.08;
          s.x = Math.random();
        }
        ctx.globalAlpha = 0.14 + s.z * 0.22;
        ctx.lineWidth = 1.1 + s.z * 1.4;
        ctx.beginPath();
        const steps = 7;
        for (let i = 0; i < steps; i++) {
          const u = i / (steps - 1);
          const x = (s.x + Math.sin(s.phase + u * 3.2) * 0.045 + pointerX * 0.03 * s.z) * w;
          const y = (s.y + u * 0.22) * h;
          if (i === 0) ctx.moveTo(x, y);
          else ctx.lineTo(x, y);
        }
        ctx.stroke();
      }

      ctx.fillStyle = accent;
      for (const m of modules) {
        m.x += (m.gx - m.x) * m.snap * dt * 4;
        m.y += (m.gy - m.y) * m.snap * dt * 4;
        const dx = m.x - (pointerX + 0.5);
        const dy = m.y - (pointerY + 0.5);
        const dist2 = dx * dx + dy * dy;
        if (dist2 < 0.04 && dist2 > 0.0002) {
          m.x += dx * 0.08;
          m.y += dy * 0.08;
        }
        if (Math.abs(m.x - m.gx) < 0.008 && Math.abs(m.y - m.gy) < 0.008 && Math.random() < 0.01) {
          m.gx = Math.round(Math.random() * 14) / 14;
          m.gy = Math.round(Math.random() * 8) / 8;
        }
        const size = 3.2 + m.z * 4.2;
        ctx.globalAlpha = 0.18 + m.z * 0.38;
        ctx.fillRect(m.x * w - size / 2, m.y * h - size / 2, size, size);
      }

      raf = requestAnimationFrame(tick);
    };
    const onVis = () => {
      if (!document.hidden && !raf && root.isConnected) raf = requestAnimationFrame(tick);
    };
    const themeWatch = new MutationObserver(readColors);
    themeWatch.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ["data-theme"],
    });
    readColors();
    resize();
    window.addEventListener("resize", resize, { passive: true });
    window.addEventListener("pointermove", onMove, { passive: true });
    document.addEventListener("visibilitychange", onVis);
    raf = requestAnimationFrame(tick);
    stopHero = () => {
      cancelAnimationFrame(raf);
      raf = 0;
      last = 0;
      themeWatch.disconnect();
      window.removeEventListener("resize", resize);
      window.removeEventListener("pointermove", onMove);
      document.removeEventListener("visibilitychange", onVis);
      canvas.remove();
      stopHero = null;
    };
  };
  const tryHero = () => mountQrDust();
  if (document.readyState === "loading") {
    document.addEventListener("DOMContentLoaded", tryHero, { once: true });
  } else {
    tryHero();
  }
  document.addEventListener("resuma:navigate", () => requestAnimationFrame(tryHero));
})();
