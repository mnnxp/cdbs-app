import * as THREE from '../../../../three/three.webgpu.min.js';
// import { WebGPURenderer } from '../../../../three/webgpu/WebGPURenderer.js';
import { STLLoader } from '../../../../three/loaders/STLLoader.js';
import { GLTFLoader } from '../../../../three/loaders/GLTFLoader.js';
import { DRACOLoader } from '../../../../three/loaders/DRACOLoader.js';
import { GCodeLoader } from '../../../../three/loaders/GCodeLoader.js';
import { OrbitControls } from '../../../../three/OrbitControls.js';
import Stats from '../../../../three/stats.module.js';
import { GUI } from '../../../../three/lil-gui.esm.min.js';

// Environment texture constants moved outside the function
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
            offset: new THREE.Vector3(5, 10, 5),
            ambientIntensity: 0.4,
            directionalIntensity: 0.6
        };
        // Variable initialization
        this.animationId = null;
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
            if (this.currentGCodeLayer >= this.gcodeLayers.length -1) {
                this.currentGCodeLayer = 0;
            }
            this.toggleSlicerAnimation(!this.isPlayingAnimation);
            return;
        }
        if (e.code === 'KeyR') {
            this.controls?.reset();
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
        // Get a reference to the container element that will hold our scene
        const sceneHull = document.querySelector('scene-hull');
        if (sceneHull) {
            ['a-container', 'b-container'].forEach(tag => {
                const container = sceneHull.querySelector(tag);
                if (container && container.children.length > 0) {
                    // console.log(`Quick clean for ${container.tagName}`);
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

        this.ambientLight = new THREE.AmbientLight(COLORS['white'], this.lightParams.ambientIntensity);
        this.scene.add(this.ambientLight);
        this.directionalLight = new THREE.DirectionalLight(COLORS['white'], this.lightParams.directionalIntensity);
        this.directionalLight.position.copy(this.lightParams.offset);
        this.scene.add(this.directionalLight);

        this.axesHelper = new THREE.AxesHelper(30);

        // const light = new THREE.SpotLight();
        // light.position.set(50, 50, 50);
        // this.scene.add(light);

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

        this.startAnimation();
    }

    async createRenderer() {
        if (!navigator.gpu) {
            console.log('WebGPU not available, using WebGL');
            return new THREE.WebGLRenderer({
                alpha: true,
                antialias: true
            });
        }
        const renderer = new THREE.WebGPURenderer({
            alpha: true,
            depth: true,
            stencil: false,
            antialias: true,
            premultipliedAlpha: true,
            preserveDrawingBuffer: false,
            powerPreference: "default",
            FailIfMajorPerformanceCaveat: false,
            desynchronized: false
        });
        await renderer.init();
        return renderer;
    }

    startAnimation() {
        // Animation loop
        const animate = () => {
            if (!this.isInitialized) return;
            this.animationId = requestAnimationFrame(animate);
            if (!this.container || this.container.clientHeight === 0) {
                this.destroy();
                return;
            }
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
        let loadedProgress = (xhr.loaded / xhr.total) * 100;
        // console.log(loadedProgress.toFixed(1) + '% loaded');
        if (this.infoMessage) {
            this.infoMessage.innerHTML = loadedProgress.toFixed(1) + '%';
        }
    }

    onError(error) {
        console.warn(error);
        if (this.infoMessage) {
            this.infoMessage.innerHTML = this.labels.model_load_failed + ': ' + error.message;
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
        if (!this.sizeFlag) return;
        this.setControlsGui();
        if (!this.stats) return;
        this.container.appendChild(this.stats.dom);
        if (this.modelFormat == 'GCode') {
            this.updateViewPreset('Top (XY)');
            this.updateGCodeLayers();
        }
    }

    loadModel() {
        console.log('=== STARTING LOAD ===');
        console.log('Model format:', this.modelFormat);
        console.log('Resource mapping count:', this.resourceMapping?.length || 0);
        console.log('Model path:', this.model.url);
        // fetch(this.model.url, { method: 'HEAD' })
        //     .then(res => {
        //         const contentType = res.headers.get('content-type');
        //         const isBinary = contentType.includes('model/gltf-binary') ||
        //                         this.model.url.includes('.glb');
        //         console.log(`isBinary: ${isBinary}`);
        //     });
        switch (this.modelFormat) {
            case 'STL':
                const stlLoader = new STLLoader();
                stlLoader.load(
                    this.model.url,
                    (geometry) => {
                        this.checkAndLog(geometry);
                        this.mesh = new THREE.Mesh(geometry, this.material);
                        this.scene.add(this.mesh);
                        geometry.center();
                        this.onComplete();
                    },
                    (xhr) => this.onProgress(xhr),
                    (error) => this.onError(error)
                );
                break;
            // GLTF and GLB use GLTFLoader
            case 'GLTF':
                // if (this.resourceMapping && this.resourceMapping.length > 0) {
                //     this.resourceMapping.forEach(item => {
                //         console.log(`  ${item.filename}`);
                //     });
                // }
                const gltfResources = this.createGLTFLoaderWithDraco();
                // If no resources, load directly
                if (this.resourceMapping.length === 0) {
                    gltfResources.loader.load(
                        this.model.url,
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
                    return;
                }
                // Load and process GLTF with resources
                this.loadGltf(gltfResources);
                break;
            case 'GLB':
                const glbResources = this.createGLTFLoaderWithDraco();
                glbResources.loader.load(
                    this.model.url,
                    (glb) => {
                        this.mesh = glb.scene;
                        this.store_animations(glb);
                        // Apply material to all meshes
                        this.mesh.traverse((child) => {
                            // this.checkAndLog(glb, child);
                            if (child.isMesh) {
                                child.material = this.material;
                                // child.material.color.setHex(COLORS[i % COLORS.length]);
                                // child.material.color.setHex(Math.random() * 0xffffff);
                            }
                        });
                        this.scene.add(this.mesh);
                        // Center the model
                        const box = new THREE.Box3().setFromObject(this.mesh);
                        const center = box.getCenter(new THREE.Vector3());
                        this.mesh.position.sub(center);
                        this.onComplete();
                        // Clean up the Draco loader
                        glbResources.dispose();
                    },
                    (xhr) => this.onProgress(xhr),
                    (error) => {
                        this.onError(error);
                        glbResources.dispose();
                    }
                );
                break;
            case 'GCode':
                if (this.sizeFlag) {
                    fetch(this.model.url)
                        .then(r => r.text())
                        .then(text => {
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
                    const loader = new GCodeLoader();
                    loader.load(this.model.url,
                        (object) => this.processGCodeModel(object),
                        (xhr) => this.onProgress(xhr),
                        (error) => this.onError(error)
                    );
                }
                break;
            default:
                console.error(`Unsupported format: ${this.modelFormat}`);
                if (this.infoMessage) {
                    this.infoMessage.innerHTML = this.labels.format_not_supported + ': ' + this.format;
                }
                break;
        }
    }

    checkAndLog(modelData, child = null) {
        console.log('Buffers count:', modelData.buffers?.length || 0);
        console.log('Images count:', modelData.images?.length || 0);
        // if (child && child.geometry?.index){
        //     console.log(`Сhild vertex count: ${child.geometry.attributes.position.count}`);
        //     console.log(`Mesh ${child.name || child.uuid}:`);
        //     console.log(`- Draw mode: ${child.material.wireframe ? 'LINES' : 'TRIANGLES'}`);
        //     console.log(`- Attributes:`, Object.keys(child.geometry.attributes));
        // }
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
        lightFolder.add(this.lightParams, 'ambientIntensity', 0, 2, 0.1)
            .name(this.labels.ambient)
            .onChange(v => this.ambientLight.intensity = v);
        lightFolder.add(this.lightParams, 'directionalIntensity', 0, 3, 0.1)
            .name(this.labels.directional)
            .onChange(v => this.directionalLight.intensity = v);
        lightFolder.add(this.lightParams.offset, 'x', -50, 50, 1).name(this.labels.light + ' X');
        lightFolder.add(this.lightParams.offset, 'y', -50, 50, 1).name(this.labels.light + ' Y');
        lightFolder.add(this.lightParams.offset, 'z', -50, 50, 1).name(this.labels.light + ' Z');
        // Info folder
        const infoFolder = this.gui.addFolder(this.labels.model_info_folder);
        infoFolder.close();
        infoFolder.add(this.model, 'filename').name(this.labels.file).listen();
        infoFolder.add(this.model, 'size').name(this.labels.size).listen();
    }

    updateLightToCamera() {
        if (!this.camera || !this.directionalLight) return;
        // Light position = camera position + offset
        this.directionalLight.position.copy(this.camera.position).add(this.lightParams.offset);
        // Light points in the same direction as the camera
        this.directionalLight.target.position.copy(this.camera.position);
        this.directionalLight.target.position.add(this.camera.getWorldDirection(new THREE.Vector3()).multiplyScalar(10));
    }

    updateViewPreset(viewName) {
        console.log(`Control view: ${viewName}`);
        if (!this.mesh) return;
        this.viewModeController = viewName
        const preset = this.viewPresets[viewName];
        const box = new THREE.Box3().setFromObject(this.mesh);
        const center = box.getCenter(new THREE.Vector3());
        const size = box.getSize(new THREE.Vector3());
        const maxSize = Math.max(size.x, size.y, size.z);
        const distance = maxSize * 2;
        const [x, y, z] = preset.pos;
        this.camera.position.set(
            center.x + x * distance,
            center.y + y * distance,
            center.z + z * distance
        );
        this.camera.lookAt(center);
        this.controls.enableRotate = preset.rot;
        this.controls.target.copy(center);
        this.controls.update();
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
        // Create layers
        // this.gcodeLayers = object.children.map((child, i) => ({
            // index: i,
            // object: child,  // Reference to the actual object
            // visible: this.getLayerVisibility(i)  // Determine initial visibility
        // }));
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
        // console.log(`Created ${this.gcodeLayers.length} layers from ${object.children.length} children`);
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
                if (!this.isPlayingAnimation) return;
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
        // const parts = t.split(/(;LAYER:\d+\n)/);
        // return parts.reduce((acc, part, i) => {
            // if (i % 2 === 1) acc.push(parts[i] + (parts[i+1] || ''));
            // return acc;
        // }, []).filter(Boolean);
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
        // Specify the path to Draco files (they should be in the three/ folder)
        dracoLoader.setDecoderPath('../../../../three/draco/');
        dracoLoader.setDecoderConfig({ type: 'js' }); // 'js'/'wasm'
        gltfLoader.setDRACOLoader(dracoLoader);
        return {
            loader: gltfLoader,
            // dracoLoader: dracoLoader,
            // Helper function for cleanup
            dispose: () => dracoLoader.dispose()
        };
    }

    loadGltf(gltfResources) {
        // Load and process GLTF with resources
        fetch(this.model.url)
            .then(response => {
                if (!response.ok) throw new Error(`HTTP ${response.status}`);
                return response.json();
            })
            .then(gltfData => {
                // Create resource map: filename -> full URL
                const resourceMap = new Map();
                this.resourceMapping.forEach(item => {
                    resourceMap.set(item.filename, item.download_url);
                });
                // console.log('=== RESOURCE MAPPING ===');
                // this.resourceMapping.forEach((item, i) => {
                //     console.log(`${i}: ${item.filename} → ${item.download_url.substring(0, 80)}...`);
                // });
                // Update resource URIs to full URLs
                this.updateGltfResourceUris(gltfData, resourceMap);
                // const allGltfUris = [];
                // if (gltfData.buffers) gltfData.buffers.forEach(b => b.uri && allGltfUris.push(b.uri));
                // if (gltfData.images) gltfData.images.forEach(i => i.uri && allGltfUris.push(i.uri));
                // console.log('Looking for these URIs in mapping:');
                // allGltfUris.forEach(uri => {
                //     const found = resourceMap.has(uri);
                //     console.log(`  ${uri} → ${found ? 'FOUND' : 'NOT FOUND'}`);
                // });
                // Create blob for modified GLTF
                const blob = new Blob([JSON.stringify(gltfData)], { type: 'model/gltf+json' });
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

    // Helper functions
    updateGltfResourceUris(gltfData, resourceMap) {
        // Update buffer URIs
        if (gltfData.buffers) {
            gltfData.buffers.forEach((buffer, i) => {
                if (buffer.uri && resourceMap.has(buffer.uri)) {
                    console.log(`Updating buffer ${i}: ${buffer.uri}`);
                    buffer.uri = resourceMap.get(buffer.uri);
                }
            });
        }
        // Update image URIs
        if (gltfData.images) {
            gltfData.images.forEach((image, i) => {
                if (image.uri && resourceMap.has(image.uri)) {
                    // console.log(`Updating image ${i}: ${image.uri}`);
                    image.uri = resourceMap.get(image.uri);
                // } else if (image.uri) {
                    // console.log(`Image ${i} NOT FOUND in map: ${image.uri}`);
                    // console.log('Available keys:', Array.from(resourceMap.keys()));
                }
            });
        }
    }

    processGltfModel(object) {
        this.mesh = object.scene;
        this.originalMaterials.clear();
        this.store_animations(object);
        this.mesh.traverse((child) => {
            this.checkAndLog(object, child);
            if (child.isMesh && child.material) {
                this.hasTextures = true;
                this.originalMaterials.set(child.uuid, child.material.clone());
            }
        });
        this.scene.add(this.mesh);
        // Center the model
        const box = new THREE.Box3().setFromObject(this.mesh);
        const center = box.getCenter(new THREE.Vector3());
        this.mesh.position.sub(center);
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
        // Stopping animation
        if (this.animationId) cancelAnimationFrame(this.animationId);
        // Removing GUI
        this.gui?.destroy();
        // Clearing Three.js objects
        this.controls?.dispose();
        this.renderer?.dispose();
        this.mixer?.stopAllAction();
        // Clearing DOM
        this.container && (this.container.textContent = '');
        // Unsubscribing from events
        document.removeEventListener('keydown', this.handleKeyDown);
        this.resizeObserver?.disconnect();
    }
}