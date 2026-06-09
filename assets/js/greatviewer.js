import * as THREE from '../../../../three/three.webgpu.min.js';
import { STLLoader } from '../../../../three/loaders/STLLoader.js';
import { GLTFLoader } from '../../../../three/loaders/GLTFLoader.js';
import { DRACOLoader } from '../../../../three/loaders/DRACOLoader.js';
import { GCodeLoader } from '../../../../three/loaders/GCodeLoader.js';
import { OrbitControls } from '../../../../three/controls/OrbitControls.js';
import Stats from '../../../../three/stats.module.js';
import { GUI } from '../../../../three/lil-gui.esm.min.js';
import { fetchWithCache, clearModelCache } from '../../../../three/model-cache.js';

// Environment texture constants
const ENV_TEXTURES = {
    nxImg: `data:image/gif;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAAyUlEQVR42u3RMREAMAgAsVKb3DHiXwIyYMhL+ERXPu31LQAAQAAACAAAAQAgAAAEAIAAABAAAAIAQAAACAAAAQAgAAAEAIAAABAAAAIAQAAACAAAAQAgAAAEAIAAABAAAAIAQAAACAAAAQAgAAAEAIAAABAAAAAEAIAAABAAAAIAQAAACAAAAQAgAAAEAIAAABAAAAIAQAAACAAAAQAgAAAEAIAAABAAAAIAQAAACAAAAQAgAAAEAIAAABAAAAIAQAAACAAAAbjQAOAVAh/Yww3UAAAAAElFTkSuQmCC`,
    nyImg: `data:image/gif;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAAx0lEQVR42u3RMQEAMAjAsDGf8ODfAzLgSCU0UdlPe30LAAAQAAACAEAAAAgAAAEAIAAABACAAAAQAAACAEAAAAgAAAEAIAAABACAAAAQAAACAEAAAAgAAAEAIAAABACAAAAQAAACAEAAAAgAAAEAIAAABAAAAAEAIAAABACAAAAQAAACAEAAAAgAAAEAIAAABACAAAAQAAACAEAAAAgAAAEAIAAABACAAAAQAAACAEAAAAgAAAEAIAAABACAAAAQAAACAEAALjQfhwIey0nZ0AAAAABJRU5ErkJggg==`,
    nzImg: `data:image/gif;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAAxUlEQVR42u3RMQEAAAjDMMDY/LtCBhyphKaTlO4aCwAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAD60K4cBvajARSMAAAAASUVORK5CYII=`,
    pxImg: `data:image/gif;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAYAAADDPmHLAAAA8klEQVR42u3SQQ0AMAgAsTH/6rYHcsAGCa2Ey0X+V4e1rgQGwAAYAANgAAyAATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADYAADSGAADIABMAAGwAAYAANgAAyAATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADMEMDMPMEe7fNplYAAAAASUVORK5CYII=`,
    pyImg: `data:image/gif;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAAxklEQVR42u3RMQEAAAQAQfQPZBRNDIb7CH/ZPaG7ygIAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABAABAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEIAPLTJyAzCjdRU4AAAAAElFTkSuQmCC`,
    pzImg: `data:image/gif;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAYAAADDPmHLAAAA8ElEQVR42u3SgQkAMAjAsLnzN/BlfUMwOaE08v06rHUlMAAGwAAYAANgAAyAATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADGEACA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGIAZGrldA6KZLJDPAAAAAElFTkSuQmCC`,
};

const COLORS = {
    cdbs_blue: 0x1872f0,    // #1872f0
    cyan: 0x00ffff,         // #00ffff
    red: 0xff0000,          // #ff0000
    green: 0x00ff00,        // #00ff00
    dark_blue: 0x0000ff,    // #0000ff
    yellow: 0xffff00,       // #ffff00
    magenta: 0xff00ff,      // #ff00ff
    white: 0xffffff,        // #ffffff
    dark: 0x000000,         // #000000
};

const modelCache = new Map();

