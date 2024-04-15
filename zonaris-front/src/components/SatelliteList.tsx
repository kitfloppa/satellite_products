import { List } from "antd";
import { useState, useEffect } from 'react';
import { getSatellites } from '../api/satellite.api'
import { getInstrumentData } from '../api/instrument-data.api'

import type { SatelliteResponse } from '../api/satellite.api'


export function SatelliteList() {
    const [satellites, setSatellites] = useState<[SatelliteResponse] | undefined>(undefined);
    const [satelliteId, setSatelliteId] = useState<number | undefined>(undefined);
    
    useEffect(() => {
        getSatellites().then((data) => setSatellites(data));
    }, []);

    useEffect(() => {
        if (satelliteId === undefined) {
            return
        }

        getInstrumentData(satelliteId).then((response) => console.log(response))
    }, [satelliteId]);

    return (
        <List
            size="small"
            header={<div>Спутники</div>}
            bordered
            dataSource={satellites}
            renderItem={(item) => <List.Item onClick={() => setSatelliteId(item.id)}>{item.name}</List.Item>}
        />
    );
}