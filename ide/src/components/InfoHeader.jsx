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
      Relevant Software Dependencies:
        {blockRenderEnginerVersions ? <p>[dtr_core]: {blockRenderEnginerVersions.dtr_core}</p> : <p>Loading...</p>}
        {blockRenderEnginerVersions ? <p>[block render engine]: {blockRenderEnginerVersions.block_render_engine}</p> : <p>Loading...</p>}
        {tempRustToDtrServerVersions ? <p>[rust_to_dtr_version]: {tempRustToDtrServerVersions.rust_to_dtr_version}</p> : <p>Loading...</p>}
        {tempRustToDtrServerVersions ? <p>[block temp_rust_to_dtr_version engine]: {tempRustToDtrServerVersions.temp_rust_to_dtr_version}</p> : <p>Loading...</p>}
    </div>
  );
}

export default InfoHeader;