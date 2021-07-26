<template lang="pug">
    div(id='space' ref='canvas')
</template>

<script>
/* eslint-disable */
import * as THREE from 'three'
import OrbitControls from 'three-orbitcontrols';

export default {
    name: 'Space',
    data: function() {
        const width = 0.6
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
            earthspeed: -0.0007,
            cloudspeed: 0.00005,
            starspeed: 0.0001,
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
            requestAnimationFrame(this.animate)
            this.renderer.render(this.scene, this.camera)
            this.earth.rotation.y += this.earthspeed
            this.clouds.rotation.y += this.cloudspeed
            this.stars.rotation.y += this.starspeed
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
