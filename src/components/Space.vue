<template lang="pug">
    div(ref='canvas')
</template>

<script>
/* eslint-disable */
// import { Clock, PerspectiveCamera, Scene, WebGLRenderer } from 'three'
import * as THREE from 'three'
import OrbitControls from 'three-orbitcontrols';

export default {
    name: 'Space',
    data: function() {
        const scene = new THREE.Scene()
        const camera = new THREE.PerspectiveCamera(
            75,
            window.innerWidth / window.innerHeight,
            0.1,
            1000
        )
        const renderer = new THREE.WebGLRenderer({ antialias: true })
        const light = new THREE.DirectionalLight('hsl(0, 100%, 100%)')
        const geometry = new THREE.SphereGeometry(5, 50, 50)
        const texture = new THREE.TextureLoader().load(require('../earth.jpg'))
        console.log(texture)
        const material = new THREE.MeshBasicMaterial({map: texture})
        const earth = new THREE.Mesh(geometry, material)

        return {
            scene: scene,
            camera: camera,
            controls: {},
            renderer: renderer,
            light: light,
            earth: earth,
            speed: 0.01
        }
    },
    created: function() {
        this.scene.add(this.camera)
        this.scene.add(this.light)
        this.scene.add(this.earth)
        this.renderer.setSize(window.innerWidth, window.innerHeight)
        this.light.position.set(5, 0, 60)
        this.camera.position.z = 50
        this.scene.background = new THREE.Color('hsl(0, 100%, 100%)')
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
            this.earth.rotation.y += this.speed
            this.controls.update()
        }
    },
    computed: {
        rotate: function() {
            if (this.speed === '') {
                return 0
            } else {
                return this.speed
            }
        }
    }
}
</script>
<style>
canvas {
    width: 100px;
    height: 100px;
}
</style>
