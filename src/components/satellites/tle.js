/* eslint-disable */
import * as satellite from 'satellite'

export const EarthRadius = 6371;

export const parseTleFile = (fileContent, stationOptions) => {
    const result = []
    let current = null

    for (let i = 0; i < lines.length; ++i) {
        const line = lines[i].trim()

        if (line.length === 0) continue

        if (line[0] === '1') {
            current.tle1 = line
        }
        else if (line[0] === '2') {
            current.tle2 = line
        }
        else {
            current = { 
                name: line, 
                ...stationOptions
            };
            result.push(current)
        }
    }

    return result;
}

const toxyz = (v) => {
    return { x: v.x, y: v.z, z: -v.y }
}

const getSolution = (satel, date) => {
    
    if (!satel.satrec) {
        const { tle1, tle2 } = satel
        if (!tle1 || !tle2) return null
        satel.satrec = satellite.twoline2satrec(tle1, tle2)
    }

    return satellite.propagate(satel.satrec, date)
}


// type: 1 ECEF coordinates   2: ECI coordinates
export const getPositionFromTle = (satel, date, type = 1) => {
    if (!satel || !date) return null

    const positionVelocity = getSolution(satel, date)

    const positionEci = positionVelocity.position
    if (type === 2) return toxyz(positionEci)

    const gmst = satellite.gstime(date)

    if (!positionEci) return null

    const positionEcf = satellite.eciToEcf(positionEci, gmst)
    return toxyz(positionEcf)
}