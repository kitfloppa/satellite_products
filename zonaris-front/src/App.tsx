import React, { useState, useEffect } from 'react';
import { getInstrumentData, getAssetData } from './api/instrument-data.api'
import { getSatellites } from './api/satellite.api'
import { SatelliteList } from './components/SatelliteList'


function App() {
  return (
    <div><SatelliteList></SatelliteList></div>
  );
}

export default App;
