/* eslint-disable */
import * as THREE from 'three'

const toThree = (v) => {
    return { x: v.x, y: v.z, z: -v.y }
}

export const getPositionFromTle = (satrec, date) => {
    var satellite = require('satellite.js')

    const positionVelocity = satellite.propagate(satrec, date)
    const positionEci = positionVelocity.position
    const gmst = satellite.gstime(date)

    const positionEcf = satellite.eciToEcf(positionEci, gmst)
    return toThree(positionEcf)
}