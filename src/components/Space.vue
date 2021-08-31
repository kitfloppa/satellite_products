<template lang="pug">
    div(id='space' ref='canvas' width='60%')
</template>

<script>
/* eslint-disable */
import * as THREE from 'three'
import * as mmi from '@/assets/photos/three_mmi.js'
import OrbitControls from 'three-orbitcontrols'
import * as TLE from '@/components/satellites/tle.js'
import * as satell from '@/components/satellites/satell.js'
import { earthRadius } from "satellite.js/lib/constants"

export default {
    name: 'Space',
    data: function() {
        const width = 0.6
        const scene = new THREE.Scene()
        
        const camera = new THREE.PerspectiveCamera(
            54,
            window.innerWidth / window.innerHeight,
            1e-6,
            1e27
        )
        
        const renderer = new THREE.WebGLRenderer({antialias: true, logarithmicDepthBuffer: true})
        const ambientlight = new THREE.AmbientLight(0x888888)
        const directionallight = new THREE.DirectionalLight(0xfdfcf0, 1)
        const geometry = new THREE.SphereGeometry(earthRadius, 100, 100)
        const cloudgeometry = new THREE.SphereGeometry(earthRadius + 20, 100, 100);
        const stargeometry = new THREE.SphereGeometry(earthRadius * 9, 50, 50);
        const texture = new THREE.TextureLoader().load(require('../assets/image/earth.jpg'))
       
        
        const material = new THREE.MeshPhongMaterial({
            map: texture,
            color: new THREE.Color('grey'),
            specular: 0x333333,
            shininess: 25,
            flatShading: false,
        })

        const cloudmaterial = new THREE.MeshPhongMaterial({
            map: new THREE.TextureLoader().load(require('../assets/image/clouds.jpg')),
            transparent: true, opacity: 0.1, 
        });

        const starmaterial = new THREE.MeshPhongMaterial({
            map: new THREE.TextureLoader().load(require('../assets/image/stars.png')),
            side: THREE.BackSide,
            shininess: 0
        });

        const earth = new THREE.Mesh(geometry, material)
        const clouds = new THREE.Mesh(cloudgeometry, cloudmaterial)
        const stars = new THREE.Mesh(stargeometry, starmaterial)

        earth.name = 'earth'
        clouds.name = 'clouds'
        stars.name = 'stars'

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
            satelli: new satell.Satellite(),
            earthspeed: -0.0002,
            cloudspeed: 0.00001,
            starspeed: 0.0001,
            width: width,
            mm: 0
        }
    },
    created: function() {
        this.renderer.setSize(window.innerWidth * this.width, window.innerHeight)
        this.camera.aspect = (window.innerWidth * this.width) / window.innerHeight
        this.camera.updateProjectionMatrix()
        this.renderer.render(this.scene, this.camera)

        this.scene.add(this.camera)
        this.scene.add(this.ambientlight)
        this.scene.add(this.directionallight)
        this.scene.add(this.earth)
        this.scene.add(this.clouds)
        this.scene.add(this.stars)
        this.earth.add(this.satelli.mesh) 
        this.earth.add(this.satelli.orbit.orbitcurve)
        this.satelli.photos.forEach(el => {this.earth.add(el.mesh)})
        
        this.mm = new mmi.MouseMeshInteraction(this.earth, this.camera)

        this.satelli.photos.forEach(el => {
            this.mm.addHandler(el.mesh.name, 'click', function() {
                var element_p1 = document.getElementById('photo1')
                var element_p2= document.getElementById('photo2')
                var element_d = document.getElementById('dt')
                element_p1.src = el.color
                element_p2.src = el.color
                element_d.src = el.data
            })
        })
        
        this.directionallight.position.set(0, 59333894, -137112541)
        this.camera.position.z = -15000
        this.camera.position.x = 15000
        this.scene.background = new THREE.Color(0x000000)
    },
    mounted: function() {
        this.$refs.canvas.appendChild(this.renderer.domElement)
        this.controls = new OrbitControls(this.camera, this.renderer.domElement)
        this.controls.enablePan = false
        this.controls.rotateSpeed = 0.5
        this.animate()
    },
    methods: {
        animate: function() {
            requestAnimationFrame(this.animate)
            this.mm.update()
            this.satelli.date = new Date(this.satelli.date.getTime() + 8)
            this.satelli.pos = TLE.getPositionFromTle(this.satelli.satrec, this.satelli.date)
            this.renderer.render(this.scene, this.camera)
            this.earth.rotation.y += this.earthspeed
            this.clouds.rotation.y += this.cloudspeed
            this.stars.rotation.y += this.starspeed
            this.satelli.mesh.position.set(this.satelli.pos.x, this.satelli.pos.y, this.satelli.pos.z)

            this.controls.update()
        },
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
    overflow: hidden;
}
</style>
