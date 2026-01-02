import * as THREE from '../../../../three/three.module.min.js';
import * as OBC from '../../../../three/ifc/components.es.js';
import Stats from '../../../../three/stats.module.js';

export class GreatViewerIFC {
    constructor(config) {
        ({
            model: this.model,
            model_format: this.modelFormat,
            resource_mapping: this.resourceMapping,
            size_flag: this.sizeFlag,
            labels: this.labels
        } = config);
        this.components = new OBC.Components();
        this.ifcPath = '../../../../three/ifc/';
        this.modelGroup = null;
        this.container = null;
        this.world = null;
        this.startTime = null;
        this.isInitialized = false;
        this.initPromise = null;
        this.infoMessage = null;
        this.isModelLoading = false;
        this.svgLoading = '<img src="../../../../icons/mini_loading.svg" />';
        this.stats = null;
        this.viewModeController = this.labels.view_perspective;
        this.viewPresets = {
            [this.labels.view_perspective]: { pos: [0, 0, 1], rot: true },
            [this.labels.view_top]: { pos: [0, 1, 0], rot: false },
            [this.labels.view_bottom]: { pos: [0, -1, 0], rot: false },
            [this.labels.view_front]: { pos: [0, 0, 1], rot: false },
            [this.labels.view_back]: { pos: [0, 0, -1], rot: false },
            [this.labels.view_left]: { pos: [-1, 0, 0], rot: false },
            [this.labels.view_right]: { pos: [1, 0, 0], rot: false },
            [this.labels.view_isometric]: { pos: [1, 1, 1], rot: true }
        };
        // Hotkeys handler
        this.handleKeyDown = this.handleKeyDown.bind(this);
        document.addEventListener('keydown', this.handleKeyDown);
    }

    handleKeyDown(e) {
        if (e.code === 'KeyF' && !this.sizeFlag) {
            this.destroy();
            document.querySelector('#three-size-button')?.click();
            return;
        }
        if (e.code === 'Escape' && this.sizeFlag) {
            this.destroy();
            document.querySelector('#three-modal-close-btn')?.click();
            return;
        }
        if (e.code === 'KeyR') {
            this?.centerModel();
            return;
        }
        const keyMap = {
            'Digit1': this.labels.view_top, 'Numpad1': this.labels.view_top,
            'Digit2': this.labels.view_front, 'Numpad2': this.labels.view_front,
            'Digit3': this.labels.view_left, 'Numpad3': this.labels.view_left,
            'Digit4': this.labels.view_perspective, 'Numpad4': this.labels.view_perspective,
            'Digit5': this.labels.view_isometric, 'Numpad5': this.labels.view_isometric
        };
        if (keyMap[e.code]) {
            this.updateViewPreset(keyMap[e.code]);
        }
    }

    async starter() {
        if (this.initPromise) return this.initPromise;
        this.startTime = performance.now();
        this.initPromise = this._starterInternal();
        return this.initPromise;
    }

    async _starterInternal() {
        const sceneHull = document.querySelector('scene-hull');
        if (sceneHull) {
            ['a-container', 'b-container'].forEach(tag => {
                const container = sceneHull.querySelector(tag);
                if (container && container.children.length > 0) {
                    container.textContent = '';
                }
            });
        }
        let container_tag = 'a-container';
        console.log(`Full screen mode: ${this.sizeFlag}`);
        if (this.sizeFlag) {
            container_tag = 'b-container';
        }
        this.container = sceneHull.querySelector(container_tag);
        try {
            this.infoMessage = document.createElement('div');
            this.infoMessage.classList.add('text-center');
            this.container.appendChild(this.infoMessage);
            this.isModelLoading = true;
            await this.initializeWorld();
            this.isInitialized = true;
            await this.loadIFC();
            if (this.infoMessage && this.infoMessage.parentNode) {
                this.infoMessage.parentNode.removeChild(this.infoMessage);
                this.infoMessage = null;
            }
        } catch (error) {
            console.error('Viewer error:', error);
            this.showError(this.labels.failed_to_load_model + ': ' + error.message);
            this.destroy();
        } finally {
            setTimeout(() => this.centerModel(), 300);
            this.isModelLoading = false;
        }
        setTimeout(() => this.printDiagnostics(), 600);
    }

    async initializeWorld() {
        const worlds = this.components.get(OBC.Worlds);
        this.world = worlds.create();
        this.world.scene = new OBC.SimpleScene(this.components);
        this.world.scene.setup();
        if (!this.sizeFlag) this.world.scene.three.background = new THREE.Color(0xffffff);
        this.world.renderer = new OBC.SimpleRenderer(this.components, this.container);
        this.world.camera = new OBC.OrthoPerspectiveCamera(this.components);
        this.components.init();
        if (this.sizeFlag) {
            const grids = this.components.get(OBC.Grids);
            grids.create(this.world);
            await this.initializeAdvancedFeatures();
        }
    }

    async loadIFC() {
        if (this.infoMessage) this.infoMessage.innerHTML = this.svgLoading;
        const response = await fetch(this.model.url);
        if (!response.ok) throw new Error(`HTTP ${response.status}`);
        const reader = response.body.getReader();
        let loadedBytes = 0;
        const chunks = [];
        while (true) {
            const { done, value } = await reader.read();
            if (done) break;
            chunks.push(value);
            loadedBytes += value.length;
            const loadedProgress = (loadedBytes / this.model.content_length) * 100;
            this.infoMessage.innerHTML = loadedProgress.toFixed(1) + '%';
        }
        const data = new Uint8Array(loadedBytes);
        let position = 0;
        for (const chunk of chunks) {
            data.set(chunk, position);
            position += chunk.length;
        }
        // console.log('Data length:', data.length);
        if (this.infoMessage) this.infoMessage.innerHTML = this.svgLoading;
        await this.loadModel(data);
    }

