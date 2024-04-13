import { HOST } from './constants.api';

const SATELLITES_GET = `${HOST}/satellite/all`;

type SatelliteResponse = {
    id: number;
    name: string;
    tle1: string;
    tle2: string;
};

export function getSatellites(): Promise<[SatelliteResponse]> {
    return fetch(SATELLITES_GET).then((response) => response.json());
}