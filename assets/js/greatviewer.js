import * as THREE from 'three'
import { STLLoader } from 'three/addons/loaders/STLLoader';
import { OrbitControls } from 'three/addons/controls/OrbitControls.js';
import Stats from 'three/addons/libs/stats.module';
import { GUI } from 'lil-gui';

// TODO:
// +Стили и gui для настройки отображения модели
// +Загрузка текстуры
// Загрузка разных файлов (stl/obj)
// Отображение прогресса загрузки "loading..."
// Манипуляции с Numpad 
// Манипуляции со стрелок 
// Манипуляции со w,a,s,d

export class GreatViewer {
    constructor(model_path, size_flag) {
        this.modelPath = model_path;
        this.sizeFlag = size_flag;
    }
  
    starter() {
        const nxImg = `data:image/gif;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAAyUlEQVR42u3RMREAMAgAsVKb3DHi
XwIyYMhL+ERXPu31LQAAQAAACAAAAQAgAAAEAIAAABAAAAIAQAAACAAAAQAgAAAEAIAAABAAAAIA
QAAACAAAAQAgAAAEAIAAABAAAAIAQAAACAAAAQAgAAAEAIAAABAAAAAEAIAAABAAAAIAQAAACAAA
AQAgAAAEAIAAABAAAAIAQAAACAAAAQAgAAAEAIAAABAAAAIAQAAACAAAAQAgAAAEAIAAABAAAAIA
QAAACAAAAbjQAOAVAh/Yww3UAAAAAElFTkSuQmCC`;
        const nyImg = `data:image/gif;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAAx0lEQVR42u3RMQEAMAjAsDGf8ODf
AzLgSCU0UdlPe30LAAAQAAACAEAAAAgAAAEAIAAABACAAAAQAAACAEAAAAgAAAEAIAAABACAAAAQ
AAACAEAAAAgAAAEAIAAABACAAAAQAAACAEAAAAgAAAEAIAAABAAAAAEAIAAABACAAAAQAAACAEAA
AAgAAAEAIAAABACAAAAQAAACAEAAAAgAAAEAIAAABACAAAAQAAACAEAAAAgAAAEAIAAABACAAAAQ
AAACAEAALjQfhwIey0nZ0AAAAABJRU5ErkJggg==`;
        const nzImg = `data:image/gif;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAAxUlEQVR42u3RMQEAAAjDMMDY/LtC
BhyphKaTlO4aCwAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAA
AgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAAAABACAAAAQAgAAAEAAAAgBAAAAI
AAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAA
AgBAAD60K4cBvajARSMAAAAASUVORK5CYII=`;
        const pxImg = `data:image/gif;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAYAAADDPmHLAAAA8klEQVR42u3SQQ0AMAgAsTH/6rYH
csAGCa2Ey0X+V4e1rgQGwAAYAANgAAyAATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAY
AANgAAyAATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADYAAM
gAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADYAADSGAADIABMAAGwAAYAANgAAyA
ATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADMEMDMPMEe7fN
plYAAAAASUVORK5CYII=`;
        const pyImg = `data:image/gif;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAIAAABMXPacAAAAxklEQVR42u3RMQEAAAQAQfQPZBRN
DIb7CH/ZPaG7ygIAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQA
gAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABAABAAAAIAAABACAAAAQAgAAAEAAA
AgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQAgAAAEAAAAgBAAAAIAAABACAAAAQA
gAAAEIAPLTJyAzCjdRU4AAAAAElFTkSuQmCC`;
        const pzImg = `data:image/gif;base64,iVBORw0KGgoAAAANSUhEUgAAAIAAAACACAYAAADDPmHLAAAA8ElEQVR42u3SgQkAMAjAsLnzN/Bl
fUMwOaE08v06rHUlMAAGwAAYAANgAAyAATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAY
AANgAAyAATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADYAAM
gAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGAADGEACA2AADIABMAAGwAAYAANgAAyA
ATAABsAAGAADYAAMgAEwAAbAABgAA2AADIABMAAGwAAYAANgAAyAATAABsAAGIAZGrldA6KZLJDP
AAAAAElFTkSuQmCC`;

        // Get a reference to the container element that will hold our scene
        const sceneHull = document.querySelector('scene-hull');
        var container_tag = 'a-container';
        var backgroundColor = '#fff';
        console.log(`this.sizeFlag: ${this.sizeFlag}`);
        if (this.sizeFlag) {
            container_tag = 'b-container';
            backgroundColor = '#a1b5d2'
        }
        console.log(`container_tag: ${container_tag}`);
        const container = sceneHull.querySelector(container_tag);

        var clientWidth = container.clientWidth;
        var clientHeight = container.clientHeight;

        // create a Scene
        const scene = new THREE.Scene();
        scene.add(new THREE.AxesHelper(5));

        const light = new THREE.SpotLight();
        light.position.set(50, 50, 50);
        scene.add(light);

        // Set the background color
        scene.background = new THREE.Color(backgroundColor);

        // Create a camera
        const fov = 45; // AKA Field of View
        const near = 0.1; // the near clipping plane
        const far = 5000; // the far clipping plane
        var camera = new THREE.PerspectiveCamera();
        function setSceneSize() {
            var aspect = clientWidth / clientHeight;
            camera.updateProjectionMatrix();
            camera = new THREE.PerspectiveCamera(fov, aspect, near, far);
            camera.position.set(0, 0, 70);
        }
        setSceneSize();

        // create the renderer
        const renderer = new THREE.WebGLRenderer({
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
        // next, set the renderer to the same size as our container element
        renderer.setSize(clientWidth, clientHeight);
        // finally, set the pixel ratio so that our scene will look good on HiDPI displays
        renderer.setPixelRatio(window.devicePixelRatio);
        // add the automatically created <canvas> element to the page
        container.append(renderer.domElement);
        
        const controls = new OrbitControls(camera, renderer.domElement);
        controls.enableDamping = true;

        const evnTexture = new THREE.CubeTextureLoader().load([
            pxImg, //right
            nxImg, //left
            pyImg, //top
            nyImg, //bottom
            pzImg, //back
            nzImg, //front
        ]);
        evnTexture.mapping = THREE.CubeReflectionMapping;

        const material = new THREE.MeshPhysicalMaterial({
            color: 0x1872f0,
            envMap: evnTexture,
            metalness: 0.25,
            roughness: 0.1,
            opacity: 1.0,
            transparent: true,
            transmission: 0.99,
            clearcoat: 1.0,
            clearcoatRoughness: 0.25,
        });

        const loader = new STLLoader()
        loader.load(
            this.modelPath,
            function (geometry) {
                const mesh = new THREE.Mesh(geometry, material);
                scene.add(mesh);
            },
            (xhr) => {
                console.log((xhr.loaded / xhr.total) * 100 + '% loaded');
            },
            (error) => {
                console.warn(error);
            }
        )

        function setControlsGui() {
            const gui = new GUI({ autoPlace: false, title: '', container: container });
                gui.domElement.id = 'three-gui'
                gui.add(material, 'wireframe').name('Каркас');
                const materialParams = { materialMeshColor: material.color.getHex() };
                gui.addColor(materialParams, 'materialMeshColor')
                    .name('Цвет модели')
                    .onChange((value) => material.color.set(value));
                const sceneParams = {
                    backgroundColor: scene.background.getHex(),
                    customScale: 1,
                };
                gui.addColor(sceneParams, 'backgroundColor')
                    .name('Цвет фона')
                    .onChange((value) => scene.background.set(value));

                gui.add(sceneParams, 'customScale', 0.01, 2)
                  .name('Масштаб модели')
                  .onChange((value) => {
                    scene.scale.setX(value);
                    scene.scale.setY(value);
                    scene.scale.setZ(value);
                    // console.log(`New... setX: ${scene.scale.x}, setY: ${scene.scale.y}, setZ: ${scene.scale.z}`);
                  });
        }

        const stats = new Stats();
        if (this.sizeFlag) {
            // show only on the full screen
            container.appendChild(stats.dom); // show stats
            setControlsGui(); // show controls GUI
        }

        // animation
        function animate() {
            if (container.clientHeight == 0) {
                console.log(`New size ${container.clientWidth}/${container.clientHeight}. Closing...`);
                container.textContent = ''; // removing the canvas and other children
                return
            }
            requestAnimationFrame(animate);
            if (container.clientWidth != clientWidth) {
                console.log(`Update size! New size ${container.clientWidth}/${container.clientHeight}`);
                clientWidth = container.clientWidth;
                clientHeight = container.clientHeight;
                renderer.setSize(clientWidth, clientHeight);
                setSceneSize()
            }

            // if (rotation) {
            //     torus.rotation.x += 0.005
            //     torus.rotation.y += 0.01
            // }

            renderer.render(scene, camera);
            stats.update();
        }
        // rendering the scene
        container.append(renderer.domElement);
        stats.begin();
        renderer.render(scene, camera);
        stats.end();
        animate();
    }
  }
