import { HOST } from './constants.api';

const DATA_GET = `${HOST}/data/get`;
const DATA_GET_ASSET = `${HOST}/data/get_asset`;

type InstrumentDataResponse = {
    id: number;
};

export function getInstrumentData(id: number): Promise<[InstrumentDataResponse]> {
    const searchParams: Record<string, any> = new URLSearchParams();
    searchParams.append("id", id);
    
    return fetch(DATA_GET + '?' + searchParams).then((response) => response.json());
}

export function getAssetData(id: number): Promise<Blob> {
    const searchParams: Record<string, any> = new URLSearchParams();
    searchParams.append("id", id);

    return fetch(DATA_GET_ASSET + '?' + searchParams).then((response) => response.blob());
}