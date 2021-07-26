<template lang="pug">
    div(id='space' ref='canvas')
</template>

<script>
/* eslint-disable */
import * as THREE from 'three'
import * as satellite from 'satellite'
import OrbitControls from 'three-orbitcontrols';
import { addSatellite } from '@/components/satellites/satellite.js'

export default {
    name: 'Space',
    data: function() {
        const NOAA20_TLE =
        `1 43013U 17073A   21207.16182848  .00000002  00000-0  21447-4 0  9993
        2 43013  98.7212 145.2178 0000870  87.0625 273.0650 14.19549652190934`

        //const satrec = satellite.twoline2satrec(
            //NOAA20_TLE.split('\n')[0].trim(),
            //NOAA20_TLE.split('\n')[1].trim()
        //)
        
        const width = 0.6
        const date = new Date();
        const scene = new THREE.Scene()
        const camera = new THREE.PerspectiveCamera(
            75,
            (window.innerWidth * width) / window.innerHeight,
            1,
            1000
        )
        const renderer = new THREE.WebGLRenderer({antialias: false, alpha: true})
        const ambientlight = new THREE.AmbientLight(0x888888)
        const directionallight = new THREE.DirectionalLight(0xfdfcf0, 1)
        const geometry = new THREE.SphereGeometry(5, 50, 50)
        const cloudgeometry = new THREE.SphereGeometry(5.01, 50, 50);
        const stargeometry = new THREE.SphereGeometry(1000, 50, 50);
        const texture = new THREE.TextureLoader().load(require('../assets/image/earth.jpg'))
        const bumptexture = new THREE.TextureLoader().load(require('../assets/image/earthbump.jpg'))
        const spectexture = new THREE.TextureLoader().load(require('../assets/image/earthspec.jpg'))
        
        const material = new THREE.MeshPhongMaterial({
            map: texture,
            bumpMap: bumptexture,
            specularMap: spectexture,
            color: new THREE.Color('grey'),
            specular: 0x333333,
            shininess: 25,
            bumpScale: 0.005
        })

        const cloudmaterial = new THREE.MeshPhongMaterial({
            map: new THREE.ImageUtils.loadTexture(require('../assets/image/clouds.jpg')),
            transparent: true,
            opacity: 0.1
        });

        const starmaterial = new THREE.MeshPhongMaterial({
            map: new THREE.ImageUtils.loadTexture(require('../assets/image/stars.png')),
            side: THREE.BackSide,
            shininess: 0
        });

        const earth = new THREE.Mesh(geometry, material)
        const clouds = new THREE.Mesh(cloudgeometry, cloudmaterial);
        const stars = new THREE.Mesh(stargeometry, starmaterial);

        //const positionAndVelocity = satellite.propagate(satrec, date);
        //const gmst = satellite.gstime(date);
        //const position = satellite.eciToGeodetic(positionAndVelocity.position, gmst);

        const moongeometry = new THREE.SphereGeometry(0.1, 50, 50);
        const moonmaterial = new THREE.MeshPhongMaterial({
            color: new THREE.Color('grey')
        });
        
        const moon = new THREE.Mesh(moongeometry, moonmaterial);

        const geometryiorbit = new THREE.TorusGeometry(5.5, 0.01, 16, 100);
        const materialorbit = new THREE.MeshBasicMaterial( { color: new THREE.Color('white'), side: THREE.DoubleSide } );
        const orbit = new THREE.Mesh(geometryiorbit, materialorbit);

        return {
            scene: scene,
            camera: camera,
            controls: {},
            renderer: renderer,
            ambientlight: ambientlight,
            directionallight: directionallight,
            earth: earth,
            clouds: clouds,
            stars: stars,
            moon: moon,
            orbit: orbit,
            earthspeed: -0.0007,
            cloudspeed: 0.00005,
            starspeed: 0.0001,
            theta: 0,
            width: width
        }
    },
    created: function() {
        this.scene.add(this.camera)
        this.scene.add(this.ambientlight)
        this.scene.add(this.directionallight)
        this.scene.add(this.earth)
        this.scene.add(this.clouds)
        this.scene.add(this.stars)
        this.scene.add(this.moon)
        this.scene.add(this.orbit)
        this.renderer.setSize(window.innerWidth * this.width, window.innerHeight)
        this.directionallight.position.set(20, 10, 20)
        this.camera.position.z = 13
        this.scene.background = new THREE.Color(0x000000)
    },
    mounted: function() {
        this.$refs.canvas.appendChild(this.renderer.domElement)
        this.controls = new OrbitControls(this.camera, this.renderer.domElement)
        this.controls.rotateSpeed = 0.5
        this.animate()
    },
    methods: {
        animate: function() {
            var r = 5.5;
            var dTheta = 2 * Math.PI / 20000;
            
            requestAnimationFrame(this.animate)
            this.renderer.render(this.scene, this.camera)
            this.earth.rotation.y += this.earthspeed
            this.clouds.rotation.y += this.cloudspeed
            this.stars.rotation.y += this.starspeed

            this.theta += dTheta;
            this.moon.position.x = r * Math.cos(this.theta);
            this.moon.position.y = r * Math.sin(this.theta);
            this.controls.update()
        }
    }
}
</script>
<style>
#space {
    position: fixed;
    left: 0;
    top: 0;
    height: 100%;
    background: black;
    overflow: auto;
}
</style>
