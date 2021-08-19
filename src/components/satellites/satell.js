/* eslint-disable */

export class Orbit {
    revsperday = 0
    mininterval = 1
    minutes = 0
    date = 0
    points = []
    position = 0
    orbitcurve = 0

    constructor() {}
}

export class Satellite {
    orbit = new Orbit()
    name = 'none'
    tle1 = 0
    tle2 = 0
    pos = 0
    
    constructor() {}
}