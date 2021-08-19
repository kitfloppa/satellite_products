/* eslint-disable */
import * as MATH from 'mathjs'
import * as THREE from 'three'

export let parseTleFile = (satell, file) => {
    const lines = file.split("\n")

    for (let i = 0; i < lines.length; ++i) {
        const line = lines[i].replace('[+]', '').trim()

        if (line.length === 0) continue

        if (line == 'NOAA 20') {
            satell.name = line
            satell.tle1 = lines[i + 1]
            satell.tle2 = lines[i + 2]
        }
    }

    return satell
}

let loadTLEFileSatellite = (satell, url) => {
    url = 'https://api.allorigins.win/raw?url=' + url
    var xmlhttp = new XMLHttpRequest()
    xmlhttp.open("GET", url, false)
    xmlhttp.send()
    parseTleFile(satell, xmlhttp.responseText)
}

const toThree = (v) => {
    return { x: v.x, y: v.z, z: -v.y }
}

export const getPositionFromTle = (satell, url, date) => {
    var satellite = require('satellite.js')

    if ((satell.tle1 == 0) && (satell.tle1 == 0)) {
        loadTLEFileSatellite(satell, url)
    }

    const satrec = satellite.twoline2satrec(satell.tle1, satell.tle2)
    satell.satrec = satrec

    const positionVelocity = satellite.propagate(satrec, date)
    const positionEci = positionVelocity.position
    const gmst = satellite.gstime(date)

    const positionEcf = satellite.eciToEcf(positionEci, gmst)
    satell.pos = toThree(positionEcf)
    return satell.pos
}

export const addorbit = (satell, url) => {
    satell.orbit.revsperday = satell.satrec.no * (1440 / (2.0 * MATH.pi))
    satell.orbit.minutes = (1440 * 2) / satell.orbit.revsperday
    satell.orbit.date = new Date()
    const material = new THREE.LineBasicMaterial({color: 0x999999, opacity: 1.0, transparent: true})
    
    for (var i = 0; i <= satell.orbit.minutes; i += satell.orbit.mininterval) {
        const date = new Date(satell.orbit.date.getTime() + i * 60000)
        const pos = getPositionFromTle(satell, url, date)
        if (!pos) continue
        satell.orbit.points.push(new THREE.Vector3(pos.x, pos.y, pos.z))
    }
    
    const geometry = new THREE.BufferGeometry().setFromPoints(satell.orbit.points)
    satell.orbit.orbitcurve = new THREE.Line(geometry, this.orbitMaterial);
    console.log(satell)
}