export class GreatViewer {
    constructor(config) {
        ({
            model: this.model,
            model_format: this.modelFormat,
            resource_mapping: this.resourceMapping,
            size_flag: this.sizeFlag,
            labels: this.labels
        } = config);

        this.startTime = null;
        this.isInitialized = false;
        this.initPromise = null;
        // Materials
        this.envTexture = new THREE.CubeTextureLoader().load([
            ENV_TEXTURES.pxImg, //right
            ENV_TEXTURES.nxImg, //left
            ENV_TEXTURES.pyImg, //top
            ENV_TEXTURES.nyImg, //bottom
            ENV_TEXTURES.pzImg, //back
            ENV_TEXTURES.nzImg, //front
        ]);
        this.envTexture.mapping = THREE.CubeReflectionMapping;
        this.material = new THREE.MeshPhysicalMaterial({
            color: COLORS['cdbs_blue'],
            envMap: this.envTexture,
            metalness: 0.5,
            roughness: 0.5,
            opacity: 1.0,
            transparent: false,
            clearcoat: 0.2,
            clearcoatRoughness: 0.1
        });
        this.lineMaterialActive = new THREE.LineBasicMaterial({
            color: COLORS['cdbs_blue'],
            linewidth: 1,
            transparent: false
        });
        this.lineMaterial = new THREE.LineBasicMaterial({
            color: COLORS['cyan'],
            linewidth: 1,
            transparent: false
        });
        this.wireMaterial = new THREE.MeshBasicMaterial({
            color: COLORS['cdbs_blue'],
            wireframe: true
        });
        this.lightParams = {
            ambientIntensity: 0.2,
            hemisphereIntensity: 0.4,
            keyIntensity: 1.2,
            fillIntensity: 0.5,
            rimIntensity: 0.4,
            keyPosition: new THREE.Vector3(5, 10, 7),
            fillPosition: new THREE.Vector3(-4, 2, 4),
            rimPosition: new THREE.Vector3(-2, 3, -5)
        };

        this.animationFrameId = null;
        this.container = null;
        this.gui = null;
        this.scene = null;
        this.renderer = null;
        this.camera = null;
        this.controls = null;
        this.stats = null;
        this.mesh = null;
        this.resizeObserver = null;
        this.originalMaterials = new Map();
        this.hasTextures = false;
        this.quality = 'high';
        // Animation
        this.mixer = null;
        this.animations = [];
        this.animationActions = new Map();
        this.isPlayingAnimation = false;
        this.animationSpeed = 1.0;
        // State management
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
        this.showAxesHelper = false;
        this.sceneRotation = false;
        this.axesHelper = null;
        this.infoMessage = null;
        this.useCustomMaterial = false;
        this.isWireframe = false;
        // GCode
        this.parsedLayers = [];
        this.gcodeLayers = [];
        this.currentGCodeLayer = 0;
        this.updateLayerControls = null; // Function to update controls
        this.displayMode = this.labels.display_up_to_current;
        // Hotkeys handler
        this.handleKeyDown = this.handleKeyDown.bind(this);
        document.addEventListener('keydown', this.handleKeyDown);
    }

