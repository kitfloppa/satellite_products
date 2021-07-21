<template lang="pug">
    div(ref='canvas')
</template>

<script>
/* eslint-disable */
// import { Clock, PerspectiveCamera, Scene, WebGLRenderer } from 'three'
import * as THREE from 'three'
import OrbitControls from 'three-orbitcontrols';
import {
    BloomEffect,
    EffectComposer,
    GlitchPass,
    EffectPass,
    RenderPass
} from 'postprocessing'

export default {
    name: 'Space',
    data: function() {
        const scene = new THREE.Scene()
        // const composer = new THREE.EffectComposer(new WebGLRenderer())
        // const effectPass = new THREE.EffectPass(camera, new BloomEffect())
        const camera = new THREE.PerspectiveCamera(
            75,
            window.innerWidth / window.innerHeight,
            0.1,
            1000
        )
        const renderer = new THREE.WebGLRenderer({ antialias: true })
        const light = new THREE.DirectionalLight('hsl(0, 100%, 100%)')
        const geometry = new THREE.BoxGeometry(1, 1, 1)
        const material = new THREE.MeshStandardMaterial({
            side: THREE.FrontSide,
            color: 'hsl(0, 100%, 50%)',
            wireframe: false
        })
        const cube = new THREE.Mesh(geometry, material)
        //const axes = new THREE.AxesHelper(5)

        return {
            scene: scene,
            camera: camera,
            controls: {},
            renderer: renderer,
            light: light,
            cube: cube,
            //axes: axes,
            speed: 0.01
        }
    },
    created: function() {
        this.scene.add(this.camera)
        this.scene.add(this.light)
        this.scene.add(this.cube)
        //this.scene.add(this.axes)
        this.renderer.setSize(window.innerWidth, window.innerHeight)
        this.light.position.set(0, 0, 10)
        this.camera.position.z = 5
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
            this.cube.rotation.y += this.speed
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