    async loadModel(buffer) {
        if (this.modelFormat !== 'IFC') {
            const msg = this.labels.format_not_supported + ': ' + this.modelFormat;
            this.showError(msg);
            return;
        }
        try {
            const ifcLoader = this.components.get(OBC.IfcLoader);
            await ifcLoader.setup({
                autoSetWasm: false,
                wasm: {
                    path: this.ifcPath,
                    absolute: true,
                },
            });
            await this?.initializeFragmentsManager();
            await ifcLoader.load(buffer, false, this.model.filename, {
                processData: {includeProperties: false, fast: true}
            });
        } catch (error) {
            console.error('IFC loading error:', error);
            throw error;
        }
    }

    async initializeFragmentsManager() {
        const fragments = this.components.get(OBC.FragmentsManager);
        const workerUrl = this.ifcPath + 'worker.mjs';
        await fragments.init(workerUrl);
        fragments.list.onItemSet.add(({ value: model }) => {
            model.useCamera(this.world.camera.three);
            this.world.scene.three.add(model.object);
            fragments.core.update(true);
            this.modelGroup = model.object;
        });
        const updateCore = () => {
            if (this.container.clientHeight === 0) return;
            fragments.core.update(true);
            this.requestRender();
        };
        this.world.camera.controls.addEventListener("update", updateCore);
        this.world.camera.controls.addEventListener("rest", updateCore);
    }

    async initializeAdvancedFeatures() {
        this.stats = new Stats();
        this.container.appendChild(this.stats.dom);
    }

    requestRender() {
        if (!this.container || this.container.clientHeight === 0) {
            this.destroy();
            return;
        }
        if (!this.world || !this.world.renderer) return;
        if (this.sizeFlag && this.stats) {
            this.stats.update();
        }
    }

    centerModel() {
        if (!this.modelGroup || !this.world?.camera) return;
        const box = new THREE.Box3().setFromObject(this.modelGroup);
        const center = box.getCenter(new THREE.Vector3());
        const size = box.getSize(new THREE.Vector3());
        const maxDim = Math.max(size.x, size.y, size.z);
        this.modelGroup.position.sub(center);
        const cameraDistance = Math.max(maxDim * 2.2, 15);
        this.world.camera.controls.setLookAt(0, cameraDistance, cameraDistance, 0, 0, 0, true);
    }

    updateViewPreset(viewName) {
        if (!this.modelGroup || !this.world?.camera) return;
        console.log(`Control view: ${viewName}`);
        this.viewModeController = viewName;
        const box = new THREE.Box3().setFromObject(this.modelGroup);
        const size = box.getSize(new THREE.Vector3());
        const center = box.getCenter(new THREE.Vector3());
        const maxSize = Math.max(size.x, size.y, size.z);
        const cameraDistance = maxSize * 2.2;
        const [x, y, z] = this.viewPresets[viewName].pos;
        if (this.world && this.world.camera && this.world.camera.controls) {
            this.world.camera.controls.setLookAt(
                center.x + x * cameraDistance,
                center.y + y * cameraDistance,
                center.z + z * cameraDistance,
                center.x, center.y, center.z,
                true
            );
            this.requestRender();
        }
    }

    printDiagnostics() {
        if (!this.modelGroup || !this.world?.camera) return;
        const loadTime = (performance.now() - this.startTime) / 1000;
        let totalVertices = 0;
        let totalTriangles = 0;
        let meshCount = 0;
        this.modelGroup.traverse((object) => {
            if (object.isMesh && object.geometry) {
                const pos = object.geometry.attributes.position;
                if (pos && pos.count > 0) {
                    totalVertices += pos.count;
                    totalTriangles += object.geometry.index ? object.geometry.index.count / 3 : pos.count / 3;
                    meshCount++;
                }
            }
        });
        console.log(
            `Loaded file '${this.model.filename}' (${this.model.size}) in ${loadTime.toFixed(3)} s\n` +
            `Meshes: ${meshCount}\n` +
            `Vertices: ${totalVertices.toLocaleString()}\n` +
            `Triangles: ${Math.round(totalTriangles).toLocaleString()}`
        );
    }

    showError(message) {
        if (this.infoMessage) {
            this.infoMessage.innerHTML = message;
            this.infoMessage.style.color = '#ff4444';
        }
    }

    destroy() {
        console.log('Destroying GreatViewerIFC:', this.isInitialized);
        if (!this.isInitialized) return;
        this.isInitialized = false;
        if (this.world?.renderer) {
            try {
                this.world.renderer.forceContextLoss();
            } catch (e) {
                console.warn('Error during forceContextLoss:', e);
            }
            this.world.renderer.dispose();
            this.world.renderer.domElement = null;
            this.world.renderer = null;
        }
        this.controls?.dispose();
        this.world?.scene?.dispose();
        this.world?.camera?.dispose();
        this.container && (this.container.textContent = '');
        document.removeEventListener('keydown', this.handleKeyDown);
    }
}