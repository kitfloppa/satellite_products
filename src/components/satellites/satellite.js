/* eslint-disable */
import * as THREE from 'three'
import { parseTleFile as parseTleFile, getPositionFromTle } from '@/components/satellites/tle.js'

const ixpdotp = 1440 / (2.0 * 3.141592654)

/*addOrbit = (satel) => {
    if (satel.orbitMinutes > 0) return

    const revsPerDay = satel.satrec.no * ixpdotp
    const intervalMinutes = 1
    const minutes = satel.orbitMinutes || 1440 / revsPerDay
    const initialDate = new Date()

    if (!this.orbitMaterial) {
        this.orbitMaterial = new THREE.LineBasicMaterial({color: 0x999999, opacity: 1.0, transparent: true })
    }

    var points = []
    
    for (var i = 0; i <= minutes; i += intervalMinutes) {
        const date = new Date(initialDate.getTime() + i * 60000)

        const pos = getPositionFromTle(station, date)
        if (!pos) continue

        points.push(new THREE.Vector3(pos.x, pos.y, pos.z))
    }

    const geometry = new THREE.BufferGeometry().setFromPoints(points)
    var orbitCurve = new THREE.Line(geometry, this.orbitMaterial)
    satel.orbit = orbitCurve

    return orbitCurve
}*/

export const addSatellite = (earth) => {
    const satellitegeomtry = new THREE.SphereGeometry(5.3, 20, 20)
    const satellitematerial = new THREE.MeshPhongMaterial({
        color: new THREE.Color('red')
    })

    const sat = new THREE.Mesh(satellitegeomtry, satellitematerial);
    //const pos = this.getSatellitePositionFromTle(satel)
    //if (!pos) return
    
    //sat.position.set(pos.x, pos.y, pos.z)
    //satel.mesh = sat

    //const orbit = this.addOrbit(satel);
    earth.add(sat)
}