/* eslint-disable */

import * as MATH from 'mathjs'  
import * as THREE from 'three'
import * as TLE from '@/components/satellites/tle.js'

export class Orbit {
    constructor(satrec, ndate) {
        const minatday = 1440
        const rotation = (minatday / (2.0 * MATH.pi))
        const material = new THREE.LineBasicMaterial({color: 0x999999, opacity: 1.0})
        
        this.revsperday = satrec.no * rotation
        this.mininterval = 1
        this.minutes = (minatday * 2.0) / this.revsperday
        this.date = ndate
        this.points = []
        
        for (var i = 0; i <= this.minutes; i += this.mininterval) {
            var date = new Date(this.date.getTime() + i * 60000)
            var pos = TLE.getPositionFromTle(satrec, date)
            if (!pos) continue
            this.points.push(new THREE.Vector3(pos.x, pos.y, pos.z))
        }

        const geometry = new THREE.BufferGeometry().setFromPoints(this.points)
        this.orbitcurve = new THREE.Line(geometry, material)
    }
}

export class Photo {
    constructor(satrec, ndate, ncolor, ndata, name, earth, namesat) {
        var radius = 80, widthSegments = 100, heightSegments = 100
        var geometry = new THREE.SphereGeometry(radius, widthSegments, heightSegments)
        var material = new THREE.MeshPhongMaterial({color: new THREE.Color('green')});
        
        this.date = new Date(ndate)
        this.color = ncolor
        this.data = ndata
        this.pos = TLE.getPositionFromTle(satrec, this.date)

        this.orbit = new Orbit(satrec, this.date)
        this.earth = earth
        this.namesat = namesat
        
        this.mesh = new THREE.Mesh(geometry, material)
        this.mesh.position.set(this.pos.x, this.pos.y, this.pos.z)
        this.mesh.name = name + '-photo'
    }
}

export class Satellite {
    constructor(earth) {
        var satellite = require('satellite.js')
        var photomanager = require('@/assets/photos/photo.json')
        var radius = 60, widthSegments = 100, heightSegments = 100
        var geometry = new THREE.SphereGeometry(radius, widthSegments, heightSegments)
        var material = new THREE.MeshPhongMaterial({color: new THREE.Color('yellow')})
        
        this.photos = []
        this.tle1 = '1 43013U 17073A   21242.12214069  .00000006  00000-0  23507-4 0  9992'
        this.tle2 = '2 43013  98.7223 179.6531 0001270  97.6122 262.5199 14.19552269195893'
        
        this.satrec = satellite.twoline2satrec(this.tle1, this.tle2)
        this.date = new Date()
        this.pos = TLE.getPositionFromTle(this.satrec, this.date)
        this.orbit = new Orbit(this.satrec, this.date)
        
        this.mesh = new THREE.Mesh(geometry, material)
        this.mesh.position.set(this.pos.x, this.pos.y, this.pos.z)
        this.mesh.name = 'satellite'

        for (var i = 0; i < photomanager.photos.length; ++i) {
            this.photos.push(new Photo(this.satrec, photomanager.photos[i].time, 
                photomanager.photos[i].color, photomanager.photos[i].data, i, earth, 
                photomanager.photos[i].name))
        }
    }
}