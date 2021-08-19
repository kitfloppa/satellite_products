<template lang="pug">
    div(id='space' ref='canvas')
</template>

<script>
/* eslint-disable */
import * as THREE from 'three'
import OrbitControls from 'three-orbitcontrols';
import * as TLE from '@/components/satellites/tle.js'
import * as satell from '@/components/satellites/satell.js'
import { earthRadius } from "satellite.js/lib/constants";

export default {
    name: 'Space',
    data: function() {
        
        const width = 0.6
        const scene = new THREE.Scene()
        const camera = new THREE.PerspectiveCamera(
            75,
            (window.innerWidth * width) / window.innerHeight,
            0.1,
            1e27
        )
        const renderer = new THREE.WebGLRenderer({antialias: false, alpha: true})
        const ambientlight = new THREE.AmbientLight(0x888888)
        const directionallight = new THREE.DirectionalLight(0xfdfcf0, 1)
        const geometry = new THREE.SphereGeometry(earthRadius, 100, 100)
        const cloudgeometry = new THREE.SphereGeometry(earthRadius + 20, 100, 100);
        const stargeometry = new THREE.SphereGeometry(earthRadius * 9, 50, 50);
        const texture = new THREE.TextureLoader().load(require('../assets/image/earth.jpg'))
        const bumptexture = new THREE.TextureLoader().load(require('../assets/image/earthbump.jpg'))
        const spectexture = new THREE.TextureLoader().load(require('../assets/image/earthspec.jpg'))
        const noaageometry = new THREE.SphereGeometry(60, 100, 100)
        
        const material = new THREE.MeshPhongMaterial({
            map: texture,
            bumpMap: bumptexture,
            specularMap: spectexture,
            color: new THREE.Color('grey'),
            specular: 0x333333,
            shininess: 25,
            bumpScale: 0.005,
        })

        const cloudmaterial = new THREE.MeshPhongMaterial({
            map: new THREE.TextureLoader().load(require('../assets/image/clouds.jpg')),
            transparent: true,
            opacity: 0.1,
            polygonOffset: true,
            polygonOffsetFactor: -10,
            polygonOffsetUnits: -10
        });

        const starmaterial = new THREE.MeshPhongMaterial({
            map: new THREE.TextureLoader().load(require('../assets/image/stars.png')),
            side: THREE.BackSide,
            shininess: 0
        });

        const noaamaterial = new THREE.MeshPhongMaterial({
            color: new THREE.Color('yellow')
        });

        const earth = new THREE.Mesh(geometry, material)
        const clouds = new THREE.Mesh(cloudgeometry, cloudmaterial);
        const stars = new THREE.Mesh(stargeometry, starmaterial);
        const noaa = new THREE.Mesh(noaageometry, noaamaterial);

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
            noaa: noaa,
            satelli: new satell.Satellite(),
            earthspeed: -0.00035,
            cloudspeed: 0.00002,
            starspeed: 0.0003,
            width: width
        }
    },
    created: function() {
        var noaapos = TLE.getPositionFromTle(this.satelli, 'https://celestrak.com/NORAD/elements/noaa.txt', new Date())
        TLE.addorbit(this.satelli, 'https://celestrak.com/NORAD/elements/noaa.txt')

        this.scene.add(this.camera) 
        this.scene.add(this.ambientlight)
        this.scene.add(this.directionallight)
        this.scene.add(this.earth)
        this.scene.add(this.clouds)
        this.scene.add(this.stars)
        this.noaa.position.set(noaapos.x, noaapos.y, noaapos.z)
        this.earth.add(this.noaa)
        this.earth.add(this.satelli.orbit.orbitcurve)
        this.renderer.setSize(window.innerWidth * this.width, window.innerHeight)
        this.directionallight.position.set(0, 59333894, -137112541)
        this.camera.position.z = -16000
        this.camera.position.x = 18000;
        this.camera.lookAt(0, 0, 0);
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
            var dTheta = 2 * Math.PI / 20000;
            
            requestAnimationFrame(this.animate)
            var noaapos = TLE.getPositionFromTle(this.satelli, 'https://celestrak.com/NORAD/elements/noaa.txt', new Date())
            this.renderer.render(this.scene, this.camera)
            this.earth.rotation.y += this.earthspeed
            this.clouds.rotation.y += this.cloudspeed
            this.stars.rotation.y += this.starspeed
            this.noaa.position.set(noaapos.x, noaapos.y, noaapos.z)

            this.theta += dTheta;
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
