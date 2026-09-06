import * as THREE from '../../../../three/three.webgpu.min.js';
import * as OBC from '../../../../three/ifc/components.es.js';
import Stats from '../../../../three/stats.module.js';
import { fetchWithCache } from '../../../../three/model-cache.js';

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
            [this.labels.view_perspective]: { pos: [0, 0, 1], rot: true, projection: 'perspective' },
            [this.labels.view_top]: { pos: [0, 1, 0], rot: false, projection: 'orthographic' },
            [this.labels.view_bottom]: { pos: [0, -1, 0], rot: false, projection: 'orthographic' },
            [this.labels.view_front]: { pos: [0, 0, 1], rot: false, projection: 'orthographic' },
            [this.labels.view_back]: { pos: [0, 0, -1], rot: false, projection: 'orthographic' },
            [this.labels.view_left]: { pos: [-1, 0, 0], rot: false, projection: 'orthographic' },
            [this.labels.view_right]: { pos: [1, 0, 0], rot: false, projection: 'orthographic' },
            [this.labels.view_isometric]: { pos: [1, 1, 1], rot: true, projection: 'perspective' }
        };
        this.animationFrameId = null;
        this._updateCoreBound = null;
        // Hotkeys handler
        this.handleKeyDown = this.handleKeyDown.bind(this);
        document.addEventListener('keydown', this.handleKeyDown);
    }

    handleKeyDown(e) {
        if (!this.isInitialized) return;
        if (e.code === 'KeyH') {
            this.centerModel();
            return;
        }
        const keyMap = {
            // Flat views
            'Digit0': 'isometric', 'Numpad0': 'isometric',
            'Digit1': 'front',     'Numpad1': 'front',
            'Digit3': 'left',      'Numpad3': 'left',
            'Digit7': 'top',       'Numpad7': 'top',
            'Digit9': 'opposite',  'Numpad9': 'opposite',
        };
        const action = keyMap[e.code];
        if (!action) return; // Ignore unmapped keys
        const viewPresets = ['front', 'left', 'top', 'isometric'];
        if (viewPresets.includes(action)) {
            const labelKey = `view_${action}`;
            const translatedPreset = this.labels[labelKey];
            if (translatedPreset) {
                this.updateViewPreset(translatedPreset);
            }
        } else if (action === 'opposite') {
            this.goToOppositeView();
        }
        e.preventDefault();
    }

    async starter() {
        if (this.initPromise) return this.initPromise;
        this.startTime = performance.now();
        this.initPromise = this._starterInternal();
        return this.initPromise;
    }

    startRenderLoop() {
        if (this.animationFrameId) return;
        const loop = () => {
            if (!this.isInitialized) return;
            const fragments = this.components.get(OBC.FragmentsManager);
            if (fragments && fragments.core) {
                fragments.core.update(true);
            }
            this.animationFrameId = requestAnimationFrame(loop);
        };
        this.animationFrameId = requestAnimationFrame(loop);
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

    resize() {
        if (!this.container || !this.renderer || !this.world?.camera) return;
        const width = this.container.clientWidth;
        const height = this.container.clientHeight;
        this.renderer.setSize(width, height, false);
        const camera = this.world.camera;
        if (camera.three) {
            camera.three.aspect = width / height;
            if (camera.three.isOrthographicCamera) {
                const frustumSize = camera.frustumSize || 45;
                const aspect = width / height;
                camera.three.left = (-frustumSize * aspect) / 2;
                camera.three.right = (frustumSize * aspect) / 2;
                camera.three.top = frustumSize / 2;
                camera.three.bottom = -frustumSize / 2;
            }
            camera.three.updateProjectionMatrix();
        }
        this.updateStats();
    }

    async initializeWorld() {
        const worlds = this.components.get(OBC.Worlds);
        this.world = worlds.create();
        this.world.scene = new OBC.SimpleScene(this.components);
        this.world.scene.setup();
        if (!this.sizeFlag) this.world.scene.three.background = new THREE.Color(0xffffff);
        const customRenderer = new THREE.WebGPURenderer({ alpha: true, depth: true, antialias: true });
        await customRenderer.init();
        this.renderer = customRenderer;
        this.world.renderer = new OBC.SimpleRenderer(this.components, this.container, this.renderer);
        this.world.camera = new OBC.OrthoPerspectiveCamera(this.components);
        this.components.init();
        const ifcLoader = this.components.get(OBC.IfcLoader);
        await ifcLoader.setup({
            autoSetWasm: false,
            wasm: {
                path: this.ifcPath,
                absolute: true,
            },
        });
        if (this.sizeFlag) {
            const grids = this.components.get(OBC.Grids);
            grids.create(this.world);
            await this.initializeAdvancedFeatures();
        }
        this.resize();
    }

    async loadIFC() {
        if (this.infoMessage) this.infoMessage.innerHTML = this.svgLoading;
        const buffer = await fetchWithCache(this.model.url, (percent) => {
            if (this.infoMessage) this.infoMessage.innerHTML = percent.toFixed(1) + '%';
        });
        const data = new Uint8Array(buffer);
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
            await this.initializeFragmentsManager();
            this.startRenderLoop();
            const ifcLoader = this.components.get(OBC.IfcLoader);
            const model = await ifcLoader.load(buffer, false, this.model.filename, {
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
        console.log('[FragmentsManager] Worker initialized');
        fragments.list.onItemSet.add(({ value: model }) => {
            if (!this.world?.camera?.three) {
                console.error('[FragmentsManager] Cannot add model - camera not ready');
                return;
            }
            model.useCamera(this.world.camera.three);
            this.world.scene.three.add(model.object);
            this.modelGroup = model.object;
            try {
                fragments.core.update(true);
            } catch (e) {
                console.warn('[FragmentsManager] Initial render error:', e.message);
            }
            console.log('[FragmentsManager] Model added to scene');
        });
        this._updateCoreBound = () => {
            if (!this.isInitialized || this.container?.clientHeight === 0) return;
            try {
                fragments.core.update(false);
            } catch (e) {
                console.warn('[FragmentsManager] Update error:', e.message);
            }
        };
        if (this.world?.camera?.controls) {
            this.world.camera.controls.addEventListener("update", this._updateCoreBound);
            this.world.camera.controls.addEventListener("rest", this._updateCoreBound);
        }
    }

    async initializeAdvancedFeatures() {
        this.stats = new Stats();
        this.container.appendChild(this.stats.dom);
    }

    updateStats() {
        if (!this.container || this.container.clientHeight === 0) {
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

    async updateViewPreset(viewName) {
        if (!this.modelGroup || !this.world?.camera) return;
        console.log(`Control view: ${viewName}`);
        this.viewModeController = viewName;
        const preset = this.viewPresets[viewName];
        if (!preset || !preset.pos) return;
        const box = new THREE.Box3().setFromObject(this.modelGroup);
        const size = box.getSize(new THREE.Vector3());
        const center = box.getCenter(new THREE.Vector3());
        const maxSize = Math.max(size.x, size.y, size.z);
        const cameraDistance = maxSize * 2.2;
        const [x, y, z] = preset.pos;
        if (this.world?.camera?.controls) {
            if (preset.projection && this.world.camera.projection !== preset.projection) {
                this.world.camera.projection = preset.projection;
            }
            this.world.camera.controls.isRotateable = !!preset.rot;
            await this.world.camera.controls.setLookAt(
                center.x + x * cameraDistance,
                center.y + y * cameraDistance,
                center.z + z * cameraDistance,
                center.x, center.y, center.z,
                true
            );
            this.updateStats();
        }
        if (this.gui) {
            const viewCtrl = this.gui.controllers.find(c => c.property === 'viewModeController');
            if (viewCtrl) viewCtrl.updateDisplay();
        }
    }

    goToOppositeView() {
        if (!this.world?.camera?.controls || !this.modelGroup) return;
        const camera = this.world.camera.three;
        const controls = this.world.camera.controls;

        const box = new THREE.Box3().setFromObject(this.modelGroup);
        const center = box.getCenter(new THREE.Vector3());
        const size = box.getSize(new THREE.Vector3());
        const maxDim = Math.max(size.x, size.y, size.z);

        const position = camera.position.clone();

        const offset = position.clone().sub(center);
        offset.negate();

        const minDistance = maxDim * 1.8;
        const maxDistance = maxDim * 15;
        const currentDistance = offset.length();
        if (currentDistance < minDistance) {
            offset.normalize().multiplyScalar(minDistance);
        } else if (currentDistance > maxDistance) {
            offset.normalize().multiplyScalar(maxDistance);
        }
        const newPosition = center.clone().add(offset);
        controls.setLookAt(
            newPosition.x, newPosition.y, newPosition.z,
            center.x, center.y, center.z,
            false
        );
        this.updateStats();
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
        if (this._isDestroying) return;
        this._isDestroying = true;
        console.log('Destroying GreatViewerIFC. Status was:', this.isInitialized);
        this.isInitialized = false;
        if (this.animationFrameId) {
            cancelAnimationFrame(this.animationFrameId);
            this.animationFrameId = null;
        }
        if (this.components) {
            try {
                const fragments = this.components.get(OBC.FragmentsManager);
                if (fragments && fragments.worker) {
                    fragments.worker.terminate();
                    fragments.worker = null;
                    console.log('Worker terminated via FragmentsManager');
                }
            } catch (e) {
                console.error('Failed to terminate via FragmentsManager:', e);
            }
        }
        if (this.modelGroup) {
            try {
                this.modelGroup.traverse((obj) => {
                    if (obj.isMesh) {
                        if (obj.geometry) obj.geometry.dispose();
                        if (obj.material) {
                            if (Array.isArray(obj.material)) {
                                obj.material.forEach(m => m.dispose());
                            } else {
                                obj.material.dispose();
                            }
                        }
                    }
                });
                if (this.world?.scene?.three) {
                    this.world.scene.three.remove(this.modelGroup);
                }
            } catch (e) {
                console.warn('Model cleanup error:', e);
            }
            this.modelGroup = null;
        }
        if (this.renderer) {
            try {
                this.renderer.dispose();
                if (this.renderer.domElement) {
                    this.renderer.domElement.remove();
                }
            } catch (e) {
                console.warn('Base renderer disposal error:', e);
            }
            this.renderer = null;
        }
        if (this.world?.renderer) {
            try {
                if (typeof this.world.renderer.forceContextLoss === 'function') {
                    this.world.renderer.forceContextLoss();
                }
                this.world.renderer.dispose();
            } catch (e) {
                console.warn('World renderer disposal error:', e);
            }
            this.world.renderer = null;
        }
        try {
            if (this.world) {
                if (this.world.scene) this.world.scene.dispose();
                if (this.world.camera) this.world.camera.dispose();
                this.world = null;
            }
        } catch (e) {
            console.warn('World disposal error:', e);
        }
        if (this.components) {
            try {
                const ifcLoader = this.components.get(OBC.IfcLoader);
                if (ifcLoader && typeof ifcLoader.cleanUp === 'function') {
                    ifcLoader.cleanUp();
                }
            } catch (e) {
                console.warn('IfcLoader cleanup skipped:', e);
            }
            try {
                this.components.dispose();
                console.log('[OBC] Components core disposed');
            } catch (e) {
                console.error('Error disposing components:', e);
            }
            this.components = null;
        }
        if (this.container) {
            this.container.textContent = '';
            this.container = null;
        }
        document.removeEventListener('keydown', this.handleKeyDown);
        console.log('GreatViewerIFC destroyed completely without errors');
        this._isDestroying = false;
    }
}