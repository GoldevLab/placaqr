import * as THREE from "/js/three.module.min.js";

export function createViewer(canvas) {
  const renderer = new THREE.WebGLRenderer({
    canvas,
    antialias: true,
    alpha: false,
    powerPreference: "high-performance",
  });
  renderer.setPixelRatio(Math.min(window.devicePixelRatio || 1, 2));
  renderer.outputColorSpace = THREE.SRGBColorSpace;
  renderer.shadowMap.enabled = true;
  renderer.shadowMap.type = THREE.PCFSoftShadowMap;
  renderer.toneMapping = THREE.ACESFilmicToneMapping;
  renderer.toneMappingExposure = 1.05;

  const scene = new THREE.Scene();
  scene.background = new THREE.Color(0xeef3f0);

  const camera = new THREE.PerspectiveCamera(38, 1, 0.1, 2000);
  const target = new THREE.Vector3(0, 12, 0);
  let yaw = 0.72;
  let pitch = 0.48;
  let dist = 120;

  const hemi = new THREE.HemisphereLight(0xffffff, 0x8aa396, 0.95);
  scene.add(hemi);
  const key = new THREE.DirectionalLight(0xffffff, 1.35);
  key.position.set(55, 90, 40);
  key.castShadow = true;
  key.shadow.mapSize.set(1024, 1024);
  key.shadow.camera.near = 1;
  key.shadow.camera.far = 280;
  key.shadow.camera.left = -90;
  key.shadow.camera.right = 90;
  key.shadow.camera.top = 90;
  key.shadow.camera.bottom = -90;
  scene.add(key);
  const fill = new THREE.DirectionalLight(0xd7efe4, 0.45);
  fill.position.set(-40, 30, -50);
  scene.add(fill);

  const ground = new THREE.Mesh(
    new THREE.CircleGeometry(90, 64),
    new THREE.MeshStandardMaterial({
      color: 0xd8e3dd,
      roughness: 1,
      metalness: 0,
    }),
  );
  ground.rotation.x = -Math.PI / 2;
  ground.receiveShadow = true;
  scene.add(ground);

  const model = new THREE.Group();
  scene.add(model);

  const fitCamera = () => {
    const box = new THREE.Box3().setFromObject(model);
    if (box.isEmpty()) return;
    const size = box.getSize(new THREE.Vector3());
    const center = box.getCenter(new THREE.Vector3());
    target.copy(center);
    const extent = Math.max(size.x, size.y, size.z, 20);
    dist = extent * 2.15;
    ground.position.y = box.min.y - 0.15;
    ground.scale.setScalar(Math.max(extent / 70, 0.7));
    key.position.set(extent * 0.8, extent * 1.4, extent * 0.6);
    updateCamera();
  };

  const updateCamera = () => {
    const cp = Math.cos(pitch);
    camera.position.set(
      target.x + dist * Math.sin(yaw) * cp,
      target.y + dist * Math.sin(pitch),
      target.z + dist * Math.cos(yaw) * cp,
    );
    camera.lookAt(target);
  };

  const hexColor = (hex) => {
    const n = Number.parseInt(String(hex || "#888888").replace("#", ""), 16);
    return Number.isFinite(n) ? n : 0x888888;
  };

  let fitted = false;
  let lastExtent = 0;
  const setMesh = (parts) => {
    while (model.children.length) {
      const child = model.children[0];
      model.remove(child);
      child.geometry?.dispose();
      child.material?.dispose();
    }
    for (const part of parts || []) {
      const pos = part.positions;
      if (!pos || pos.length < 9) continue;
      const geometry = new THREE.BufferGeometry();
      geometry.setAttribute("position", new THREE.Float32BufferAttribute(pos, 3));
      geometry.computeVertexNormals();
      const material = new THREE.MeshStandardMaterial({
        color: hexColor(part.color),
        roughness: 0.48,
        metalness: 0.04,
        flatShading: true,
        side: THREE.DoubleSide,
      });
      const mesh = new THREE.Mesh(geometry, material);
      mesh.castShadow = true;
      mesh.receiveShadow = true;
      model.add(mesh);
    }
    const box = new THREE.Box3().setFromObject(model);
    if (box.isEmpty()) return;
    const size = box.getSize(new THREE.Vector3());
    const center = box.getCenter(new THREE.Vector3());
    const extent = Math.max(size.x, size.y, size.z, 20);
    ground.position.y = box.min.y - 0.15;
    ground.scale.setScalar(Math.max(extent / 70, 0.7));
    key.position.set(extent * 0.8, extent * 1.4, extent * 0.6);
    const sizeChanged = !fitted || Math.abs(extent - lastExtent) / Math.max(lastExtent, 1) > 0.22;
    if (sizeChanged) {
      fitCamera();
      fitted = true;
    } else {
      target.copy(center);
      updateCamera();
    }
    lastExtent = extent;
  };

  let lastW = 0;
  let lastH = 0;
  const resize = () => {
    const host = canvas.parentElement;
    if (!host) return;
    const rect = host.getBoundingClientRect();
    const w = Math.min(Math.max(Math.round(rect.width) || 320, 160), 640);
    const h = Math.min(Math.max(Math.round(rect.height) || 320, 160), 400);
    if (w === lastW && h === lastH) return;
    lastW = w;
    lastH = h;
    renderer.setSize(w, h, false);
    canvas.style.width = "100%";
    canvas.style.height = "100%";
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
  };

  let dragging = false;
  let lastX = 0;
  let lastY = 0;
  const onPointerDown = (e) => {
    dragging = true;
    lastX = e.clientX;
    lastY = e.clientY;
    try {
      canvas.setPointerCapture(e.pointerId);
    } catch (_) {}
  };
  const onPointerMove = (e) => {
    if (!dragging) return;
    yaw -= (e.clientX - lastX) * 0.008;
    pitch = Math.min(1.2, Math.max(0.12, pitch + (e.clientY - lastY) * 0.006));
    lastX = e.clientX;
    lastY = e.clientY;
    updateCamera();
  };
  const stopDrag = () => {
    dragging = false;
  };
  const onWheel = (e) => {
    e.preventDefault();
    dist = Math.min(400, Math.max(24, dist * (e.deltaY > 0 ? 1.08 : 0.92)));
    updateCamera();
  };
  canvas.addEventListener("pointerdown", onPointerDown);
  canvas.addEventListener("pointermove", onPointerMove);
  canvas.addEventListener("pointerup", stopDrag);
  canvas.addEventListener("pointercancel", stopDrag);
  canvas.addEventListener("lostpointercapture", stopDrag);
  canvas.addEventListener("wheel", onWheel, { passive: false });

  let raf = 0;
  const tick = () => {
    raf = requestAnimationFrame(tick);
    renderer.render(scene, camera);
  };

  const ro = new ResizeObserver(resize);
  ro.observe(canvas.parentElement || canvas);
  resize();
  updateCamera();
  tick();

  return {
    setMesh,
    resize,
    rotate(deg) {
      yaw += (Number(deg) * Math.PI) / 180;
      updateCamera();
    },
    dispose() {
      cancelAnimationFrame(raf);
      ro.disconnect();
      canvas.removeEventListener("pointerdown", onPointerDown);
      canvas.removeEventListener("pointermove", onPointerMove);
      canvas.removeEventListener("pointerup", stopDrag);
      canvas.removeEventListener("pointercancel", stopDrag);
      canvas.removeEventListener("lostpointercapture", stopDrag);
      canvas.removeEventListener("wheel", onWheel);
      while (model.children.length) {
        const child = model.children[0];
        model.remove(child);
        child.geometry?.dispose();
        child.material?.dispose();
      }
      ground.geometry?.dispose();
      ground.material?.dispose();
      renderer.dispose();
    },
  };
}

globalThis.__placaqrCreateViewer = createViewer;
