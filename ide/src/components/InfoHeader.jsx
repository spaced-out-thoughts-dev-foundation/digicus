import React, { useEffect, useState } from 'react';

import '.././styles/InfoHeader.css';

function InfoHeader() {
  const [blockRenderEnginerVersions, setBlockRenderEngineVersions] = useState(null);
  const [tempRustToDtrServerVersions, setTempRustToDtrServerVersions] = useState(null);

  useEffect(() => {
    fetch('https://block-render-engine.vercel.app/api/ide_version')
      .then(response => {
        return response.json()
      })
      .then(json => setBlockRenderEngineVersions(json))
      .catch(error => console.error(error));
  }, []);


  useEffect(() => {
    fetch('https://rust-to-dtr.vercel.app/api/versions')
      .then(response => {
        console.log(response)
        return response.json()
      })
      .then(json => setTempRustToDtrServerVersions(json))
      .catch(error => console.error(error));
  }, []);


  return (
    <div className='info-header'>
      <div className='info-header-dependency-container'>
        <div>
          <h3>Block Render Engine ({blockRenderEnginerVersions ? blockRenderEnginerVersions.block_render_engine : "Loading..."})</h3>
          {blockRenderEnginerVersions ? <p>[dtr_core]: {blockRenderEnginerVersions.dtr_core}</p> : <p>Loading...</p>}
          {blockRenderEnginerVersions ? <p>[dtr_to_rust]: {blockRenderEnginerVersions.dtr_to_rust}</p> : <p>Loading...</p>}
        </div>
        <div>
          <h3>Rust to DTR Server ({tempRustToDtrServerVersions ? tempRustToDtrServerVersions.temp_rust_to_dtr_version : "Loading..."}) </h3>
          {tempRustToDtrServerVersions ? <p>[rust_to_dtr]: {tempRustToDtrServerVersions.rust_to_dtr_version}</p> : <p>Loading...</p>}
        </div>
      </div>
    </div>
  );
}

export default InfoHeader;