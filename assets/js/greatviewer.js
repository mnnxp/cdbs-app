import * as THREE from '../../../../three/three.webgpu.min.js';
import { STLLoader } from '../../../../three/loaders/STLLoader.js';
import { GLTFLoader } from '../../../../three/loaders/GLTFLoader.js';
import { DRACOLoader } from '../../../../three/loaders/DRACOLoader.js';
import { STEPLoader } from '../../../../three/loaders/STEPLoader.js';
import { GCodeLoader } from '../../../../three/loaders/GCodeLoader.js';
import { OrbitControls } from '../../../../three/controls/OrbitControls.js';
import { TransformControls } from '../../../../three/controls/TransformControls.js';
import Stats from '../../../../three/stats.module.js';
import { GUI } from '../../../../three/lil-gui.esm.min.js';
import { fetchWithCache } from '../../../../three/model-cache.js';

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
    bg_dark: 0x212529,  // #212529
    bg_light: 0xf0f2f5, // #f0f2f5
};

const QUALITY_MAP = [
    { threshold: 8000000, level: 'low', ratio: 1 },
    { threshold: 3500000, level: 'medium', ratio: 1.5 },
    { threshold: 0, level: 'high', ratio: 2 }
];

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
        this.svgLoading = '<img src="../../../../icons/mini_loading.svg" />';
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
        this._gcodeMaterials = {
            normal: new THREE.LineBasicMaterial({
                color: COLORS.cyan,
                linewidth: 1,
                transparent: false
            }),
            active: new THREE.LineBasicMaterial({
                color: COLORS.cdbs_blue,
                linewidth: 1,
                transparent: false
            })
        };
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
        this.container = null;
        this.gui = null;
        this.scene = null;
        this.renderer = null;
        this.camera = null;
        this.isOrthographic = false;
        this.controls = null;
        this.transformControls = null;
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
        this._gcodeProgress = 0;
        this._lastGCodeTime = 0;
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
        this.axesHelper = new THREE.AxesHelper(5);
        this.infoMessage = null;
        this.useCustomMaterial = false;
        this.isWireframe = false;
        // GCode
        this.parsedLayers = [];
        this.gcodeLayers = [];
        this.currentGCodeLayer = 0;
        this.displayMode = this.labels.display_up_to_current;
        // Rendering control
        this._renderRequested = false;
        this._animationLoopActive = false;
        // Hotkeys handler
        this.handleKeyDown = this.handleKeyDown.bind(this);
        this.handleDoubleClick = this.handleDoubleClick.bind(this);
        this._isDestroying = false;
    }

    handleKeyDown(e) {
        if (!this.isInitialized || e.target.tagName !== 'CANVAS') {
            return;
        }
        if (e.code === 'Space') {
            e.preventDefault();
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
        if (e.code === 'KeyH') {
            e.preventDefault()
            this.controls?.reset();
            this._requestRender();
            return;
        }
        if (e.code === 'KeyZ') {
            e.preventDefault()
            this.isWireframe = !this.isWireframe;
            if (this.isWireframe) this.useCustomMaterial = true;
            this.updateMaterial();
            return;
        }
        if (e.code === 'KeyT') {
            e.preventDefault();
            if (this.transformControls?.object) {
                this.detachTransformGizmo();
            } else if (this.mesh) {
                this.attachTransformGizmo(this.mesh);
            }
            return;
        }
        if (this.sizeFlag && this.transformControls) {
            const isTransformationKey = ['KeyW', 'KeyE', 'KeyR'].includes(e.code);
            if (isTransformationKey && !this.transformControls.object && this.mesh) {
                this.attachTransformGizmo(this.mesh);
            }
            switch (e.code) {
                case 'KeyW':
                    this.transformControls.setMode('translate');
                    break;
                case 'KeyE':
                    this.transformControls.setMode('rotate');
                    break;
                case 'KeyR':
                    this.transformControls.setMode('scale');
                    break;
            }
            if (isTransformationKey) return;
        }
        const keyMap = {
            // Flat views
            'Digit0': 'isometric', 'Numpad0': 'isometric',
            'Digit1': 'front',     'Numpad1': 'front',
            'Digit3': 'left',      'Numpad3': 'left',
            'Digit7': 'top',       'Numpad7': 'top',
            'Digit9': 'opposite',  'Numpad9': 'opposite',
            // Rotations
            'Digit2': 'rotate_down',   'Numpad2': 'rotate_down',
            'Digit4': 'rotate_left',   'Numpad4': 'rotate_left',
            'Digit6': 'rotate_right',  'Numpad6': 'rotate_right',
            'Digit8': 'rotate_up',     'Numpad8': 'rotate_up',
            // Camera modes
            'Digit5': 'toggle_camera', 'Numpad5': 'toggle_camera',
        };
        const action = keyMap[e.code];
        if (!action) return; // Ignore unmapped keys
        const viewPresets = ['front', 'left', 'top', 'isometric'];
        const rotateActions = ['rotate_up', 'rotate_down', 'rotate_left', 'rotate_right'];
        if (viewPresets.includes(action)) {
            const labelKey = `view_${action}`;
            const translatedPreset = this.labels[labelKey];
            if (translatedPreset) {
                this.updateViewPreset(translatedPreset);
            }
            // Auto-switch camera mode
            const shouldBeOrtho = action !== 'isometric';
            if (this.isOrthographic !== shouldBeOrtho) {
                this.toggleCameraMode(shouldBeOrtho);
            }
        } else if (rotateActions.includes(action)) {
            this.rotateCameraDiscrete(action);
        } else if (action === 'toggle_camera') {
            this.toggleCameraMode();
        } else if (action === 'opposite') {
            this.goToOppositeView();
        }
        e.preventDefault();
    }

    handleDoubleClick(e) {
        if (!this.isInitialized) return;
        this.initPivotSelection(e);
        e.preventDefault();
    }

    centerAndFitCamera() {
        if (!this.mesh) return;
        this.mesh.updateMatrixWorld(true);
        const box = new THREE.Box3().setFromObject(this.mesh);
        if (box.isEmpty()) return;
        const center = box.getCenter(new THREE.Vector3());
        const size = box.getSize(new THREE.Vector3());
        const maxDim = Math.max(size.x, size.y, size.z);
        const aspect = this.container.clientWidth / this.container.clientHeight;
        if (this.isOrthographic) {
            const viewSize = maxDim * 1.3;
            if (!this.camera || this.camera.isPerspectiveCamera) {
                this.camera = new THREE.OrthographicCamera(
                    -viewSize * aspect / 2,
                    viewSize * aspect / 2,
                    viewSize / 2,
                    -viewSize / 2,
                    Math.max(0.1, maxDim / 100),
                    maxDim * 10
                );
                if (this.controls) this.controls.object = this.camera;
            } else {
                this.camera.left = -viewSize * aspect / 2;
                this.camera.right = viewSize * aspect / 2;
                this.camera.top = viewSize / 2;
                this.camera.bottom = -viewSize / 2;
                this.camera.near = Math.max(0.1, maxDim / 100);
                this.camera.far = maxDim * 10;
            }
            this.camera.zoom = 1;
            const orthoDistance = maxDim * 2;
            this.camera.position.set(
                center.x + orthoDistance * 0.5,
                center.y + orthoDistance * 0.5,
                center.z + orthoDistance
            );
            this.camera.updateProjectionMatrix();
        } else {
            if (!this.camera || this.camera.isOrthographicCamera) {
                this.camera = new THREE.PerspectiveCamera(
                    45,
                    aspect,
                    Math.max(0.1, maxDim / 100),
                    maxDim * 10
                );
                if (this.controls) this.controls.object = this.camera;
            } else {
                this.camera.aspect = aspect;
                this.camera.near = Math.max(0.1, maxDim / 100);
                this.camera.far = maxDim * 10;
            }
            const fovInRadians = (this.camera.fov * Math.PI) / 180;
            let distance = maxDim / (2 * Math.tan(fovInRadians / 2));
            distance *= 1.3;
            this.camera.position.set(
                center.x + distance * 0.5,
                center.y + distance * 0.5,
                center.z + distance
            );
            this.camera.updateProjectionMatrix();
        }
        if (this.controls) {
            this.controls.target.copy(center);
            this.camera.lookAt(center);
            this.controls.update();
        }
        this._requestRender();
    }

    toggleCameraMode(isOrthographic = null) {
        const targetMode = (isOrthographic === null) ? !this.isOrthographic : isOrthographic;
        if (this.isOrthographic === targetMode) return;
        const currentPosition = this.camera.position.clone();
        const currentTarget = this.controls.target.clone();
        const aspect = this.container.clientWidth / this.container.clientHeight;
        let newCamera;
        if (targetMode) {
            // Orthographic: calculate frustum size based on current view
            const distance = currentPosition.distanceTo(currentTarget);
            const visibleHeight = Math.tan((45 * Math.PI) / 360) * distance * 2;
            const visibleWidth = visibleHeight * aspect;
            newCamera = new THREE.OrthographicCamera(
                -visibleWidth / 2, visibleWidth / 2,
                visibleHeight / 2, -visibleHeight / 2,
                0.1, 5000
            );
            console.log("Switched to OrthographicCamera");
        } else {
            // Perspective
            newCamera = new THREE.PerspectiveCamera(45, aspect, 0.1, 5000);
            console.log("Switched to PerspectiveCamera");
        }
        newCamera.position.copy(currentPosition);
        newCamera.quaternion.copy(this.camera.quaternion);
        newCamera.up.copy(this.camera.up);
        const oldCamera = this.camera;
        this.camera = newCamera;
        this.controls.object = this.camera;
        this.controls.target.copy(currentTarget);
        this.controls.update();
        if (this.transformControls) {
            this.transformControls.camera = this.camera;
        }
        if (oldCamera.dispose) oldCamera.dispose();
        this.isOrthographic = targetMode;
        this._requestRender();
    }

    goToOppositeView() {
        if (!this.controls || !this.camera) return;
        // Invert the vector from center to camera
        const offset = this.camera.position.clone().sub(this.controls.target);
        offset.negate();
        this.camera.position.copy(this.controls.target).add(offset);
        this.controls.update();
        this._requestRender();
    }

    rotateCameraDiscrete(direction, angleDegrees = 15) {
        if (!this.controls || !this.camera) return;
        const angleRadians = (angleDegrees * Math.PI) / 180;
        const offset = this.camera.position.clone().sub(this.controls.target);
        switch (direction) {
            case 'rotate_left':
                offset.applyAxisAngle(new THREE.Vector3(0, 1, 0), -angleRadians);
                break;
            case 'rotate_right':
                offset.applyAxisAngle(new THREE.Vector3(0, 1, 0), angleRadians);
                break;
            case 'rotate_up': {
                const right = new THREE.Vector3(1, 0, 0).applyQuaternion(this.camera.quaternion);
                offset.applyAxisAngle(right, -angleRadians);
                break;
            }
            case 'rotate_down': {
                const right = new THREE.Vector3(1, 0, 0).applyQuaternion(this.camera.quaternion);
                offset.applyAxisAngle(right, angleRadians);
                break;
            }
        }
        this.camera.position.copy(this.controls.target).add(offset);
        this.controls.update();
        this._requestRender();
    }

    initTransformTools() {
        if (!this.camera || !this.renderer) return;
        this.transformControls = new TransformControls(this.camera, this.renderer.domElement);
        this.scene.add(this.transformControls.getHelper());
        this.transformControls.addEventListener('dragging-changed', (event) => {
            if (this.controls) {
                this.controls.enabled = !event.value;
            }
        });
        this.transformControls.addEventListener('change', () => {
            this._requestRender();
        });
    }

    async starter() {
        if (this.initPromise) return this.initPromise;
        this.startTime = performance.now();
        this.initPromise = this._starterInternal();
        return this.initPromise;
    }

    async _starterInternal() {
        const sceneHull = document.querySelector('scene-hull');
        if (!sceneHull) {
            console.error('Scene Hull element not found');
            return;
        }

        ['a-container', 'b-container'].forEach(tag => {
            const container = sceneHull.querySelector(tag);
            if (container) {
                while (container.firstChild) {
                    container.removeChild(container.firstChild);
                }
            }
        });

        let container_tag = this.sizeFlag ? 'b-container' : 'a-container';
        let backgroundColor = this.sizeFlag ? COLORS.bg_dark : COLORS.bg_light;
        console.log(`Full screen mode: ${this.sizeFlag}`);

        this.container = sceneHull.querySelector(container_tag);
        if (!this.container) {
            console.error(`Container <${container_tag}> not found`);
            return;
        }

        let clientWidth = this.container.clientWidth;
        let clientHeight = this.container.clientHeight;

        this.infoMessage = document.createElement('div');
        this.infoMessage.classList.add('text-center');
        this.container.appendChild(this.infoMessage);

        this.scene = new THREE.Scene();
        this.scene.background = new THREE.Color(backgroundColor);

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

        // Camera
        const fov = 45;
        const near = 0.1;
        const far = 5000;
        const aspect = clientWidth / clientHeight;
        this.camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
        this.camera.position.set(0, 0, 70);
        this.isOrthographic = false;

        // Renderer
        this.renderer = await this.createRenderer();
        this.isInitialized = true;

        this.setQuality('high');

        // Add the automatically created <canvas> element
        this.container.append(this.renderer.domElement);
        // Set the renderer size
        this.renderer.setSize(clientWidth, clientHeight, false);

        this.controls = new OrbitControls(this.camera, this.renderer.domElement);
        this.controls.enableDamping = false;
        this.controls.addEventListener('change', () => {
            this._requestRender();
        });

        if (this.sizeFlag) {
            this.initTransformTools();
        }

        this.renderer.domElement.setAttribute('tabindex', '0');
        this.renderer.domElement.style.outline = 'none';
        this.renderer.domElement.addEventListener('keydown', this.handleKeyDown);
        this.renderer.domElement.addEventListener('dblclick', this.handleDoubleClick);
        setTimeout(() => {
            if (this.renderer && this.renderer.domElement) {
                this.renderer.domElement.focus();
            }
        }, 150);

        if (this.sizeFlag) {
            this.stats = new Stats();
            this.container.appendChild(this.stats.dom);
        }

        this.setupResizeObserver();

        // Loading the model
        await this.loadModel();

        // First render
        if (this.isInitialized && this.renderer && this.container) {
            if (this.controls) {
                this.controls.update();
            }
            this._requestRender();
        }
    }

    setupResizeObserver() {
        this.resizeObserver = new ResizeObserver((entries) => {
            if (!this.isInitialized || !this.renderer || !this.camera) return;
            for (let entry of entries) {
                const width = Math.round(entry.contentBoxSize ? entry.contentBoxSize[0].inlineSize : entry.contentRect.width);
                const height = Math.round(entry.contentBoxSize ? entry.contentBoxSize[0].blockSize : entry.contentRect.height);
                if (width === 0 || height === 0) continue;
                const canvas = this.renderer.domElement;
                if (canvas.clientWidth !== width || canvas.clientHeight !== height) {
                    this.renderer.setSize(width, height, false);
                    const aspect = width / height;
                    if (this.camera.isOrthographicCamera) {
                        const viewSize = (this.camera.top - this.camera.bottom) / this.camera.zoom;
                        this.camera.left = -viewSize * aspect / 2;
                        this.camera.right = viewSize * aspect / 2;
                        this.camera.top = viewSize / 2;
                        this.camera.bottom = -viewSize / 2;
                    } else {
                        this.camera.aspect = aspect;
                    }
                    this.camera.updateProjectionMatrix();
                    this._requestRender();
                }
            }
        });
        this.resizeObserver.observe(this.container);
    }

    async createRenderer() {
        const renderer = new THREE.WebGPURenderer({
            alpha: true,
            depth: true,
            stencil: false,
            antialias: true
        });
        await renderer.init();
        console.log(`CADBase Viewer: Renderer initialized on backend: "${renderer.backend.name}"`);
        return renderer;
    }

    _requestRender() {
        console.log(`CADBase Viewer: _renderRequested: "${this._renderRequested}"`);
        if (this._renderRequested) return;
        this._renderRequested = true;
        requestAnimationFrame(() => {
            this._renderRequested = false;
            if (!this.isInitialized || !this.renderer || !this.container) return;
            this.renderer.render(this.scene, this.camera);
        });
    }

    _startAnimationLoop() {
        if (this._animationLoopActive) return;
        this._animationLoopActive = true;
        this.renderer.setAnimationLoop((time) => {
            if (!this.isInitialized || !this._animationLoopActive) {
                this._stopAnimationLoop();
                return;
            }
            this._renderWithUpdates();
        });
    }

    _stopAnimationLoop() {
        this._animationLoopActive = false;
        this.renderer.setAnimationLoop(null);
        this._requestRender();
    }

    _renderWithUpdates() {
        if (!this.isInitialized || !this.renderer || !this.scene || !this.camera) {
            return;
        }
        if (this.container.clientHeight === 0) {
            this.destroy();
            return;
        }
        if (this.controls) {
            this.controls.update();
        }
        this.updateLightToCamera();
        if (this.mixer) {
            const delta = 0.016;
            this.mixer.update(delta * this.animationSpeed);
        }
        if (this.stats) {
            this.stats.update();
        }
        if (this.sceneRotation) {
            this.scene.rotation.x += 0.005;
            this.scene.rotation.y += 0.01;
        }
        // For GCode
        if (this.isPlayingAnimation && this.sizeFlag && this.gcodeLayers.length > 0) {
            this._updateGCodeAnimation();
        }

        this.renderer.render(this.scene, this.camera);
    }

    _updateGCodeAnimation() {
        if (!this._lastGCodeTime) {
            this._lastGCodeTime = performance.now();
            this._gcodeProgress = this.currentGCodeLayer;
            return;
        }
        const now = performance.now();
        const delta = (now - this._lastGCodeTime) / 1000;
        this._lastGCodeTime = now;
        this._gcodeProgress += delta * this.animationSpeed;
        if (this._gcodeProgress >= this.gcodeLayers.length) {
            this.isPlayingAnimation = false;
            this._gcodeProgress = this.gcodeLayers.length - 1;
            this._stopAnimationLoop();
        }
        const newLayer = Math.floor(this._gcodeProgress);
        if (newLayer !== this.currentGCodeLayer) {
            this.currentGCodeLayer = newLayer;
            this.updateGCodeLayers();
        }
    }

    _hasActiveAnimations() {
        return this.isPlayingAnimation ||
               this.sceneRotation ||
               (this.mixer && this.mixer._actions && this.mixer._actions.length > 0);
    }

    attachTransformGizmo(object) {
        if (!this.transformControls) return;
        this.transformControls.attach(object);
        this._requestRender();
    }

    detachTransformGizmo() {
        if (!this.transformControls) return;
        this.transformControls.detach();
        this._requestRender();
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
        if (this.infoMessage && this.infoMessage.parentNode) {
            this.infoMessage.parentNode.removeChild(this.infoMessage);
            this.infoMessage = null;
        }
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
        this._requestRender();
    }

    async loadModel() {
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
                        this.centerAndFitCamera();
                        this.optimizeQualityModel();
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
            case 'STEP':
                const stepLoader = new STEPLoader();
                try {
                    const buffer = await fetchWithCache(this.model.url, (percent) => {
                        this.infoMessage.innerHTML = percent.toFixed(1) + '%';
                    });
                    this.infoMessage.innerHTML = this.svgLoading;
                    const group = await stepLoader.loadFromBuffer(buffer);
                    this.mesh = group;
                    this.scene.add(this.mesh);
                    this.centerAndFitCamera();
                    this.optimizeQualityModel();
                    this.onComplete();
                } catch (error) {
                    console.error('[STEP] Error:', error);
                    this.onError(error);
                } finally {
                    stepLoader.terminate();
                }
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
        // Display controls folder
        const displayFolder = this.gui.addFolder(this.labels.display);
        displayFolder.open();
        displayFolder.add(controlParams, 'showAxesHelper')
            .name(this.labels.axes)
            .onChange((value) => {
                if (value) {
                    this.scene.add(this.axesHelper);
                } else {
                    this.scene.remove(this.axesHelper);
                }
                this.showAxesHelper = value;
                this._requestRender();
            });
        displayFolder.add(controlParams, 'sceneRotation')
            .name(this.labels.rotation)
            .onChange((value) => {
                this.sceneRotation = value;
                if (value) {
                    this._startAnimationLoop();
                } else {
                    this._stopAnimationLoop();
                }
            });
        displayFolder.add(controlParams, 'customScale', 0.01, 2)
            .name(this.labels.model_scale)
            .onChange((value) => {
                this.scene.scale.set(value, value, value);
                this._requestRender();
            });
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
                    this._requestRender();
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
                            this._startAnimationLoop();
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
            .onChange((value) => {
                this.scene.background.set(value);
                this._requestRender();
            });
        materialFolder.add(this.material, 'metalness', 0, 1, 0.05)
            .name(this.labels.metalness)
            .onChange(() => this._requestRender());
        materialFolder.add(this.material, 'roughness', 0, 1, 0.05)
            .name(this.labels.roughness)
            .onChange(() => this._requestRender());
        if (this.material.envMap) {
            materialFolder.add(this.material, 'envMapIntensity', 0, 2, 0.1)
                .name(this.labels.env_intensity)
                .onChange(() => this._requestRender());
        }
        // MeshPhysicalMaterial
        if (this.material.clearcoat !== undefined) {
            materialFolder.add(this.material, 'clearcoat', 0, 1, 0.05)
                .name(this.labels.clearcoat)
                .onChange(() => this._requestRender());
            materialFolder.add(this.material, 'clearcoatRoughness', 0, 1, 0.05)
                .name(this.labels.clearcoat_rough)
                .onChange(() => this._requestRender());
        }
        const lightFolder = this.gui.addFolder(this.labels.lighting_folder);
        lightFolder.close();
        lightFolder.add(this.lightParams, 'ambientIntensity', 0, 1, 0.01)
            .name(this.labels.ambient || 'Ambient')
            .onChange(v => {
                this.ambientLight.intensity = v;
                this._requestRender();
            });
        lightFolder.add(this.lightParams, 'hemisphereIntensity', 0, 1, 0.01)
            .name(this.labels.hemisphere || 'Hemisphere')
            .onChange(v => {
                this.hemisphereLight.intensity = v;
                this._requestRender();
            });
        lightFolder.add(this.lightParams, 'keyIntensity', 0, 2, 0.01)
            .name(this.labels.key_light || 'Key Light')
            .onChange(v => {
                this.keyLight.intensity = v;
                this._requestRender();
            });
        lightFolder.add(this.lightParams, 'fillIntensity', 0, 1.5, 0.01)
            .name(this.labels.fill_light || 'Fill Light')
            .onChange(v => {
                this.fillLight.intensity = v;
                this._requestRender();
            });
        lightFolder.add(this.lightParams, 'rimIntensity', 0, 1, 0.01)
            .name(this.labels.rim_light || 'Rim Light')
            .onChange(v => {
                this.rimLight.intensity = v;
                this._requestRender();
            });
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
        this._requestRender();
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
        this.mesh = object;
        this.scene.add(this.mesh);
        this.mesh.updateMatrixWorld(true);
        if (this.mesh.children && this.mesh.children.length > 0) {
            this.gcodeLayers = this.mesh.children.map((child, i) => {
                const layerInfo = this.parsedLayers?.[i];
                return {
                    index: i,
                    object: child,
                    visible: this.getLayerVisibility(i),
                    number: layerInfo?.number || i,
                    z: layerInfo?.z || i * 0.2
                };
            });
        } else {
            this.gcodeLayers = [{
                index: 0,
                object: this.mesh,
                visible: true,
                number: 0,
                z: 0
            }];
        }
        this.centerAndFitCamera();
        this.optimizeQualityModel();
        this.onComplete();
    }

    updateGCodeLayers() {
        if (!this.gcodeLayers || !this.mesh) return;
        this.gcodeLayers.forEach((layer, i) => {
            if (!layer.object) return;
            layer.object.visible = this.getLayerVisibility(i);
            if (layer.object.visible && layer.object.children[0]?.isLineSegments) {
                if (i === this.currentGCodeLayer) {
                    layer.object.children[0].material = this._gcodeMaterials.active;
                } else {
                    layer.object.children[0].material = this._gcodeMaterials.normal;
                }
            }
        });
        this._requestRender();
    }

    toggleSlicerAnimation(play) {
        this.isPlayingAnimation = play;
        if (play) {
            this._lastGCodeTime = performance.now();
            this._gcodeProgress = this.currentGCodeLayer;
            this._startAnimationLoop();
        } else {
            this._stopAnimationLoop();
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
        this._requestRender();
    }

    setMaterialColor(color) {
        if (this.modelFormat == 'GCode') {
            this._gcodeMaterials.normal.color.set(color);
            this._gcodeMaterials.active.color.set(color);
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
        this._requestRender();
    }

    getLayerVisibility(index) {
        if (this.displayMode === this.labels.display_all) return true;
        if (this.displayMode === this.labels.display_current_only) return index === this.currentGCodeLayer;
        return index <= this.currentGCodeLayer;
    }
    parseGCodeLayers(t) {
        if (!t) return [];
        console.time('[GCode Parser] Execution Time');
        const layers = [];
        let currentLayer = null;
        let currentLayerLines = [];
        let currentZ = null;
        const lineRegex = /[^\r\n]+/g;
        let match;
        while ((match = lineRegex.exec(t)) !== null) {
            const line = match[0];
            if (line.startsWith(';LAYER:')) {
                if (currentLayer !== null) {
                    layers.push({
                        text: currentLayerLines.join('\n') + '\n',
                        number: currentLayer,
                        z: currentZ !== null ? currentZ : currentLayer * 0.2
                    });
                }
                currentLayer = parseInt(line.slice(7), 10);
                currentLayerLines = [line];
                currentZ = null;
            } else if (currentLayer !== null) {
                currentLayerLines.push(line);
                if (currentZ === null && (line.startsWith('G0') || line.startsWith('G1'))) {
                    const zValue = this.extractZFromLayer(line);
                    if (zValue !== null) {
                        currentZ = zValue;
                    }
                }
            }
        }

        if (currentLayer !== null) {
            layers.push({
                text: currentLayerLines.join('\n') + '\n',
                number: currentLayer,
                z: currentZ !== null ? currentZ : currentLayer * 0.2
            });
        }

        console.timeEnd('[GCode Parser] Execution Time');
        console.log(`[GCode Parser] Successfully parsed ${layers.length} layers.`);
        return layers;
    }

    extractZFromLayer(layerText) {
        const zMatch = layerText.match(/[Zz](-?\d+(\.\d+)?)/);
        return zMatch ? parseFloat(zMatch[1]) : null;
    }

    createGLTFLoaderWithDraco() {
        const gltfLoader = new GLTFLoader();
        const dracoLoader = new DRACOLoader();
        dracoLoader.setDecoderPath('../../../../three/draco/');
        dracoLoader.setDecoderConfig({ type: 'wasm' });
        gltfLoader.setDRACOLoader(dracoLoader);
        return {
            loader: gltfLoader,
            dispose: () => dracoLoader.dispose()
        };
    }

    loadGltf(gltfResources) {
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
        if (!this.mesh || !this.renderer) return;
        let totalTriangles = 0;
        this.mesh.traverse((child) => {
            if (child.isMesh && child.geometry) {
                const geom = child.geometry;
                totalTriangles += geom.index ? geom.index.count / 3 : geom.attributes.position.count / 3;
            }
        });
        console.log(`[Optimizer] Model has ${totalTriangles.toLocaleString()} triangles`);
        const matched = QUALITY_MAP.find(q => totalTriangles > q.threshold) || QUALITY_MAP[QUALITY_MAP.length - 1];
        this.setQuality(matched.level);
        this._requestRender();
    }

    setQuality(level) {
        if (!this.renderer) return;
        this.quality = level;
        const matched = QUALITY_MAP.find(q => q.level === level);
        const targetRatio = matched ? matched.ratio : 2;
        this.renderer.setPixelRatio(Math.min(window.devicePixelRatio, targetRatio));
        this._requestRender();
    }

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
        this.centerAndFitCamera();
        this.onComplete();
    }

    toggleAnimation(play) {
        this.isPlayingAnimation = play;
        if (play) {
            this.animationActions.forEach(action => {
                action.reset();
                action.play();
            });
            this._startAnimationLoop();
        } else {
            this.stopAllAnimations();
            this._stopAnimationLoop();
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
            this._startAnimationLoop();
            return true;
        }
        return false;
    }

    initPivotSelection(e) {
        if (this.transformControls?.object) return;
        const rect = this.renderer.domElement.getBoundingClientRect();
        const mouse = new THREE.Vector2(
            ((e.clientX - rect.left) / rect.width) * 2 - 1,
            -((e.clientY - rect.top) / rect.height) * 2 + 1
        );
        const raycaster = new THREE.Raycaster();
        raycaster.setFromCamera(mouse, this.camera);
        // Find intersection with the model
        const intersects = raycaster.intersectObject(this.mesh, true);
        if (intersects.length > 0) {
            const hitPoint = intersects[0].point;
            // Smoothly update orbit control target
            this.controls.target.copy(hitPoint);
            this.controls.update();
            this._requestRender();
            console.log(`Pivot set to: ${hitPoint.x.toFixed(2)}, ${hitPoint.y.toFixed(2)}, ${hitPoint.z.toFixed(2)}`);
        }
    }

    destroy() {
        if (this._isDestroying) return;
        this._isDestroying = true;
        console.log('Destroying GreatViewer. Status was:', this.isInitialized);

        // Stopping the cycles
        this._stopAnimationLoop();
        this._renderRequested = false;

        this.detachTransformGizmo();
        if (this.transformControls) {
            this.transformControls.dispose();
            this.scene?.remove(this.transformControls);
            this.transformControls = null;
        }
        this.gui?.destroy();
        if (this.mesh && this.scene) {
            this.scene.remove(this.mesh);
            this.mesh.traverse((child) => {
                if (!child.isMesh) return;
                if (child.geometry) {
                    child.geometry.dispose();
                }
                if (child.material) {
                    const materials = Array.isArray(child.material) ? child.material : [child.material];
                    materials.forEach((mat) => {
                        for (const key in mat) {
                            if (mat[key] && typeof mat[key].dispose === 'function') {
                                mat[key].dispose();
                            }
                        }
                        mat.dispose();
                    });
                }
            });
            this.mesh = null;
        }
        this.controls?.dispose();
        if (this.renderer) {
            this.renderer.setAnimationLoop(null);
            if (this.renderer.domElement) {
                const canvas = this.renderer.domElement;
                canvas.removeEventListener('keydown', this.handleKeyDown);
                canvas.removeEventListener('dblclick', this.handleDoubleClick);
            }
            if (typeof this.renderer.forceContextLoss === 'function') {
                this.renderer.forceContextLoss();
            } else if (this.renderer.backend && typeof this.renderer.backend.loseContext === 'function') {
                this.renderer.backend.loseContext();
            }
            this.renderer.dispose();
            if (this.renderer.domElement && this.renderer.domElement.parentNode) {
                this.renderer.domElement.parentNode.removeChild(this.renderer.domElement);
            }
            this.renderer = null;
        }
        this.mixer?.stopAllAction();
        this.mixer = null;
        this.originalMaterials.forEach((mat) => {
            if (typeof mat.dispose === 'function') mat.dispose();
        });
        this.originalMaterials.clear();
        if (this.resizeObserver) {
            this.resizeObserver.disconnect();
        }
        if (this.container) {
            while (this.container.firstChild) {
                this.container.removeChild(this.container.firstChild);
            }
            this.container = null;
        }
        this.scene = null;
        this.camera = null;
        this.isInitialized = false;
    }
}