    handleKeyDown(e) {
        if (!this.isInitialized) return;
        if (e.code === 'Space' || e.code === 'KeyF') {
            e.preventDefault();
        }
        if (e.code === 'KeyF' && !this.sizeFlag) {
            document.querySelector('#three-size-button')?.click();
            return;
        }
        if (e.code === 'Escape' && this.sizeFlag) {
            document.querySelector('#three-modal-close-btn')?.click();
            return;
        }
        if (e.code === 'Space' && this.mixer) {
            this.toggleAnimation(!this.isPlayingAnimation);
            return;
        }
        if (e.code === 'Space' && this.sizeFlag) {
            if (this.currentGCodeLayer >= this.gcodeLayers.length - 1) {
                this.currentGCodeLayer = 0;
            }
            this.toggleSlicerAnimation(!this.isPlayingAnimation);
            return;
        }
        if (e.code === 'KeyH' || e.code === 'Digit0' || e.code === 'Numpad0') {
            this.controls?.reset();
            return;
        }
        if (e.code === 'KeyZ') {
            this.isWireframe = !this.isWireframe;
            if (this.isWireframe) this.useCustomMaterial = true;
            this.updateMaterial();
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

    setQuality(level) {
        this.quality = level;
        let pixelRatio = window.devicePixelRatio;
        switch(level) {
            case 'low':
                pixelRatio = Math.min(pixelRatio, 1);
                break;
            case 'medium':
                pixelRatio = Math.min(pixelRatio, 1.5);
                break;
            case 'high':
                pixelRatio = Math.min(pixelRatio, 2);
                break;
        }
        this.renderer.setPixelRatio(pixelRatio);
    }

    centerAndFitCamera() {
        if (!this.mesh) return;
        const box = new THREE.Box3().setFromObject(this.mesh);
        const center = box.getCenter(new THREE.Vector3());
        const size = box.getSize(new THREE.Vector3());
        const maxDim = Math.max(size.x, size.y, size.z);
        const distance = maxDim * 1.5;
        this.mesh.position.sub(center);
        this.camera.position.set(distance, distance, distance);
        this.controls.target.set(0, 0, 0);
        this.controls.update();
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
        let backgroundColor = '#fff';
        console.log(`Full screen mode: ${this.sizeFlag}`);
        if (this.sizeFlag) {
            container_tag = 'b-container';
        }
        // console.log(`container_tag: ${container_tag}`);
        this.container = sceneHull.querySelector(container_tag);

        let clientWidth = this.container.clientWidth;
        let clientHeight = this.container.clientHeight;

        // Create a Scene
        this.scene = new THREE.Scene();

        // Light
        this.ambientLight = new THREE.AmbientLight(0xffffff, this.lightParams.ambientIntensity);
        this.scene.add(this.ambientLight);
        this.hemisphereLight = new THREE.HemisphereLight(0x88aaff, 0x443322, this.lightParams.hemisphereIntensity);
        this.scene.add(this.hemisphereLight);
        this.keyLight = new THREE.DirectionalLight(0xffffff, this.lightParams.keyIntensity);
        this.keyLight.position.copy(this.lightParams.keyPosition);
        this.scene.add(this.keyLight);
        this.fillLight = new THREE.DirectionalLight(0xaaccff, this.lightParams.fillIntensity);
        this.fillLight.position.copy(this.lightParams.fillPosition);
        this.scene.add(this.fillLight);
        this.rimLight = new THREE.DirectionalLight(0xffaa88, this.lightParams.rimIntensity);
        this.rimLight.position.copy(this.lightParams.rimPosition);
        this.scene.add(this.rimLight);

        this.axesHelper = new THREE.AxesHelper(30);

        // Set the background color
        this.scene.background = new THREE.Color(backgroundColor);

        // Set scaling to 1/2 by default
        let customScale = 0.5;
        this.scene.scale.set(customScale, customScale, customScale);

        // Create a camera
        const fov = 45; // AKA Field of View
        const near = 0.1; // The near clipping plane
        const far = 5000; // The far clipping plane

        const aspect = clientWidth / clientHeight;
        this.camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
        this.camera.position.set(0, 0, 70);

        // Create the renderer
        this.renderer = await this.createRenderer();
        this.isInitialized = true;

        // Next, set the renderer to the same size as our container element
        this.renderer.setSize(clientWidth, clientHeight);

        // Optimization for high resolutions
        const pixelRatio = window.devicePixelRatio;
        this.renderer.setPixelRatio(Math.min(pixelRatio, 2));

        // Add the automatically created <canvas> element to the page
        this.container.append(this.renderer.domElement);

        this.controls = new OrbitControls(this.camera, this.renderer.domElement);
        this.controls.enableDamping = true;

        this.infoMessage = document.createElement('div');
        this.infoMessage.classList.add('text-center');
        this.container.appendChild(this.infoMessage);

        // Start loading
        this.loadModel();

        if (this.sizeFlag) {
            this.stats = new Stats();
            // Show only in full screen mode
            this.container.appendChild(this.stats.dom); // Show statistics
        }
        // Use ResizeObserver to track size changes
        this.setupResizeObserver(clientWidth, clientHeight)
        this.startAnimation();
    }

    setupResizeObserver(clientWidth, clientHeight) {
        this.resizeObserver = new ResizeObserver((entries) => {
            for (let entry of entries) {
                const { width, height } = entry.contentRect;
                if (width !== clientWidth || height !== clientHeight) {
                    clientWidth = width;
                    clientHeight = height;
                    this.renderer.setSize(clientWidth, clientHeight);
                    this.camera.aspect = clientWidth / clientHeight;
                    this.camera.updateProjectionMatrix();
                }
            }
        });
        this.resizeObserver.observe(this.container);
    }

    /**
     * Initializes the unified WebGPURenderer with an automatic WebGL2 fallback.
     * @returns {Promise<THREE.WebGPURenderer>} The initialized renderer instance.
     */
    async createRenderer() {
        const isWebGPUSupported = !!(navigator.gpu && navigator.gpu.requestAdapter);
        if (!isWebGPUSupported) {
            console.warn('CADBase Viewer: WebGPU API is missing in this browser. Falling back to WebGL2.');
        }
        const renderer = new THREE.WebGPURenderer({
            alpha: true,
            depth: true,
            stencil: false,
            antialias: true
        });
        try {
            await renderer.init();
            console.log(`CADBase Viewer: Renderer successfully initialized on backend: "${renderer.backend.name}"`);
        } catch (error) {
            console.error('CADBase Viewer: Critical error during renderer initialization:', error);
            throw error;
        }

        return renderer;
    }

    startAnimation() {
        // Cancel existing animation if any
        if (this.animationFrameId) {
            cancelAnimationFrame(this.animationFrameId);
            this.animationFrameId = null;
        }
        // Animation loop
        const animate = () => {
            // Check if viewer is still valid
            if (!this.isInitialized || !this.container || !this.renderer || !this.scene || !this.camera) {
                return;
            }
            // Check if container is still in DOM
            if (this.container.clientHeight === 0) {
                this.destroy();
                return;
            }
            this.animationFrameId = requestAnimationFrame(animate);
            // Update light points
            this.updateLightToCamera()
            // Update animation mixer
            if (this.mixer) {
                const delta = 0.016; // ~60FPS
                this.mixer.update(delta * this.animationSpeed);
            }
            // Update statistics
            if (this.stats) {
                this.stats.update();
            }
            // Update scene rotation
            if (this.sceneRotation) {
                this.scene.rotation.x += 0.005;
                this.scene.rotation.y += 0.01;
            }
            this.renderer.render(this.scene, this.camera);
        };

        // Start animation
        animate();
    }

    onProgress(xhr) {
        if (this.infoMessage) {
            let loadedProgress = (xhr.loaded / xhr.total) * 100;
            this.infoMessage.innerHTML = loadedProgress.toFixed(1) + '%';
        }
    }

    onError(error) {
        console.warn(error);
        if (this.infoMessage) {
            this.infoMessage.innerHTML = this.labels.failed_to_load_model + ': ' + error.message;
        }
    }

    onComplete() {
        // Remove the loading message
        if (this.infoMessage && this.infoMessage.parentNode) {
            this.infoMessage.parentNode.removeChild(this.infoMessage);
            this.infoMessage = null;
        }
        // Calculate execution time and display the result
        const loadTime = (performance.now() - this.startTime) / 1000;
        console.log(`Loaded file '${this.model.filename}' in ${loadTime.toFixed(3)} s`);
        if (performance.memory) {
            const mem = performance.memory;
            const usedMB = Math.round(mem.usedJSHeapSize / 1048576);
            const totalMB = Math.round(mem.totalJSHeapSize / 1048576);
            const limitMB = Math.round(mem.jsHeapSizeLimit / 1048576);
            console.log(
                `  Working Set:    ${usedMB} MiB (peak: ${totalMB} MiB)\n` +
                `  Virtual memory: ${limitMB} MiB\n` +
                `  Heap memory:    ${usedMB} MiB`
            );
        }
        if (!this.sizeFlag) {
            this.updateViewPreset(this.labels.view_isometric);
            return;
        }
        this.setControlsGui();
        if (!this.stats) return;
        this.container.appendChild(this.stats.dom);
        if (this.modelFormat == 'GCode') {
            this.updateViewPreset(this.labels.view_top);
            this.updateGCodeLayers();
        }
    }

    loadModel() {
        console.log('=== STARTING LOAD ===');
        console.log('Model format:', this.modelFormat);
        console.log('Resource mapping count:', this.resourceMapping?.length || 0);
        console.log('Model path:', this.model.url);
        switch (this.modelFormat) {
            case 'STL':
                fetchWithCache(this.model.url, (percent) => { this.infoMessage.innerHTML = percent.toFixed(1) + '%'; })
                    .then(buffer => {
                        const stlLoader = new STLLoader();
                        const geometry = stlLoader.parse(buffer);
                        this.mesh = new THREE.Mesh(geometry, this.material);
                        this.scene.add(this.mesh);
                        geometry.center();
                        this.centerAndFitCamera();
                        this.onComplete();
                    })
                    .catch(error => this.onError(error));
                break;
            // GLTF and GLB use GLTFLoader
            case 'GLTF':
                const gltfResources = this.createGLTFLoaderWithDraco();
                if (this.resourceMapping.length === 0) {
                    fetchWithCache(this.model.url, (percent) => { this.infoMessage.innerHTML = percent.toFixed(1) + '%'; })
                        .then(buffer => {
                            gltfResources.loader.parse(
                                buffer,
                                '',
                                (gltf) => {
                                    this.processGltfModel(gltf);
                                    gltfResources.dispose();
                                },
                                (xhr) => this.onProgress(xhr),
                                (error) => {
                                    this.onError(error);
                                    gltfResources.dispose();
                                }
                            );
                        })
                        .catch(error => {
                            this.onError(error);
                            gltfResources.dispose();
                        });
                    return;
                }
                this.loadGltf(gltfResources);
                break;
            case 'GLB':
                const glbResources = this.createGLTFLoaderWithDraco();
                fetchWithCache(this.model.url, (percent) => { this.infoMessage.innerHTML = percent.toFixed(1) + '%'; })
                    .then(buffer => {
                        glbResources.loader.parse(
                            buffer,
                            '',
                            (glb) => {
                                this.mesh = glb.scene;
                                this.store_animations(glb);
                                this.optimizeQualityModel();
                                this.scene.add(this.mesh);
                                this.centerAndFitCamera();
                                this.onComplete();
                                glbResources.dispose();
                            },
                            (xhr) => this.onProgress(xhr),
                            (error) => {
                                this.onError(error);
                                glbResources.dispose();
                            }
                        );
                    })
                    .catch(error => {
                        this.onError(error);
                        glbResources.dispose();
                    });
                break;
            case 'GCode':
                if (this.sizeFlag) {
                    fetchWithCache(this.model.url, (percent) => { this.infoMessage.innerHTML = percent.toFixed(1) + '%'; })
                        .then(buffer => {
                            const decoder = new TextDecoder();
                            const text = decoder.decode(buffer);
                            this.parsedLayers = this.parseGCodeLayers(text);
                            const layers = this.parsedLayers.map(l => l.text);
                            const loader = new GCodeLoader();
                            const combined = new THREE.Group();
                            layers.forEach((layerText, i) => {
                                try {
                                    const object = loader.parse(layerText);
                                    object.name = `layer_${i}`;
                                    object.visible = this.getLayerVisibility(i);
                                    combined.add(object);
                                } catch (e) { console.error(e); }
                            });
                            this.processGCodeModel(combined);
                        })
                        .catch(error => this.onError(error));
                } else {
                    fetchWithCache(this.model.url, (percent) => { this.infoMessage.innerHTML = percent.toFixed(1) + '%'; })
                        .then(buffer => {
                            const decoder = new TextDecoder();
                            const text = decoder.decode(buffer);
                            const loader = new GCodeLoader();
                            const object = loader.parse(text);
                            this.processGCodeModel(object);
                        })
                        .catch(error => this.onError(error));
                }
                break;
            default:
                console.error(`Unsupported format: ${this.modelFormat}`);
                if (this.infoMessage) {
                    this.infoMessage.innerHTML = this.labels.format_not_supported + ': ' + this.modelFormat;
                }
                break;
        }
    }

    addVector3Controls(folder, obj, name, min = -50, max = 50, step = 1) {
        const controls = folder.addFolder(name);
        controls.close();
        controls.add(obj, 'x', min, max, step).name('X');
        controls.add(obj, 'y', min, max, step).name('Y');
        controls.add(obj, 'z', min, max, step).name('Z');
    }

    setControlsGui() {
        // Remove the old GUI if it exists
        if (this.gui) {
            this.gui.destroy();
        }
        const isGLTF = this.modelFormat == 'GLTF' || this.modelFormat == 'GLB';
        const isSTL = this.modelFormat == 'STL';
        const isGCode = this.modelFormat == 'GCode';
        console.log(`Controls: isGLTF - ${isGLTF}, isSTL - ${isSTL}, isGCode - ${isGCode}`);
        this.gui = new GUI({
            autoPlace: false,
            title: this.labels.controls,
            container: this.container
        });
        this.gui.domElement.id = 'three-gui';
        this.gui.add(this, 'viewModeController', Object.keys(this.viewPresets))
            .name(this.labels.view)
            .onChange(viewName => this.updateViewPreset(viewName));
        const controlParams = {
            showAxesHelper: this.showAxesHelper,
            sceneRotation: this.sceneRotation,
            backgroundColor: this.scene.background.getHex(),
            materialMeshColor: this.material.color.getHex(),
            customScale: 0.5,
        };
        this.gui.add(controlParams, 'showAxesHelper')
            .name(this.labels.axes)
            .onChange((value) => {
                if (value) {
                    this.scene.add(this.axesHelper);
                } else {
                    this.scene.remove(this.axesHelper);
                }
                this.showAxesHelper = value;
            });
        this.gui.add(controlParams, 'sceneRotation')
            .name(this.labels.rotation)
            .onChange((value) => this.sceneRotation = value);
        this.gui.add(controlParams, 'customScale', 0.01, 2)
            .name(this.labels.model_scale)
            .onChange((value) => this.scene.scale.set(value, value, value));

        // GCode specific controls
        if (isGCode && this.gcodeLayers && this.gcodeLayers.length > 0) {
            const gcodeFolder = this.gui.addFolder('GCode');
            gcodeFolder.open();
            // Play/Pause toggle
            gcodeFolder.add(this, 'isPlayingAnimation')
                .name(this.labels.play)
                .onChange((value) => this.toggleSlicerAnimation(value));
            // Animation speed
            gcodeFolder.add(this, 'animationSpeed', 0, 5, 0.1)
                .name(this.labels.speed)
                .onChange((value) => {
                    this.animationActions.forEach(action => {
                        action.timeScale = value;
                    });
                });
            const params = {
                displayMode: this.displayMode || this.labels.display_up_to_current,
                currentLayer: this.currentGCodeLayer || 0,
                hideTravelMoves: false
            };
            // Slider
            if (this.gcodeLayers.length > 1) {
                gcodeFolder.add(params, 'currentLayer', 0, this.gcodeLayers.length - 1, 1)
                    .name(this.labels.active_layer)
                    .onChange(v => {
                        this.currentGCodeLayer = v;
                        this.updateGCodeLayers();
                    });
            }
            // Hide red lines
            gcodeFolder.add(params, 'hideTravelMoves')
                .name(this.labels.hide_travel_moves)
                .onChange((value) => {
                    this.gcodeLayers.forEach(layer => {
                        if (layer.object && layer.object.children[1]) {
                            layer.object.children[1].visible = !value;
                        }
                    });
                });
            // Mode toggle
            gcodeFolder.add(params, 'displayMode', [
                    this.labels.display_all,
                    this.labels.display_up_to_current,
                    this.labels.display_current_only
                ])
                .name(this.labels.display)
                .onChange(v => {
                    this.displayMode = v;
                    this.updateGCodeLayers();
                });
        }

        console.log(`Controls (GLTF/GLB): this.mixer ${this.mixer}, this.animationActions.size ${this.animationActions.size}`);
        // For complex GLTF/GLB models with animations
        if (isGLTF && this.mixer && this.animationActions.size > 0) {
            const gltfFolder = this.gui.addFolder('GLTF/GLB');
            gltfFolder.open();
            // Play/Pause toggle
            gltfFolder.add(this, 'isPlayingAnimation')
                .name(this.labels.play)
                .onChange((value) => this.toggleAnimation(value));
            // Animation speed
            gltfFolder.add(this, 'animationSpeed', 0, 2, 0.1)
                .name(this.labels.speed)
                .onChange((value) => {
                    this.animationActions.forEach(action => {
                        action.timeScale = value;
                    });
                });
            // Animation selector
            const animationNames = Array.from(this.animationActions.keys());
            if (animationNames.length > 0) {
                let currentAnimation = animationNames[0];
                gltfFolder.add({ animation: currentAnimation }, 'animation', animationNames)
                    .name(this.labels.select)
                    .onChange((value) => {
                        this.stopAllAnimations();
                        const action = this.animationActions.get(value);
                        if (action) {
                            action.play();
                            this.isPlayingAnimation = true;
                        }
                    });
            }
        }
        // Material control folder
        const materialFolder = this.gui.addFolder(this.labels.material_folder);
        materialFolder.close();
        // Texture/material switching (only for GLTF/GLB with textures)
        if (isGLTF && this.hasTextures) {
            materialFolder.add(this, 'useCustomMaterial')
                .name(this.labels.hide_textures)
                .onChange(() => {
                    if (!this.useCustomMaterial) this.isWireframe = false;
                    this.updateMaterial();
                });
        }
        // Wireframe control (for non-GCode models)
        if (!isGCode) {
            materialFolder.add(this, 'isWireframe')
                .name(this.labels.wireframe)
                .onChange(() => {
                    if (this.isWireframe) this.useCustomMaterial = true;
                    this.updateMaterial();
                });
        }
        materialFolder.addColor(controlParams, 'materialMeshColor')
            .name(this.labels.model_color)
            .onChange(color => this.setMaterialColor(color));
        // Background color
        materialFolder.addColor(controlParams, 'backgroundColor')
            .name(this.labels.background_color)
            .onChange((value) => this.scene.background.set(value));
        materialFolder.add(this.material, 'metalness', 0, 1, 0.05).name(this.labels.metalness);
        materialFolder.add(this.material, 'roughness', 0, 1, 0.05).name(this.labels.roughness);
        if (this.material.envMap) {
            materialFolder.add(this.material, 'envMapIntensity', 0, 2, 0.1).name(this.labels.env_intensity);
        }
        // MeshPhysicalMaterial
        if (this.material.clearcoat !== undefined) {
            materialFolder.add(this.material, 'clearcoat', 0, 1, 0.05).name(this.labels.clearcoat);
            materialFolder.add(this.material, 'clearcoatRoughness', 0, 1, 0.05).name(this.labels.clearcoat_rough);
        }
        // Light folder
        const lightFolder = this.gui.addFolder(this.labels.lighting_folder);
        lightFolder.close();
        lightFolder.add(this.lightParams, 'ambientIntensity', 0, 1, 0.01)
            .name(this.labels.ambient || 'Ambient')
            .onChange(v => this.ambientLight.intensity = v);
        lightFolder.add(this.lightParams, 'hemisphereIntensity', 0, 1, 0.01)
            .name(this.labels.hemisphere || 'Hemisphere')
            .onChange(v => this.hemisphereLight.intensity = v);
        lightFolder.add(this.lightParams, 'keyIntensity', 0, 2, 0.01)
            .name(this.labels.key_light || 'Key Light')
            .onChange(v => this.keyLight.intensity = v);
        lightFolder.add(this.lightParams, 'fillIntensity', 0, 1.5, 0.01)
            .name(this.labels.fill_light || 'Fill Light')
            .onChange(v => this.fillLight.intensity = v);
        lightFolder.add(this.lightParams, 'rimIntensity', 0, 1, 0.01)
            .name(this.labels.rim_light || 'Rim Light')
            .onChange(v => this.rimLight.intensity = v);
        this.addVector3Controls(lightFolder, this.lightParams.keyPosition, this.labels.key_light_position || 'Key Light Position');
        this.addVector3Controls(lightFolder, this.lightParams.fillPosition, this.labels.fill_light_position || 'Fill Light Position');
        this.addVector3Controls(lightFolder, this.lightParams.rimPosition, this.labels.rim_light_position || 'Rim Light Position');
        // Info folder
        const infoFolder = this.gui.addFolder(this.labels.model_info_folder);
        infoFolder.close();
        infoFolder.add(this.model, 'filename').name(this.labels.file).listen();
        infoFolder.add(this.model, 'size').name(this.labels.size).listen();
    }

    // Remove old updateLightToCamera method or keep only for keyLight:
    updateLightToCamera() {
        if (!this.camera || !this.keyLight) return;
        // Only key light follows camera for consistent illumination
        this.keyLight.position.copy(this.camera.position).add(this.lightParams.keyPosition);
        this.keyLight.target.position.copy(this.camera.position);
        this.keyLight.target.position.add(this.camera.getWorldDirection(new THREE.Vector3()).multiplyScalar(10));
    }

    updateViewPreset(viewName) {
        console.log(`Control view: ${viewName}`);
        if (!this.mesh) return;
        this.viewModeController = viewName;
        const preset = this.viewPresets[viewName];
        if (!preset || !preset.pos) return;
        const box = new THREE.Box3().setFromObject(this.mesh);
        const center = box.getCenter(new THREE.Vector3());
        const size = box.getSize(new THREE.Vector3());
        const maxSize = Math.max(size.x, size.y, size.z);
        const distance = maxSize * 1.5;
        const [x, y, z] = preset.pos;
        const newPosX = center.x + x * distance;
        const newPosY = center.y + y * distance;
        const newPosZ = center.z + z * distance;
        this.camera.position.set(newPosX, newPosY, newPosZ);
        if (this.controls) {
            this.controls.target.copy(center);
            this.controls.enableRotate = !!preset.rot;
            this.controls.update();
        } else {
            this.camera.lookAt(center);
        }
        if (this.gui) {
            const viewCtrl = this.gui.controllers.find(c => c.property === 'viewModeController');
            if (viewCtrl) viewCtrl.updateDisplay();
        }
    }

    store_animations(object) {
        console.log(`GLTF animations count: ${object.animations?.length || 0}`);
        // Store animations if any
        if (object.animations && object.animations.length > 0) {
            this.animations = object.animations;
            this.mixer = new THREE.AnimationMixer(this.mesh);
            object.animations.forEach((clip, i) => {
                const action = this.mixer.clipAction(clip);
                this.animationActions.set(clip.name || `animation_${i}`, action);
            });
        }
    }

    processGCodeModel(object) {
        if (!object) return;
        this.scene.add(object);
        this.mesh = object;
        // Centering
        object.updateMatrix();
        const box = new THREE.Box3().setFromObject(object);
        const center = box.getCenter(new THREE.Vector3());
        object.position.sub(center);
        this.gcodeLayers = object.children.map((child, i) => {
            const layerInfo = this.parsedLayers?.[i];
            return {
                index: i,
                object: child,
                visible: this.getLayerVisibility(i), // Reference to the actual object
                number: layerInfo?.number || i, // Determine initial visibility
                z: layerInfo?.z || i * 0.2 // Function to get the height Z
            };
        });
        this.centerAndFitCamera();
        this.onComplete();
    }

    updateGCodeLayers() {
        if (!this.gcodeLayers || !this.mesh) return;
        this.gcodeLayers.forEach((layer, i) => {
            if (!layer.object) return;
            layer.object.visible = this.getLayerVisibility(i);
            if (layer.object.visible && layer.object.children[0]?.isLineSegments) {
                if (i === this.currentGCodeLayer) {
                    layer.object.children[0].material = this.lineMaterialActive;
                } else {
                    layer.object.children[0].material = this.lineMaterial;
                }
            }
        });
    }

    toggleSlicerAnimation(play) {
        this.isPlayingAnimation = play;
        if (play) {
            let lastTime = performance.now();
            let progress = this.currentGCodeLayer;
            const animate = (time) => {
                if (!this.isPlayingAnimation || !this.isInitialized) return;
                const delta = (time - lastTime) / 1000;
                progress += delta * this.animationSpeed;
                lastTime = time;
                if (progress >= this.gcodeLayers.length) {
                    this.isPlayingAnimation = false;
                    progress = this.gcodeLayers.length - 1;
                }
                this.currentGCodeLayer = Math.floor(progress);
                this.updateGCodeLayers();
                if (this.isPlayingAnimation) requestAnimationFrame(animate);
            };
            requestAnimationFrame(animate);
        }
    }

    updateMaterial() {
        if (!this.mesh) return;
        const mat = this.isWireframe ? this.wireMaterial : this.useCustomMaterial ? this.material : null;
        this.mesh.traverse(child => {
            if (child.isMesh) {
                child.material = mat || this.originalMaterials.get(child.uuid) || child.material;
            }
        });
    }

    setMaterialColor(color) {
        if (this.modelFormat == 'GCode') {
            this.gcodeLayers?.forEach(layer => {
                layer.object?.children[0]?.material?.color.set(color);
            });
        } else if (this.material) {
            this.material.color.set(color);
            if (this.isWireframe) {
                this.mesh?.traverse(child => {
                    if (child.isMesh) child.material.color.set(color);
                });
            }
        }
    }

    getLayerVisibility(index) {
        if (this.displayMode === this.labels.display_all) return true;
        if (this.displayMode === this.labels.display_current_only) return index === this.currentGCodeLayer;
        return index <= this.currentGCodeLayer;
    }

    parseGCodeLayers(t) {
        const layers = [];
        let currentLayer = null;
        let currentText = '';
        const lines = t.split('\n');
        for (const line of lines) {
            if (line.startsWith(';LAYER:')) {
                // Save the previous layer
                if (currentLayer !== null) {
                    layers.push({
                        text: currentText,
                        number: currentLayer,
                        z: this.extractZFromLayer(currentText) || currentLayer * 0.2
                    });
                }
                // Start a new layer
                currentLayer = parseInt(line.slice(7));
                currentText = line + '\n';
            } else if (currentLayer !== null) {
                currentText += line + '\n';
            }
        }
        // The last layer
        if (currentLayer !== null) {
            layers.push({
                text: currentText,
                number: currentLayer,
                z: this.extractZFromLayer(currentText) || currentLayer * 0.2
            });
        }
        return layers;
    }

    extractZFromLayer(layerText) {
        const zMatch = layerText.match(/Z(-?\d+\.?\d*)/);
        return zMatch ? parseFloat(zMatch[1]) : null;
    }

    createGLTFLoaderWithDraco() {
        const gltfLoader = new GLTFLoader();
        const dracoLoader = new DRACOLoader();
        dracoLoader.setDecoderPath('../../../../three/draco/');
        dracoLoader.setDecoderConfig({ type: 'wasm' }); // 'js'/'wasm'
        gltfLoader.setDRACOLoader(dracoLoader);
        return {
            loader: gltfLoader,
            // Helper function for cleanup
            dispose: () => dracoLoader.dispose()
        };
    }

    loadGltf(gltfResources) {
        // Load and process GLTF with resources
        fetch(this.model.url, { cache: 'default' })
        .then(response => {
            if (!response.ok) throw new Error(`HTTP ${response.status}`);
            return response.json();
        })
        .then(gltfData => {
            const resourceMap = new Map();
            this.resourceMapping.forEach(item => {
                resourceMap.set(item.filename, item.download_url);
            });
            // fix links to URLs
            this.updateGltfResourceUris(gltfData, resourceMap);
            const modifiedJson = JSON.stringify(gltfData);
            const blob = new Blob([modifiedJson], { type: 'model/gltf+json' });
            const blobUrl = URL.createObjectURL(blob);
            gltfResources.loader.load(
                blobUrl,
                (gltf) => {
                    this.processGltfModel(gltf);
                    URL.revokeObjectURL(blobUrl);
                    gltfResources.dispose();
                },
                (xhr) => this.onProgress(xhr),
                (error) => {
                    this.onError(error);
                    URL.revokeObjectURL(blobUrl);
                    gltfResources.dispose();
                }
            );
        })
        .catch(error => {
            this.onError(error);
            gltfResources.dispose();
        });
    }

    optimizeQualityModel() {
        if (!this.mesh) return;
        let totalTriangles = 0;
        this.mesh.traverse((child) => {
            if (child.isMesh && child.geometry) {
                const count = child.geometry.index ? child.geometry.index.count / 3 : child.geometry.attributes.position.count / 3;
                totalTriangles += count;
            }
        });
        console.log(`Model has ${totalTriangles.toLocaleString()} triangles`);
        if (totalTriangles > 3000000) {
            this.setQuality('low');
            console.warn('Very large model detected, setting low quality');
        } else if (totalTriangles > 500000) {
            this.setQuality('medium');
            console.warn('Large model detected, reducing quality');
        }
    }

    // Helper functions
    updateGltfResourceUris(gltfData, resourceMap) {
        // Update buffer URIs
        if (gltfData.buffers) {
            gltfData.buffers.forEach(buffer => {
                if (buffer.uri && resourceMap.has(buffer.uri)) {
                    console.log(`Updating buffer ${buffer.uri} -> ${resourceMap.get(buffer.uri)}`);
                    buffer.uri = resourceMap.get(buffer.uri);
                }
            });
        }
        // Update image URIs
        if (gltfData.images) {
            gltfData.images.forEach(image => {
                if (image.uri && resourceMap.has(image.uri)) {
                    image.uri = resourceMap.get(image.uri);
                }
            });
        }
    }

    processGltfModel(object) {
        this.mesh = object.scene;
        this.originalMaterials.clear();
        this.store_animations(object);
        this.optimizeQualityModel();
        this.scene.add(this.mesh);
        // Center the model
        this.centerAndFitCamera();
        this.onComplete();
    }

    toggleAnimation(play) {
        this.isPlayingAnimation = play;
        if (play) {
            // Play all animations
            this.animationActions.forEach(action => {
                action.reset();
                action.play();
            });
        } else {
            // Stop all animations
            this.stopAllAnimations();
        }
    }

    stopAllAnimations() {
        this.animationActions.forEach(action => {
            action.stop();
        });
    }

    playAnimation(name) {
        const action = this.animationActions.get(name);
        if (action) {
            this.stopAllAnimations();
            action.reset();
            action.play();
            this.isPlayingAnimation = true;
            return true;
        }
        return false;
    }

    destroy() {
        // Removing GUI
        this.gui?.destroy();
        // Clearing previous model before loading new one
        if (this.mesh && this.scene) {
            this.scene.remove(this.mesh);
            if (this.mesh.geometry) this.mesh.geometry.dispose();
            if (this.mesh.material) this.mesh.material.dispose();
            this.mesh = null;
        }
        // Clearing Three.js objects
        this.controls?.dispose();
        if (this.animationFrameId) {
            cancelAnimationFrame(this.animationFrameId);
            this.animationFrameId = null;
        }
        this.mixer?.stopAllAction();
        // Clearing materials cache
        this.originalMaterials.clear();
        // Clearing model cache
        modelCache.clear();
        if (this.resizeObserver) {
            this.resizeObserver.disconnect();
        }
        // Clearing DOM
        if (this.container) {
            this.container.textContent = '';
            this.container = null;
        }
        // Mark as destroyed
        this.isInitialized = false;
        // Unsubscribing from events
        document.removeEventListener('keydown', this.handleKeyDown);
    }
}